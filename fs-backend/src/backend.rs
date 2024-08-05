use std::path::PathBuf;
use anyhow::anyhow;
use crate::hierarchy::Hierarchy;
use crate::backend_stdfs;

pub enum Backend {
    StdFS(String),
    Vfs(vfs::MemoryFS),
}

impl Backend {
    pub fn construct_hierarchy(&self) -> anyhow::Result<Hierarchy> {
        match self {
            Backend::StdFS(base_path) => backend_stdfs::scan_dir(&PathBuf::from(base_path)),

            // don't handle scan_all
            _ => Err(anyhow!("scan not supported on this")),
        }
    }
}
