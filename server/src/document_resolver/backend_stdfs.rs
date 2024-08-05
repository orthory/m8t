use std::path::PathBuf;
use crate::document_resolver::backend::FileType;

// scan scans 1 depth of the hierarchy
// returns a list of files
pub fn scan(base_path: &String) -> anyhow::Result<Vec<FileType>> {
    let files = std::fs::read_dir(base_path)
        .unwrap()
        .map(|entry| {
            let entry = entry.expect("invalid dirEntry");
            let path = entry.path();
            let path_str = path.to_string_lossy().into_owned();
            
            match path.is_file() {
                true => FileType::File(path_str),
                false => FileType::Directory(path_str, None)
            }
        })
        .collect();
    
    Ok(files)
}

pub fn read_file(base_path: &String, path: &String) -> anyhow::Result<document::Document> {
    let mut path_buf = PathBuf::new();
    path_buf.push(base_path);
    path_buf.push(path);

    let file =
        std::fs::read_to_string(path_buf).expect("(Backend::StdFS) failed to load file");
    let document = document::Document::new_from_buffer(file.as_str())
        .expect("(Backend::StdFS) unable to parse document");

    Ok(document)
}