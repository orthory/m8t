use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::{ReadLineResult, Section};

const PATTERN: &str = "---";

#[derive(Debug, Serialize, Deserialize)]
pub struct FrontMatter {
    frontmatter: HashMap<String, serde_yaml::Value>,

    #[serde(skip_serializing)]
    line_pos: u64,

    #[serde(skip_serializing)]
    temp_buffer: Vec<String>,
}

impl FrontMatter {
    fn finalize(&mut self) -> &mut Self {
        let as_yaml_str = self.temp_buffer.join("\n");
        self.frontmatter = serde_yaml::from_str(as_yaml_str.as_str()).expect("frontmatter: yaml decode failed");
        self
    }
}

impl Section for FrontMatter {
    fn new(line_pos: u64) -> Self {
        Self {
            line_pos,
            temp_buffer: Vec::new(),
            frontmatter: HashMap::new()
        }
    }

    fn match_line(line: &str) -> bool {
        line.eq(PATTERN)
    }

    fn read_line(&mut self, line: &str) -> ReadLineResult {
        match line {
            "---" => match self.temp_buffer.is_empty() {
                false => {
                    self.finalize();
                    ReadLineResult::Done
                },
                true  => ReadLineResult::Keep
            },
            _ => {
                self.temp_buffer.push(String::from(line));
                ReadLineResult::Keep
            }
        }
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
        let mut sec = FrontMatter::new(0);

        for l in fixture.lines() {
            sec.read_line(l);
        }

        dbg!(sec);
    }
}
