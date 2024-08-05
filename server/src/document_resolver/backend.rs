use std::path::PathBuf;
#[cfg(test)]
use vfs::FileSystem;

pub enum Backend {
    StdFS(String),

    #[cfg(test)]
    Vfs(vfs::MemoryFS),
}

impl Backend {
    pub(crate) fn load(&self, path: &String) -> anyhow::Result<document::Document> {
        match self {
            Backend::StdFS(base_path) => {
                let mut path_buf = PathBuf::new();
                path_buf.push(base_path);
                path_buf.push(path);
                
                dbg!(&path_buf);
                
                let file =
                    std::fs::read_to_string(path_buf).expect("(Backend::StdFS) failed to load file");
                let document = document::Document::new_from_buffer(file.as_str())
                    .expect("(Backend::StdFS) unable to parse document");

                Ok(document)
            }

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
