pub mod frontmatter;
pub mod body;
pub mod comment;

pub enum ReadLineResult {
    Keep,
    Done
}

pub trait Section: Sized
{
    fn new(line_pos: u64) -> Self;
    fn match_line(line: &str) -> bool;
    fn read_line(&mut self, line: &str) -> ReadLineResult;

}

#[derive(Debug)]
pub enum SectionType {
    Empty,
    Body(body::Body),
    FrontMatter(frontmatter::FrontMatter),
    Comment(comment::Comment)
}

impl Section for SectionType {
    fn new(line_pos: u64) -> Self {
        unreachable!()
    }

    fn match_line(line: &str) -> bool {
        unreachable!()
    }

    fn read_line(&mut self, line: &str) -> ReadLineResult {
        match self {
            SectionType::FrontMatter(c) => c.read_line(line),
            SectionType::Comment(c) => c.read_line(line),
            SectionType::Body(c) => c.read_line(line),
            r => {
                dbg!(r);
                panic!("wtf");
            }
        }
    }
}

#[derive(Debug)]
pub enum LineDelimiters<'parse> {
    Body(&'parse str),
    FrontMatter(&'parse str),
    Comment(&'parse str),
}