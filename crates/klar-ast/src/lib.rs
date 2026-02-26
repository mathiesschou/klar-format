#[derive(Debug, PartialEq)]
pub struct Document {
    pub blocks: Vec<Block>,
}

#[derive(Debug, PartialEq)]
pub enum Block {
    Heading { level: u8, inlines: Vec<Inline> },
    Paragraph(Vec<Inline>),
}

#[derive(Debug, PartialEq)]
pub enum Inline {
    Text(String),
    Bold(String),
}
