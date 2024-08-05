use crate::Hierarchy;
use std::collections::BTreeMap;
use std::path::PathBuf;
use log::info;

// scan scans 1 depth of the sample
// returns a list of files
pub(crate) fn scan_dir(base_path: &PathBuf) -> anyhow::Result<Hierarchy> {
    info!("scan_dir {}", base_path.to_str().unwrap());
    let dir_entries = std::fs::read_dir(base_path)
        .expect("unable to read dir");

    let hierarchy = dir_entries
        .map(|entry| {
            let entry = entry.expect("invalid dir_entry");
            let path = entry.path();

            let path_type = get_path_type(&path)
                .expect("failed to guess file type");

            let qualified_path = path.strip_prefix(base_path)
                .expect("path strip failed")
                .to_string_lossy();

            match path_type {
                PathType::File => read_file(&path)
                    .map(Hierarchy::File)
                    .map(|h| (String::from(qualified_path), h)),

                PathType::Directory => scan_dir(&path)
                    .map(|h| (String::from(qualified_path), h)),

                // TODO: fix me
                PathType::Symlink
                | PathType::Other => panic!("don't know how to handle yet")
            }
        })
        .map(|f| f.unwrap())
        .collect::<BTreeMap<String, Hierarchy>>();

    Ok(Hierarchy::Directory(hierarchy))
}

pub(crate) fn read_file(path: &PathBuf) -> anyhow::Result<document::Document> {
    let file =
        std::fs::read_to_string(path).expect("(Backend::StdFS) failed to load file");
    let document = document::Document::new_from_buffer(file.as_str())
        .expect("(Backend::StdFS) unable to parse document");

    Ok(document)
}

enum PathType {
    File,
    Directory,
    Symlink,
    Other,
}

fn get_path_type(path: &PathBuf) -> anyhow::Result<PathType> {
    let metadata = std::fs::metadata(path)?;
    let file_type = metadata.file_type();

    Ok(if file_type.is_file() {
        PathType::File
    } else if file_type.is_dir() {
        PathType::Directory
    } else if file_type.is_symlink() {
        PathType::Symlink
    } else {
        PathType::Other
    })
}