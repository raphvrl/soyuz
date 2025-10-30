use crate::assets::loaders::gltf::{GltfLoadError, load_gltf_mesh};
use crate::graphics::core::GpuContext;
use crate::graphics::resources::mesh::GpuMesh;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct MeshHandle {
    path: PathBuf,
}

impl MeshHandle {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
}

pub struct MeshCache {
    cache: HashMap<PathBuf, Arc<GpuMesh>>,
}

impl MeshCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn load(
        &mut self,
        gpu: &GpuContext,
        path: impl AsRef<Path>,
    ) -> Result<Arc<GpuMesh>, GltfLoadError> {
        let path = path.as_ref();

        if let Some(mesh) = self.cache.get(path) {
            return Ok(Arc::clone(mesh));
        }

        let label = path.file_name().and_then(|n| n.to_str());
        let gpu_mesh = load_gltf_mesh(gpu, path, label)?;
        let arc_mesh = Arc::new(gpu_mesh);

        self.cache.insert(path.to_path_buf(), Arc::clone(&arc_mesh));
        Ok(arc_mesh)
    }

    pub fn get(&self, path: impl AsRef<Path>) -> Option<Arc<GpuMesh>> {
        self.cache.get(path.as_ref()).map(Arc::clone)
    }

    pub fn unload(&mut self, path: impl AsRef<Path>) -> bool {
        self.cache.remove(path.as_ref()).is_some()
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }

    pub fn len(&self) -> usize {
        self.cache.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}

impl Default for MeshCache {
    fn default() -> Self {
        Self::new()
    }
}
