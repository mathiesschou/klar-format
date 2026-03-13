use crate::inline::parse_inlines;
use klar_ast::{Block, Inline};

pub fn parse_blocks(input: &str) -> Vec<Block> {
    let mut blocks = Vec::new();
    let mut paragraph_inlines = Vec::new();
    let mut blank_line_count = 0;
    let mut pending_hard_break = false;

    for line in input.lines() {
        if line.trim().is_empty() {
            blank_line_count += 1;
            continue;
        }

        if blank_line_count > 0 {
            flush_paragraph(&mut blocks, &mut paragraph_inlines);
            pending_hard_break = false;

            if blank_line_count >= 2 && !blocks.is_empty() {
                blocks.push(Block::SectionBreak);
            }

            blank_line_count = 0;
        }

        if line.starts_with("### ") {
            flush_paragraph(&mut blocks, &mut paragraph_inlines);
            pending_hard_break = false;
            blocks.push(Block::Heading {
                level: 3,
                inlines: vec![Inline::Text(line[4..].to_string())],
            });
        } else if line.starts_with("## ") {
            flush_paragraph(&mut blocks, &mut paragraph_inlines);
            pending_hard_break = false;
            blocks.push(Block::Heading {
                level: 2,
                inlines: vec![Inline::Text(line[3..].to_string())],
            });
        } else if line.starts_with("# ") {
            flush_paragraph(&mut blocks, &mut paragraph_inlines);
            pending_hard_break = false;
            blocks.push(Block::Heading {
                level: 1,
                inlines: vec![Inline::Text(line[2..].to_string())],
            });
        } else {
            if !paragraph_inlines.is_empty() {
                if pending_hard_break {
                    paragraph_inlines.push(Inline::LineBreak);
                } else {
                    push_text(&mut paragraph_inlines, " ");
                }
            }

            let (content, has_hard_break) = match line.strip_suffix('\\') {
                Some(stripped) => (stripped, true),
                None => (line, false),
            };

            for inline in parse_inlines(content) {
                push_inline(&mut paragraph_inlines, inline);
            }

            pending_hard_break = has_hard_break;
        }
    }

    flush_paragraph(&mut blocks, &mut paragraph_inlines);

    blocks
}

fn flush_paragraph(blocks: &mut Vec<Block>, paragraph_inlines: &mut Vec<Inline>) {
    if !paragraph_inlines.is_empty() {
        blocks.push(Block::Paragraph(std::mem::take(paragraph_inlines)));
    }
}

fn push_text(inlines: &mut Vec<Inline>, text: &str) {
    if text.is_empty() {
        return;
    }

    match inlines.last_mut() {
        Some(Inline::Text(existing)) => existing.push_str(text),
        _ => inlines.push(Inline::Text(text.to_string())),
    }
}

fn push_inline(inlines: &mut Vec<Inline>, inline: Inline) {
    match inline {
        Inline::Text(text) => push_text(inlines, &text),
        other => inlines.push(other),
    }
}
