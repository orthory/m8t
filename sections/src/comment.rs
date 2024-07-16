use serde::{Deserialize, Serialize};
use crate::{ReadLineResult, Section};

const PATTERN: &str = "---comment";

// comment extension
//
// this extension is a simple extension that facilitates in-place comment
//
// ```markdown
// ---comment <author>,<parent_id>
// <markdown content>
// ---comment
#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    author: Option<String>,
    parent_id: Option<u64>,
    body: String,

    #[serde(skip_serializing)]
    line_pos: u64,
}

impl Section for Comment {
    fn new(line_pos: u64) -> Self {
        Self {
            line_pos,
            author: None,
            parent_id: None,
            body: String::new(),
        }
    }

    fn match_line(line: &str) -> bool {
        line.starts_with(PATTERN)
    }

    fn read_line(&mut self, line: &str) -> ReadLineResult {
        match line {
            // comment end
            line if line.eq(PATTERN) => ReadLineResult::Done,
            // comment start
            line if line.starts_with(PATTERN) => {
                parse_opener(line, self);
                ReadLineResult::Keep
            },
            // body
            line => {
                self.body.push_str(line);
                self.body.push('\n');
                ReadLineResult::Keep
            }
        }
    }
}

static PARSE_RULES: [for<'a> fn(&str, &'a mut Comment) -> &'a mut Comment; 2] = [
    |author, comment| {
        let as_str = String::from(author);
        comment.author = Some(as_str);
        comment
    },

    |parent_id, comment| {
        let as_u64 = parent_id.parse::<u64>().expect("second argument in comment must be numbers");
        comment.parent_id = Some(as_u64);
        comment
    },
];

fn parse_opener<'parse>(line: &str, target: &'parse mut Comment) -> &'parse mut Comment {
    line[PATTERN.len()+1..]
        .split(',')
        .enumerate()
        .fold(
            target,
            |next, (pos, fragment)| -> &mut Comment {
                dbg!(&pos);
                PARSE_RULES[pos](fragment, next)
            }
        )
}

#[cfg(test)]
mod tests {
    use crate::Section;
    use super::Comment;

    const fixture: &str = r"---comment @kjessec,18235235
Hello World
This is a markdown comment
asdfasdf
---comment
";

    #[test]
    fn comment_works() {
        let mut sec = Comment::new(0);

        for l in fixture.lines() {
            sec.read_line(l);
        }

        assert_eq!(sec.line_pos, 0);
        assert_eq!(sec.author, Some(String::from("@kjessec")));
        assert_eq!(sec.parent_id, Some(18235235_u64));
    }
}