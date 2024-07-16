use serde::{Deserialize, Serialize};
use crate::{ReadLineResult, Section};

#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    body: String,
    
    #[serde(skip_serializing)]
    line_pos: u64,
}

impl Section for Body {
    fn new(line_pos: u64) -> Self {
        Self {
            line_pos,
            body: String::new()
        }
    }

    fn match_line(_: &str) -> bool {
        true
    }


    fn read_line(&mut self, line: &str) -> ReadLineResult {
        self.body.push_str(line);
        self.body.push('\n');
        ReadLineResult::Keep
    }
}