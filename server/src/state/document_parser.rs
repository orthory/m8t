use std::io::Cursor;
use crate::state::Document;

pub trait DocumentParser {
    fn parse(
        &self,
        cursor: Cursor<&str>
    ) -> dyn Document;
}
