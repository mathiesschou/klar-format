use klar_ast::Inline;

pub fn parse_inlines(input: &str) -> Vec<Inline> {
    let mut inlines = Vec::new();
    let mut text = String::new();
    let mut index = 0;

    while index < input.len() {
        let remaining = &input[index..];

        if remaining.starts_with("[[") {
            if let Some((inline, consumed)) = parse_document_link(remaining) {
                flush_text(&mut inlines, &mut text);
                inlines.push(inline);
                index += consumed;
                continue;
            }

            text.push_str(remaining);
            break;
        }

        let ch = remaining.chars().next().unwrap();

        if matches!(ch, '*' | '/') {
            if let Some((inline, consumed)) = parse_delimited(remaining, ch) {
                flush_text(&mut inlines, &mut text);
                inlines.push(inline);
                index += consumed;
                continue;
            }

            text.push_str(remaining);
            break;
        }

        if ch == '#' {
            if let Some((inline, consumed)) = parse_tag(input, index) {
                flush_text(&mut inlines, &mut text);
                inlines.push(inline);
                index += consumed;
                continue;
            }
        }

        text.push(ch);
        index += ch.len_utf8();
    }

    flush_text(&mut inlines, &mut text);
    inlines
}

fn parse_delimited(input: &str, marker: char) -> Option<(Inline, usize)> {
    let close_pos = input[marker.len_utf8()..].find(marker)?;
    let content_end = marker.len_utf8() + close_pos;
    let content = &input[marker.len_utf8()..content_end];
    let consumed = content_end + marker.len_utf8();

    let inline = match marker {
        '*' => Inline::Bold(content.to_string()),
        '/' => Inline::Italic(content.to_string()),
        _ => unreachable!(),
    };

    Some((inline, consumed))
}

fn parse_document_link(input: &str) -> Option<(Inline, usize)> {
    let close_pos = input[2..].find("]]")?;
    let content_end = 2 + close_pos;
    let content = &input[2..content_end];
    let consumed = content_end + 2;

    let (target, alias) = match content.split_once('|') {
        Some((target, alias)) => (
            target.trim().to_string(),
            Some(alias.trim().to_string()),
        ),
        None => (content.trim().to_string(), None),
    };

    if target.is_empty() {
        return None;
    }

    Some((Inline::DocumentLink { target, alias }, consumed))
}

fn parse_tag(input: &str, start: usize) -> Option<(Inline, usize)> {
    if start > 0 {
        let previous = input[..start].chars().next_back().unwrap();
        if is_tag_char(previous) {
            return None;
        }
    }

    let mut end = start + 1;

    for ch in input[start + 1..].chars() {
        if !is_tag_char(ch) {
            break;
        }
        end += ch.len_utf8();
    }

    if end == start + 1 {
        return None;
    }

    Some((Inline::Tag(input[start + 1..end].to_string()), end - start))
}

fn is_tag_char(ch: char) -> bool {
    ch.is_alphanumeric() || matches!(ch, '-' | '_')
}

fn flush_text(inlines: &mut Vec<Inline>, text: &mut String) {
    if !text.is_empty() {
        inlines.push(Inline::Text(std::mem::take(text)));
    }
}
