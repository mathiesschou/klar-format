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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_can_hold_blocks() {
        let doc = Document {
            blocks: vec![
                Block::Heading {
                    level: 1,
                    inlines: vec![Inline::Text("Hello".to_string())],
                },
                Block::Paragraph(vec![Inline::Text("World".to_string())]),
            ],
        };
        assert_eq!(doc.blocks.len(), 2);
    }
}
