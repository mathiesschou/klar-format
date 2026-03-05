use klar_ast::{Block, Document, Inline};
use klar_parser::parse;

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
            blocks: vec![Block::Paragraph(vec![Inline::Text(
                "hello *incomplete".to_string()
            )])]
        }
    );
}

#[test]
fn parse_empty_bold() {
    let result = parse("**");
    assert_eq!(
        result,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Bold("".to_string())])]
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

#[test]
fn parse_italic() {
    let result = parse("/hello/");
    assert_eq!(
        result,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Italic("hello".to_string())])]
        }
    )
}

#[test]
fn parse_italic_with_text_around() {
    let result = parse("hello /italic/ world");
    assert_eq!(
        result,
        Document {
            blocks: vec![Block::Paragraph(vec![
                Inline::Text("hello ".to_string()),
                Inline::Italic("italic".to_string()),
                Inline::Text(" world".to_string())
            ])]
        }
    )
}

#[test]
fn parse_italic_and_bold() {
    let result = parse("*bold* and /italic/");
    assert_eq!(
        result,
        Document {
            blocks: vec![Block::Paragraph(vec![
                Inline::Bold("bold".to_string()),
                Inline::Text(" and ".to_string()),
                Inline::Italic("italic".to_string()),
            ])]
        }
    )
}
