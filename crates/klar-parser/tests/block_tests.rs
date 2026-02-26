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
