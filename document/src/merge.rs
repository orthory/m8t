use anyhow::{anyhow, Result};
use sections::{comment, frontmatter, SectionType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocumentFragment {
    Frontmatter(Option<frontmatter::FrontMatter>),
    Comments(Option<Vec<comment::Comment>>),
    Body(Option<String>),
}

pub fn merge(
    fragments: &mut HashMap<String, DocumentFragment>,
    next: SectionType,
) -> Result<()> {
    let section_key = extract_key(&next).to_string();

    let fragment = fragments
        .entry(section_key)
        .or_insert_with(|| create_empty_fragment(&next));

    match (fragment, next) {
        (DocumentFragment::Body(body), SectionType::Body(next)) => {
            body.get_or_insert_with(String::new).push_str(format!("{}\n", &next).as_str());
        }
        (DocumentFragment::Frontmatter(f), SectionType::FrontMatter(next)) => {
            *f = Some(next);
        }
        (DocumentFragment::Comments(c), SectionType::Comment(next)) => {
            c.get_or_insert_with(Vec::new).push(next);
        }
        _ => return Err(anyhow!("Mismatched fragment type and section type")),
    }

    Ok(())
}

fn create_empty_fragment(section_type: &SectionType) -> DocumentFragment {
    match section_type {
        SectionType::Body(_) => DocumentFragment::Body(None),
        SectionType::FrontMatter(_) => DocumentFragment::Frontmatter(None),
        SectionType::Comment(_) => DocumentFragment::Comments(None),
        SectionType::Empty => panic!("Cannot create fragment for Empty section type"),
        SectionType::Task | SectionType::Event => todo!("Implement Task and Event fragment types"),
    }
}

fn extract_key(section_type: &SectionType) -> &'static str {
    match section_type {
        SectionType::Empty => "empty",
        SectionType::Body(_) => "body",
        SectionType::FrontMatter(_) => "frontmatter",
        SectionType::Comment(_) => "comments",
        SectionType::Task => "tasks",
        SectionType::Event => "events",
    }
}