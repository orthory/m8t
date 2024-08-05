use std::collections::{BTreeMap};
use std::path::{PathBuf};
use anyhow::anyhow;
use serde::Deserialize;
use document::Document;
use crate::Backend;

#[derive(Debug, Deserialize)]
pub enum Hierarchy {
    File(Document),
    Directory(BTreeMap<String, Hierarchy>)
}

impl Hierarchy {
    pub fn new(backend: Backend) -> anyhow::Result<Self> {
        backend.construct_hierarchy()
    }
    
    pub fn resolve(&self, path_segments: &PathBuf) -> anyhow::Result<&Self> {
        let mut next = self;
        for segment in path_segments {
            match next {
                Hierarchy::File(_) => return Err(anyhow!("encountered non-descendant element")),
                Hierarchy::Directory(tree) => {
                    let descendant = tree.get(&String::from(segment.to_string_lossy()))
                        .expect("file not found");
                    
                    next = descendant
                }
            }
        }
        
        Ok(next)
    }
}

// #[cfg(test)]
// mod tests {
//     use std::collections::BTreeMap;
//     use anyhow::anyhow;
//     use document::Document;
//     use super::Hierarchy::{self, Directory, File};
//
//     fn setup() -> Hierarchy {
//         // setup directory hierarchy
//         Directory(BTreeMap::from([
//             ("a".to_string(), Directory(BTreeMap::from([
//                 ("ab".to_string(), File((Document::default()))),
//                 ("ac".to_string(), Directory(BTreeMap::from([
//                     ("aca".to_string(), File(Document::default())),
//                     ("acb".to_string(), File(Document::default())),
//                     ("acc".to_string(), File(Document::default())),
//                     ("acd".to_string(), File(Document::default())),
//                     ("ace".to_string(), File(Document::default())),
//                     ("acf".to_string(), File(Document::default())),
//                     ("acg".to_string(), File(Document::default())),
//                 ])))
//             ]))),
//             ("b".to_string(), File(Document::default())),
//             ("c".to_string(), Directory(BTreeMap::from([
//                 ("ca".to_string(), Directory(BTreeMap::new())) // not yet
//             ])))
//         ]))
//     }
//
//     fn run<'run>(root: &'run Hierarchy, segments: Vec<&str>) -> anyhow::Result<DSR<'run>> {
//         let mut h = root;
//
//         for segment in segments {
//             let search = h.descendants(&segment.to_string()).unwrap();
//             let DSR::Indexed(next) = search else {
//                 return Err(anyhow!("must have been indexed"));
//             };
//
//             h = next;
//         }
//
//         Ok(DSR::Indexed(h))
//     }
//
//     #[test]
//     fn descendants_found() {
//         let root = setup();
//         let res = run(&root, vec!["a", "ac"])
//             .expect("should not panic");
//
//         let DSR::Indexed(res) = res else {
//             panic!("must have been indexed but not really");
//         };
//
//         let Directory(d) = res else {
//             panic!("must have found directory but not really");
//         };
//
//         let Some(files) = d else {
//             panic!("must have been already indexed but not really");
//         };
//
//         assert_eq!(files.len(), 7);
//         assert_eq!(
//             files.keys().collect::<Vec<&String>>(),
//             vec!["aca", "acb", "acc", "acd", "ace", "acf", "acg"]
//         );
//     }
//
//     #[test]
//     fn descendants_not_indexed() {
//         let root = setup();
//         let res = run(&root, vec!["c", "ca"])
//             .expect("should not panic");
//
//         let DSR::Indexed(res) = res else {
//             panic!("must have been indexed but not really");
//         };
//
//         let Directory(index) = res else {
//             panic!("must have been directory but not really");
//         };
//
//         assert!(index.is_none());
//     }
//
//     #[test]
//     fn descendants_error_if_file() {
//         let root = setup();
//         let res = run(&root, vec!["a", "ab"])
//             .expect("should not panic");
//
//         let DSR::Indexed(res) = res else {
//             panic!("must have been indexed but not really");
//         };
//
//         let File = res else {
//             panic!("must have been directory but not really");
//         };
//     }
// }