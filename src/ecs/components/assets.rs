use crate::assets::caches::MeshCache;
use crate::assets::loaders::GltfLoadError;
use crate::graphics::core::GpuContext;
use crate::graphics::resources::mesh::GpuMesh;

use std::path::Path;
use std::sync::Arc;

use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct AssetManager {
    meshes: MeshCache,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            meshes: MeshCache::new(),
        }
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
}

impl Default for AssetManager {
    fn default() -> Self {
        Self::new()
    }
}
