use klar_ast::{Block, Document, Inline};

fn parse_inlines(input: &str) -> Vec<Inline> {
    let mut inlines = Vec::new();
    let mut remaining = input;
    todo!() //TODO inplement parser logic for bold.  
}

pub fn parse(input: &str) -> Document {
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
            blocks.push(Block::Paragraph(vec![Inline::Text(line.to_string())]));
        }
    }

    Document { blocks }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_heading_level_1() {
        let result = parse("# Hello");
        assert_eq!(
            result,
            Document {
                blocks: vec![Block::Heading {
                    level: 1,
                    inlines: vec![Inline::Text("Hello".to_string())]
                },],
            }
        );
    }

    #[test]
    fn parse_heading_level_2() {
        let result = parse("## World");
        assert_eq!(
            result,
            Document {
                blocks: vec![Block::Heading {
                    level: 2,
                    inlines: vec![Inline::Text("World".to_string())]
                },],
            }
        );
    }

    #[test]
    fn parse_heading_level_3() {
        let result = parse("### Foo");
        assert_eq!(
            result,
            Document {
                blocks: vec![Block::Heading {
                    level: 3,
                    inlines: vec![Inline::Text("Foo".to_string())]
                },],
            }
        );
    }

    #[test]
    fn parse_bold() {
        let result = parse("*hello*");
        assert_eq!(
            result,
            Document {
                blocks: vec![Block::Paragraph(vec![Inline::Bold("hello".to_string())])]
            }
        );
    }
}
