use anyhow::anyhow;
use serde::Deserialize;
#[cfg(test)]
use vfs::FileSystem;
use crate::document_resolver::backend_stdfs;

#[derive(Debug, Deserialize)]
pub enum FileType {
    File(String),
    Directory(String, Option<Vec<FileType>>)
}

pub enum Backend {
    StdFS(String),

    #[cfg(test)]
    Vfs(vfs::MemoryFS),
}

impl Backend {
    pub(crate) fn scan_all(&self, path: &String) -> anyhow::Result<Vec<FileType>> {
        match self {
            Backend::StdFS(base_path) => backend_stdfs::scan(base_path),
            
            // don't handle scan_all
            _ => Err(anyhow!("scan not supported on this")),
        }
    }

    pub(crate) fn load(&self, path: &String) -> anyhow::Result<document::Document> {
        match self {
            Backend::StdFS(base_path) => backend_stdfs::read_file(base_path, path),
            
            #[cfg(test)]
            Backend::Vfs(vfs) => {
                let mut file = vfs
                    .open_file(path)
                    .expect("(Backend::VFS) failed to load file");
                let mut buf = String::new();
                let _ = file.read_to_string(&mut buf);
                let document = document::Document::new_from_buffer(buf.as_str())
                    .expect("(Backend::VFS) unable to parse document");

                Ok(document)
            }
        }
    }
}
