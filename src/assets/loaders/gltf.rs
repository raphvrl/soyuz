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

        let mut all_vertices = Vec::new();
        let mut all_indices = Vec::new();

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

                let vertex_offset = all_vertices.len() as u32;

                if let Some(indices_reader) = reader.read_indices() {
                    let indices: Vec<u16> = indices_reader
                        .into_u32()
                        .map(|i| (i + vertex_offset) as u16)
                        .collect();

                    all_indices.extend(indices);
                } else {
                    let indices: Vec<u16> = (0..vertices.len() as u32)
                        .map(|i| (i + vertex_offset) as u16)
                        .collect();
                    all_indices.extend(indices);
                }

                all_vertices.extend(vertices);
            }
        }

        if all_vertices.is_empty() {
            return Err(GltfLoadError::NoPrimitives);
        }

        Ok(Self {
            vertices: all_vertices,
            indices: all_indices,
        })
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
