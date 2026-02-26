use klar_ast::{Block, Document, Inline};

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
