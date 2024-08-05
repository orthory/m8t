use crate::merge::{merge, DocumentFragment};
use sections::{CommandBlockType, SectionType};
use std::collections::HashMap;
use std::iter::{Map, once_with};
use std::ops::ControlFlow;
use std::ops::ControlFlow::{Break, Continue};
use std::str::Lines;

#[derive(Debug, Clone)]
pub struct Document {
    fragments: Fragments,
}

type Fragments = HashMap<&'static str, DocumentFragment>;

impl Document {
    pub fn new_from_buffer(buffer: &str) -> anyhow::Result<Self> {
        let mut document = Self {
            fragments: HashMap::new(),
        };

        // handle each section
        for section in Parser::new_from_buffer(buffer) {
            merge(&mut document.fragments, section).unwrap();
        }

        Ok(document)
    }

    pub fn fragments(&self) -> &Fragments {
        &self.fragments
    }
}

struct Parser<'file> {
    _iterator: Map<Lines<'file>, fn(&str) -> String>,
}

impl<'file> Parser<'file> {
    fn new_from_buffer(buffer: &'file str) -> Self {
        Self {
            _iterator: buffer.lines().map(|e| e.to_string()),
        }
    }
}

// TODO: make me more efficient
impl<'file> Iterator for Parser<'file> {
    type Item = SectionType;

    fn next(&mut self) -> Option<Self::Item> {
        let document_buffer = self._iterator.by_ref();
        let next_line = document_buffer.next()?;

        // see if special blocks section has any match
        let there_is_match = sections::SECTION_ENTRIES
            .iter()
            .clone()
            .find(|(block_type, _)| match block_type {
                CommandBlockType::Line(p) => p.is_matched(&next_line),
                CommandBlockType::Block(s, _) => s.is_matched(&next_line),
            });

        // if there is no match, process as body and continue
        if there_is_match.is_none() {
            return Some(SectionType::Body(next_line));
        }

        // if there IS a match, process block
        let (block_type, processor) = there_is_match.unwrap();

        let next = match block_type {
            // for line, process line as is
            CommandBlockType::Line(_) => processor(vec![next_line.clone()]),

            // for block, read until end match
            // and process as a complete block
            CommandBlockType::Block(_, e) => {
                let mut block = vec![next_line.clone()];
                
                for line in document_buffer {
                    block.push(line.clone());
                    if e.is_matched(&line) {
                        break
                    }
                }

                processor(block) // wtf??
            }
        };

        Some(next.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::Document;

    #[test]
    fn works() {
        let mut doc = Document::new_from_buffer(testutil::fixtures::TestDocumentBuffer).unwrap();

        dbg!(doc);
    }
}
