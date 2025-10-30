use crate::assets::vertices::GltfVertex;
use crate::graphics::core::GpuContext;
use crate::graphics::resources::mesh::GpuMesh;
use glam::{Vec2, Vec3};
use std::path::Path;

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
}

pub struct GltfMeshData {
    pub vertices: Vec<GltfVertex>,
    pub indices: Vec<u16>,
}

impl GltfMeshData {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, GltfLoadError> {
        let (gltf, buffers, _) = gltf::import(path)?;

        let mesh = gltf.meshes().next().ok_or(GltfLoadError::NoPrimitives)?;

        let primitive = mesh
            .primitives()
            .next()
            .ok_or(GltfLoadError::NoPrimitives)?;

        let reader = primitive.reader(|buffer| buffers.get(buffer.index()).map(|data| &data[..]));

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

        let vertices = positions
            .into_iter()
            .zip(normals)
            .zip(uvs)
            .map(|((position, normal), uv)| GltfVertex::new(position, normal, uv))
            .collect();

        let indices = reader
            .read_indices()
            .ok_or(GltfLoadError::MissingIndices)?
            .into_u32()
            .map(|i| i as u16)
            .collect();

        Ok(Self { vertices, indices })
    }

    pub fn to_gpu_mesh(&self, gpu: &GpuContext, label: Option<&str>) -> GpuMesh {
        GpuMesh::new(gpu, &self.vertices, &self.indices, label)
    }
}

pub fn load_gltf_mesh<P: AsRef<Path>>(
    gpu: &GpuContext,
    path: P,
    label: Option<&str>,
) -> Result<GpuMesh, GltfLoadError> {
    let mesh_data = GltfMeshData::from_file(path)?;
    Ok(mesh_data.to_gpu_mesh(gpu, label))
}
