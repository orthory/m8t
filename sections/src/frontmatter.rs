use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const PATTERN: &str = "---";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FrontMatter(HashMap<String, serde_yaml::Value>);

impl FrontMatter {
    pub fn new(block: Vec<String>) -> anyhow::Result<Self> {
        let block_filtered = block
            .iter()
            .filter_map(|l| (!l.eq(PATTERN)).then_some(l.clone()))
            .collect::<Vec<String>>();

        let block_filtered = block_filtered.join("\n");
        let block_filtered = block_filtered.as_str();

        let frontmatter =
            serde_yaml::from_str(block_filtered).expect("frontmatter block contains invalid yaml");

        Ok(Self(frontmatter))
    }
}

#[cfg(test)]
mod tests {
    use crate::frontmatter::*;

    const fixture: &str = r"---
rules:
- id: rule-1
  languages:
    - c
    - cpp
- id: rule-2
  languages:
    - rust
---";

    #[test]
    fn frontmatter_works() {
        let split = fixture
            .split("\n")
            .map(|l| l.to_string())
            .collect::<Vec<String>>();
        let res = FrontMatter::new(split).unwrap();

        assert!(res.0.get("rules").is_some());
    }
}
