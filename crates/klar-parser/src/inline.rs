use klar_ast::Inline;

pub fn parse_inlines(input: &str) -> Vec<Inline> {
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
