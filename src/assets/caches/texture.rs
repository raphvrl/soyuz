use crate::graphics::core::GpuContext;
use crate::graphics::resources::texture::GpuTexture;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct TextureCache {
    cache: HashMap<PathBuf, Arc<GpuTexture>>,
}

impl Default for TextureCache {
    fn default() -> Self {
        Self::new()
    }
}

impl TextureCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn load(
        &mut self,
        gpu: &GpuContext,
        path: impl AsRef<Path>,
    ) -> Result<Arc<GpuTexture>, image::ImageError> {
        let path = path.as_ref();

        if let Some(texture) = self.cache.get(path) {
            return Ok(Arc::clone(texture));
        }

        let img = image::open(path)?;
        let label = path.file_name().and_then(|n| n.to_str());

        let gpu_texture = GpuTexture::from_image(gpu, img, label);

        let arc_texture = Arc::new(gpu_texture);
        self.cache
            .insert(path.to_path_buf(), Arc::clone(&arc_texture));
        Ok(arc_texture)
    }

    pub fn get(&self, path: impl AsRef<Path>) -> Option<Arc<GpuTexture>> {
        self.cache.get(path.as_ref()).map(Arc::clone)
    }

    pub fn unload(&mut self, path: impl AsRef<Path>) -> bool {
        self.cache.remove(path.as_ref()).is_some()
    }
}
