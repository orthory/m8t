use std::io::{BufRead, Cursor};
use sections::{Section, LineDelimiters, SectionType, ReadLineResult};

// takes a buffer and ..
type ParseResult<'parse> = Vec<SectionType>;

// wtf
pub fn parse<'file>(
    res: &'file mut ParseResult,
    doc: Cursor<&str>
) -> anyhow::Result<&'file mut ParseResult<'file>> {
    Ok(doc
        .lines()
        .enumerate()
        .fold(res, | sdstack, (line_pos, line) | -> &mut ParseResult {
            let last_in_stack = sdstack.last_mut();
            let l = line.expect("line stream failed");

            let result = match parse_line(l.as_str()) {
                // handle frontmatter
                LineDelimiters::FrontMatter(line) => {
                    match last_in_stack {
                        Some(SectionType::FrontMatter(c)) => {
                            c.read_line(line)
                        },
                        _ => {
                            let mut next = sections::frontmatter::FrontMatter::new(line_pos as u64);
                            let res = next.read_line(line);
                            sdstack.push(SectionType::FrontMatter(next));
                            
                            res
                        }
                    }
                }

                // handle comment
                LineDelimiters::Comment(line) => {
                    match last_in_stack {
                        Some(SectionType::Comment(c)) => {
                            c.read_line(line)
                        },
                        _ => {
                            let mut next = sections::comment::Comment::new(line_pos as u64);
                            let res = next.read_line(line);
                            sdstack.push(SectionType::Comment(next));
                            
                            res
                        }
                    }
                }

                // handle body
                LineDelimiters::Body(line) => {
                    match last_in_stack {
                        Some(SectionType::Body(c)) => {
                            c.read_line(line)
                        },
                        Some(SectionType::Empty) => {
                            let mut next = sections::body::Body::new(line_pos as u64);
                            let res = next.read_line(line);
                            
                            sdstack.pop();
                            sdstack.push(SectionType::Body(next));

                            res
                        },
                        x => {
                            x.unwrap().read_line(line)
                        }
                    }
                }
            };
            
            match result {
                ReadLineResult::Keep => sdstack,
                ReadLineResult::Done => {
                    sdstack.push(SectionType::Empty);
                    sdstack
                }
            }
        })
    )
}

pub fn parse_line(line: &str) -> LineDelimiters {
    match line.trim() {
        l if sections::frontmatter::FrontMatter::match_line(l) => LineDelimiters::FrontMatter(l),
        l if sections::comment::Comment::match_line(l) => LineDelimiters::Comment(l),
        l => LineDelimiters::Body(l),
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use crate::parser::{parse, ParseResult};

    #[test]
    fn parser_works() {

        const FIXTURE: &str = r"
---
title: hahaha
tags:
- asdf
- ij
- oin
author: kjessec
---

# Hello
## Hello World

WTF

---comment @kjessec,0
wtf is happening here?
---comment

---comment @kjessec,1
what?
---comment
";

        let file = Cursor::new(FIXTURE.trim_start());
        let mut res: ParseResult = vec![];
        let next = parse(&mut res, file).unwrap();

        for c in next {
            println!("{:?}", c);
        }
    }
}