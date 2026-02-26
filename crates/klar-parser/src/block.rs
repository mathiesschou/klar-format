use crate::inline::parse_inlines;
use klar_ast::{Block, Inline};

pub fn parse_blocks(input: &str) -> Vec<Block> {
    let mut blocks = Vec::new();

    for line in input.lines() {
        if line.starts_with("### ") {
            blocks.push(Block::Heading {
                level: 3,
                inlines: vec![Inline::Text(line[4..].to_string())],
            });
        } else if line.starts_with("## ") {
            blocks.push(Block::Heading {
                level: 2,
                inlines: vec![Inline::Text(line[3..].to_string())],
            });
        } else if line.starts_with("# ") {
            blocks.push(Block::Heading {
                level: 1,
                inlines: vec![Inline::Text(line[2..].to_string())],
            });
        } else {
            blocks.push(Block::Paragraph(parse_inlines(line)));
        }
    }

    blocks
}
