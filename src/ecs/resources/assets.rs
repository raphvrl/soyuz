use crate::assets::caches::{MeshCache, TextureCache};
use crate::assets::loaders::{GltfAsset, GltfLoadError};
use crate::ecs::components::Material;
use crate::graphics::core::{BindGroup, BindResource, GpuContext};
use crate::graphics::resources::mesh::GpuMesh;
use crate::graphics::resources::texture::GpuTexture;

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use bevy_ecs::prelude::*;

const MAX_TEXTURES: usize = 512;

#[derive(Resource)]
pub struct AssetManager {
    meshes: MeshCache,
    textures: TextureCache,

    texture_slots: Vec<Arc<GpuTexture>>,
    texture_index_map: HashMap<String, u32>,

    texture_bind_group: BindGroup,
    sampler: wgpu::Sampler,
    bind_group_dirty: bool,
}

impl AssetManager {
    pub fn new(gpu: &GpuContext) -> Self {
        let device = gpu.device();

        let default_texture = Arc::new(GpuTexture::from_rgba(
            gpu,
            1,
            1,
            &[255, 255, 255, 255],
            Some("default_white_texture"),
        ));

        let texture_slots = vec![default_texture; MAX_TEXTURES];

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("asset_manager_sampler"),
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let texture_bind_group = Self::create_texture_bind_group(device, &texture_slots, &sampler);

        Self {
            meshes: MeshCache::new(),
            textures: TextureCache::new(),
            texture_slots,
            texture_index_map: HashMap::new(),
            texture_bind_group,
            sampler,
            bind_group_dirty: false,
        }
    }

    fn create_texture_bind_group(
        device: &wgpu::Device,
        textures: &[Arc<GpuTexture>],
        sampler: &wgpu::Sampler,
    ) -> BindGroup {
        let views: Vec<&wgpu::TextureView> = textures.iter().map(|t| t.view()).collect();

        BindGroup::builder()
            .label("global_texture_array")
            .texture_array(0, wgpu::ShaderStages::FRAGMENT, MAX_TEXTURES as u32)
            .sampler(1, wgpu::ShaderStages::FRAGMENT)
            .build(
                device,
                &[
                    BindResource::TextureViewArray(0, &views),
                    BindResource::Sampler(1, sampler),
                ],
            )
            .unwrap()
    }

    pub fn load_mesh(
        &mut self,
        gpu: &GpuContext,
        path: impl AsRef<Path>,
    ) -> Result<Arc<GpuMesh>, GltfLoadError> {
        self.meshes.load(gpu, path)
    }

    pub fn get_mesh(&self, path: impl AsRef<Path>) -> Option<Arc<GpuMesh>> {
        self.meshes.get(path)
    }

    pub fn unload_mesh(&mut self, path: impl AsRef<Path>) -> bool {
        self.meshes.unload(path)
    }

    pub fn load_texture(
        &mut self,
        gpu: &GpuContext,
        path: impl AsRef<Path>,
    ) -> Result<Arc<GpuTexture>, image::ImageError> {
        self.textures.load(gpu, path)
    }

    pub fn get_texture(&self, path: impl AsRef<Path>) -> Option<Arc<GpuTexture>> {
        self.textures.get(path)
    }

    pub fn unload_texture(&mut self, path: impl AsRef<Path>) -> bool {
        self.textures.unload(path)
    }

    pub fn load_gltf_asset(
        &mut self,
        _gpu: &GpuContext,
        path: impl AsRef<Path>,
    ) -> Result<GltfAsset, GltfLoadError> {
        GltfAsset::from_file(path)
    }

    pub fn load_texture_for_rendering(
        &mut self,
        gpu: &GpuContext,
        path: impl AsRef<Path>,
    ) -> Result<u32, image::ImageError> {
        let path_str = path.as_ref().to_string_lossy().to_string();

        if let Some(&index) = self.texture_index_map.get(&path_str) {
            return Ok(index);
        }

        let texture = self.textures.load(gpu, &path)?;

        let index = self.register_texture_for_rendering(texture, path_str);

        Ok(index)
    }

    pub fn register_texture_for_rendering(
        &mut self,
        texture: Arc<GpuTexture>,
        name: String,
    ) -> u32 {
        if let Some(&index) = self.texture_index_map.get(&name) {
            return index;
        }

        let index = self.texture_index_map.len() as u32 + 1;

        if index >= MAX_TEXTURES as u32 {
            panic!("Maximum de {} textures atteintes !", MAX_TEXTURES);
        }

        self.texture_slots[index as usize] = texture;
        self.texture_index_map.insert(name, index);
        self.bind_group_dirty = true;

        index
    }

    pub fn update_texture_bindings(&mut self, gpu: &GpuContext) {
        if self.bind_group_dirty {
            self.texture_bind_group =
                Self::create_texture_bind_group(gpu.device(), &self.texture_slots, &self.sampler);
            self.bind_group_dirty = false;
        }
    }

    pub fn texture_bind_group(&self) -> &BindGroup {
        &self.texture_bind_group
    }

    pub fn texture_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        self.texture_bind_group.layout()
    }

    pub fn get_texture_by_index(&self, index: u32) -> Option<&Arc<GpuTexture>> {
        self.texture_slots.get(index as usize)
    }

    pub fn load_material_from_gltf(
        &mut self,
        gpu: &GpuContext,
        gltf_material: &crate::assets::loaders::gltf::GltfMaterial,
        gltf_textures: &[image::DynamicImage],
    ) -> Material {
        if let Some(tex_idx) = gltf_material.base_color_texture_index {
            if let Some(img) = gltf_textures.get(tex_idx) {
                let texture = Arc::new(GpuTexture::from_image(
                    gpu,
                    img.clone(),
                    Some(&format!("gltf_texture_{}", tex_idx)),
                ));

                let texture_index = self
                    .register_texture_for_rendering(texture, format!("gltf_texture_{}", tex_idx));

                Material::new(texture_index)
            } else {
                Material::with_color(gltf_material.base_color_factor)
            }
        } else {
            Material::with_color(gltf_material.base_color_factor)
        }
    }
}
