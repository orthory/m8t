pub mod comment;
pub mod frontmatter;

#[derive(Clone, Debug)]
pub enum SectionType {
    Empty,
    Body(String),

    // extensions
    FrontMatter(frontmatter::FrontMatter),
    Comment(comment::Comment),

    // todo
    Task,
    Event,
}

pub enum CommandBlockType {
    Line(Pattern),
    Block(Pattern, Pattern),
}

pub enum Pattern {
    Exact(&'static str),
    StartsWith(&'static str),
}

impl Pattern {
    pub fn is_matched(&self, line: &String) -> bool {
        match *self {
            Pattern::Exact(patt) => line.eq(patt),
            Pattern::StartsWith(patt) => line.starts_with(patt),
        }
    }
}

type LineFeed<'file> = fn(Vec<String>) -> anyhow::Result<SectionType>;

pub const SECTION_ENTRIES: [(CommandBlockType, LineFeed); 2] = [
    (
        CommandBlockType::Block(Pattern::Exact("---"), Pattern::Exact("---")),
        |block| frontmatter::FrontMatter::new(block).map(SectionType::FrontMatter),
    ),
    (
        CommandBlockType::Block(Pattern::StartsWith("/comment"), Pattern::Exact("/comment")),
        |block| comment::Comment::new(block).map(SectionType::Comment),
    ),
];
