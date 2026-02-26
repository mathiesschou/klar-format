use klar_ast::{Block, Document, Inline};

fn parse_inlines(input: &str) -> Vec<Inline> {
    let mut inlines = Vec::new();
    let mut remaining = input;

    while !remaining.is_empty() {
        if let Some(star_pos) = remaining.find('*') {
            // Check if there's a closing star first
            if let Some(close_pos) = remaining[star_pos + 1..].find('*') {
                // Valid bold found
                if star_pos > 0 {
                    inlines.push(Inline::Text(remaining[..star_pos].to_string()));
                }
                let bold_text = &remaining[star_pos + 1..star_pos + 1 + close_pos];
                inlines.push(Inline::Bold(bold_text.to_string()));
                remaining = &remaining[star_pos + 2 + close_pos..];
            } else {
                // No closing star - treat everything as text
                inlines.push(Inline::Text(remaining.to_string()));
                break;
            }
        } else {
            // No stars at all - rest is text
            inlines.push(Inline::Text(remaining.to_string()));
            break;
        }
    }

    inlines
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
            blocks.push(Block::Paragraph(parse_inlines(line)));
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

    #[test]
    fn parse_bold_with_text_around() {
        let result = parse("hello *bold* world");
        assert_eq!(
            result,
            Document {
                blocks: vec![Block::Paragraph(vec![
                    Inline::Text("hello ".to_string()),
                    Inline::Bold("bold".to_string()),
                    Inline::Text(" world".to_string())
                ])]
            }
        );
    }

    #[test]
    fn parse_multiple_bold_sections() {
        let result = parse("*first* normal *second*");
        assert_eq!(
            result,
            Document {
                blocks: vec![Block::Paragraph(vec![
                    Inline::Bold("first".to_string()),
                    Inline::Text(" normal ".to_string()),
                    Inline::Bold("second".to_string())
                ])]
            }
        );
    }

    #[test]
    fn parse_unclosed_bold_as_text() {
        let result = parse("hello *incomplete");
        assert_eq!(
            result,
            Document {
                blocks: vec![Block::Paragraph(vec![
                    Inline::Text("hello *incomplete".to_string())
                ])]
            }
        );
    }

    #[test]
    fn parse_empty_bold() {
        let result = parse("**");
        assert_eq!(
            result,
            Document {
                blocks: vec![Block::Paragraph(vec![
                    Inline::Bold("".to_string())
                ])]
            }
        );
    }

    #[test]
    fn parse_bold_at_start() {
        let result = parse("*bold* at start");
        assert_eq!(
            result,
            Document {
                blocks: vec![Block::Paragraph(vec![
                    Inline::Bold("bold".to_string()),
                    Inline::Text(" at start".to_string())
                ])]
            }
        );
    }

    #[test]
    fn parse_bold_at_end() {
        let result = parse("at end *bold*");
        assert_eq!(
            result,
            Document {
                blocks: vec![Block::Paragraph(vec![
                    Inline::Text("at end ".to_string()),
                    Inline::Bold("bold".to_string())
                ])]
            }
        );
    }
}
