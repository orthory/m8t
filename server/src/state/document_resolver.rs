use crate::state::Document;

pub trait DocumentResolver {
    fn read_file(&self, path: String) -> anyhow::Result<impl Document>;
    fn create_file(&self, path: String, buf: String) -> anyhow::Result<usize>;
}

#[cfg(test)]
mod test_resolver {
    use vfs::FileSystem;
    use std::io::Cursor;
    use crate::state::Document;
    use crate::state::document_resolver::DocumentResolver;


    impl DocumentResolver for vfs::MemoryFS {
        fn read_file(&self, path: String) -> anyhow::Result<impl Document> {
            let f = self.open_file(path.as_str())?;
            let c = Cursor::new(f);
            
            let res = 
            parser::parse(
                
            )
            




            todo!()
        }

        fn create_file(&self, path: String, buf: String) -> anyhow::Result<usize> {
            todo!()
        }
    }


}
