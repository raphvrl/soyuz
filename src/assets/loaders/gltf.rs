use crate::assets::vertices::GltfVertex;
use crate::graphics::core::GpuContext;
use crate::graphics::resources::mesh::GpuMesh;
use crate::graphics::resources::texture::GpuTexture;
use glam::{Vec2, Vec3};
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, thiserror::Error)]
pub enum GltfLoadError {
    #[error("Failed to load GLTF file: {0}")]
    IoError(#[from] std::io::Error),
    #[error("GLTF error: {0}")]
    GltfError(#[from] gltf::Error),
    #[error("Mesh has no primitives")]
    NoPrimitives,
    #[error("Missing position attribute")]
    MissingPosition,
    #[error("Missing indices")]
    MissingIndices,
    #[error("Unsupported index format")]
    UnsupportedIndexFormat,
    #[error("Image error: {0}")]
    ImageError(#[from] image::ImageError),
}

pub struct GltfMeshData {
    pub vertices: Vec<GltfVertex>,
    pub indices: Vec<u16>,
}

#[derive(Clone)]
pub struct GltfMaterial {
    pub name: Option<String>,
    pub base_color_texture_index: Option<usize>,
    pub base_color_factor: [f32; 4],
}

pub struct GltfAsset {
    pub meshes: Vec<GltfMeshData>,
    pub textures: Vec<image::DynamicImage>,
    pub materials: Vec<GltfMaterial>,
}

impl GltfAsset {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, GltfLoadError> {
        let (gltf, buffers, images) = gltf::import(path)?;

        let mut combined_vertices = Vec::new();
        let mut combined_indices = Vec::new();

        for mesh in gltf.meshes() {
            for primitive in mesh.primitives() {
                let reader =
                    primitive.reader(|buffer| buffers.get(buffer.index()).map(|data| &data[..]));

                let positions = reader
                    .read_positions()
                    .ok_or(GltfLoadError::MissingPosition)?
                    .map(Vec3::from)
                    .collect::<Vec<_>>();

                let normals = reader
                    .read_normals()
                    .map(|iter| iter.map(Vec3::from).collect::<Vec<_>>())
                    .unwrap_or_else(|| vec![Vec3::Y; positions.len()]);

                let uvs = reader
                    .read_tex_coords(0)
                    .map(|iter| iter.into_f32().map(Vec2::from).collect::<Vec<_>>())
                    .unwrap_or_else(|| vec![Vec2::ZERO; positions.len()]);

                let vertices: Vec<GltfVertex> = positions
                    .into_iter()
                    .zip(normals)
                    .zip(uvs)
                    .map(|((position, normal), uv)| GltfVertex::new(position, normal, uv))
                    .collect();

                let vertex_offset = combined_vertices.len() as u32;

                if let Some(indices_reader) = reader.read_indices() {
                    let indices: Vec<u16> = indices_reader
                        .into_u32()
                        .map(|i| (i + vertex_offset) as u16)
                        .collect();

                    combined_indices.extend(indices);
                } else {
                    let indices: Vec<u16> = (0..vertices.len() as u32)
                        .map(|i| (i + vertex_offset) as u16)
                        .collect();
                    combined_indices.extend(indices);
                }

                combined_vertices.extend(vertices);
            }
        }

        if combined_vertices.is_empty() {
            return Err(GltfLoadError::NoPrimitives);
        }

        let meshes = vec![GltfMeshData {
            vertices: combined_vertices,
            indices: combined_indices,
        }];

        let textures: Vec<image::DynamicImage> = images
            .into_iter()
            .map(|data| match data.format {
                gltf::image::Format::R8G8B8 => {
                    let rgb_image = image::RgbImage::from_raw(data.width, data.height, data.pixels)
                        .expect("Failed to create RGB image from GLTF data");
                    image::DynamicImage::ImageRgb8(rgb_image)
                }
                gltf::image::Format::R8G8B8A8 => {
                    let rgba_image =
                        image::RgbaImage::from_raw(data.width, data.height, data.pixels)
                            .expect("Failed to create RGBA image from GLTF data");
                    image::DynamicImage::ImageRgba8(rgba_image)
                }
                gltf::image::Format::R8 => {
                    let luma_image =
                        image::GrayImage::from_raw(data.width, data.height, data.pixels)
                            .expect("Failed to create grayscale image from GLTF data");
                    image::DynamicImage::ImageLuma8(luma_image)
                }
                gltf::image::Format::R8G8 => {
                    let luma_alpha_image =
                        image::GrayAlphaImage::from_raw(data.width, data.height, data.pixels)
                            .expect("Failed to create grayscale+alpha image from GLTF data");
                    image::DynamicImage::ImageLumaA8(luma_alpha_image)
                }
                _ => {
                    panic!("Unsupported image format: {:?}", data.format);
                }
            })
            .collect();

        let materials: Vec<GltfMaterial> = gltf
            .materials()
            .map(|material| {
                let pbr = material.pbr_metallic_roughness();
                let base_color_texture_index = pbr
                    .base_color_texture()
                    .map(|info| info.texture().source().index());

                GltfMaterial {
                    name: material.name().map(String::from),
                    base_color_texture_index,
                    base_color_factor: pbr.base_color_factor(),
                }
            })
            .collect();

        Ok(Self {
            meshes,
            textures,
            materials,
        })
    }
}

pub fn load_gltf_texture(
    gpu: &GpuContext,
    gltf_path: &Path,
    texture_index: usize,
) -> Result<Arc<GpuTexture>, GltfLoadError> {
    let asset = GltfAsset::from_file(gltf_path)?;

    if texture_index >= asset.textures.len() {
        return Err(GltfLoadError::ImageError(image::ImageError::Parameter(
            image::error::ParameterError::from_kind(image::error::ParameterErrorKind::NoMoreData),
        )));
    }

    let img = &asset.textures[texture_index];
    let label = Some(format!("gltf_texture_{}", texture_index));

    let texture = GpuTexture::from_image(gpu, img.clone(), label.as_deref());
    Ok(Arc::new(texture))
}

pub fn load_gltf_mesh<P: AsRef<Path>>(
    gpu: &GpuContext,
    path: P,
    label: Option<&str>,
) -> Result<GpuMesh, GltfLoadError> {
    let asset = GltfAsset::from_file(path)?;
    if let Some(mesh_data) = asset.meshes.first() {
        Ok(GpuMesh::new(
            gpu,
            &mesh_data.vertices,
            &mesh_data.indices,
            label,
        ))
    } else {
        Err(GltfLoadError::NoPrimitives)
    }
}
