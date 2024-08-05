pub mod backend;
pub use backend::Backend;

use std::collections::HashMap;

pub struct DocumentResolver {
    _backend: Backend,
    _cache: HashMap<String, document::Document>,
}

impl DocumentResolver {
    pub fn new(backend: Backend) -> Self {
        Self {
            _backend: backend,
            _cache: HashMap::new(),
        }
    }

    pub fn resolve(&mut self, path: String) -> anyhow::Result<&document::Document> {
        if !self._cache.contains_key(&path) {
            let document_from_disk = self._backend.load(&path)?;
            self._cache.insert(path.clone(), document_from_disk.clone());
        }

        Ok(self._cache.get(&path).unwrap())
    }
}
