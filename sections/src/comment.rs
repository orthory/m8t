use log::warn;
use serde::{Deserialize, Serialize};

const PATTERN: &str = "/comment";
const FIELD_DELIMITER: &str = ",";

// comment extension
//
// this extension is a simple extension that facilitates in-place comment
//
// ```markdown
// /comment <author>,<parent_id>
// <markdown content>
// /comment
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Comment {
    author: Option<String>,
    parent_id: Option<String>,
    body: String,
}

impl Comment {
    pub fn new(block: Vec<String>) -> anyhow::Result<Self> {
        let mut comment: Self = Default::default();

        // first, parse headers and assign to comment
        let comment_header = &block[0];
        comment_header
            .trim_start_matches(PATTERN)
            .split(FIELD_DELIMITER)
            .map(|variable| variable.trim_start())
            .enumerate()
            .for_each(|(variable_pos, variable)| {
                match PARSE_RULES.get(variable_pos) {
                    Some(parser) => parser(&mut comment, variable),
                    None => {
                        // noop
                        warn!(
                            "unexpected variable for comment ({}, {:?})",
                            variable_pos, variable
                        );
                    }
                }
            });

        // read until next pattern is found,
        // append as body
        comment.body = block
            .iter()
            .filter(|&l| !l.starts_with(PATTERN))
            .cloned()
            .collect::<Vec<String>>().join("\n");

        Ok(comment)
    }
}

static PARSE_RULES: [for<'parse> fn(&'parse mut Comment, &str); 2] = [
    |_self, author_id| {
        _self.author = Some(String::from(author_id));
    },
    |_self, parent_id| {
        _self.parent_id = Some(String::from(parent_id));
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    const fixture: &str = r"/comment @kjessec,18235235
Hello World
/comment
";

    #[test]
    fn comment_works() {
        let mut e = fixture
            .split("\n")
            .map(|l| l.to_string())
            .collect::<Vec<String>>();
        let res = Comment::new(e).unwrap();

        dbg!(&res);
        assert_eq!(res.author.unwrap(), "@kjessec");
        assert_eq!(res.parent_id.unwrap(), "18235235");
        assert!(!res.body.is_empty());
    }
}
