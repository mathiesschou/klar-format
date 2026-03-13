use klar_ast::{Block, Document, Inline};
use klar_parser::parse;

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
fn parse_single_newline_within_paragraph_as_space() {
    let result = parse("First line\nsecond line");
    assert_eq!(
        result,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Text(
                "First line second line".to_string()
            )])],
        }
    );
}

#[test]
fn parse_trailing_backslash_as_hard_line_break() {
    let result = parse("First line\\\nsecond line");
    assert_eq!(
        result,
        Document {
            blocks: vec![Block::Paragraph(vec![
                Inline::Text("First line".to_string()),
                Inline::LineBreak,
                Inline::Text("second line".to_string()),
            ])],
        }
    );
}

#[test]
fn parse_single_blank_line_as_new_paragraph() {
    let result = parse("First paragraph\n\nSecond paragraph");
    assert_eq!(
        result,
        Document {
            blocks: vec![
                Block::Paragraph(vec![Inline::Text("First paragraph".to_string())]),
                Block::Paragraph(vec![Inline::Text("Second paragraph".to_string())]),
            ],
        }
    );
}

#[test]
fn parse_double_blank_line_as_section_break() {
    let result = parse("First paragraph\n\n\nSecond paragraph");
    assert_eq!(
        result,
        Document {
            blocks: vec![
                Block::Paragraph(vec![Inline::Text("First paragraph".to_string())]),
                Block::SectionBreak,
                Block::Paragraph(vec![Inline::Text("Second paragraph".to_string())]),
            ],
        }
    );
}
