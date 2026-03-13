use klar_ast::{Block, Document, Inline};

#[test]
fn document_can_hold_blocks() {
    let doc = Document {
        blocks: vec![
            Block::Heading {
                level: 1,
                inlines: vec![Inline::Text("Hello".to_string())],
            },
            Block::Paragraph(vec![
                Inline::Text("World ".to_string()),
                Inline::DocumentLink {
                    target: "project-plan".to_string(),
                    alias: Some("Project Plan".to_string()),
                },
                Inline::Text(" ".to_string()),
                Inline::Tag("important".to_string()),
            ]),
            Block::Paragraph(vec![
                Inline::Bold("Bold".to_string()),
                Inline::LineBreak,
                Inline::Italic("Italic".to_string()),
            ]),
            Block::SectionBreak,
        ],
    };
    assert_eq!(doc.blocks.len(), 4);
}
