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

#[test]
fn parse_underscore_delimited_text_as_plain_text() {
    let result = parse("hello _italic_ world");
    assert_eq!(
        result,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Text(
                "hello _italic_ world".to_string()
            )])]
        }
    )
}

#[test]
fn parse_document_link() {
    let result = parse("[[project-plan]]");
    assert_eq!(
        result,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::DocumentLink {
                target: "project-plan".to_string(),
                alias: None,
            }])]
        }
    );
}

#[test]
fn parse_document_link_with_alias() {
    let result = parse("[[project-plan | Project Plan]]");
    assert_eq!(
        result,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::DocumentLink {
                target: "project-plan".to_string(),
                alias: Some("Project Plan".to_string()),
            }])]
        }
    );
}

#[test]
fn parse_document_link_with_surrounding_text() {
    let result = parse("See [[project-plan]] today");
    assert_eq!(
        result,
        Document {
            blocks: vec![Block::Paragraph(vec![
                Inline::Text("See ".to_string()),
                Inline::DocumentLink {
                    target: "project-plan".to_string(),
                    alias: None,
                },
                Inline::Text(" today".to_string()),
            ])]
        }
    );
}

#[test]
fn parse_unclosed_document_link_as_text() {
    let result = parse("See [[project-plan");
    assert_eq!(
        result,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Text(
                "See [[project-plan".to_string()
            )])]
        }
    );
}

#[test]
fn parse_section_link_like_syntax_as_plain_document_target() {
    let result = parse("[[project-plan # decisions]]");
    assert_eq!(
        result,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::DocumentLink {
                target: "project-plan # decisions".to_string(),
                alias: None,
            }])]
        }
    );
}

#[test]
fn parse_tag() {
    let result = parse("#project");
    assert_eq!(
        result,
        Document {
            blocks: vec![Block::Paragraph(vec![Inline::Tag("project".to_string())])]
        }
    );
}

#[test]
fn parse_multiple_tags() {
    let result = parse("#project #important");
    assert_eq!(
        result,
        Document {
            blocks: vec![Block::Paragraph(vec![
                Inline::Tag("project".to_string()),
                Inline::Text(" ".to_string()),
                Inline::Tag("important".to_string()),
            ])]
        }
    );
}

#[test]
fn parse_numeric_tag_in_normal_text() {
    let result = parse("Release #2024 soon");
    assert_eq!(
        result,
        Document {
            blocks: vec![Block::Paragraph(vec![
                Inline::Text("Release ".to_string()),
                Inline::Tag("2024".to_string()),
                Inline::Text(" soon".to_string()),
            ])]
        }
    );
}
