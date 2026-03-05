use klar_ast::Inline;

pub fn parse_inlines(input: &str) -> Vec<Inline> {
    let mut inlines = Vec::new();
    let mut remaining = input;

    while !remaining.is_empty() {
        // Find next formatting marker
        let star_pos = remaining.find('*');
        let slash_pos = remaining.find('/');

        // Determine which marker comes first
        let (marker_pos, marker_char) = match (star_pos, slash_pos) {
            (Some(s), Some(sl)) if s < sl => (s, '*'),
            (Some(s), Some(sl)) if sl < s => (sl, '/'),
            (Some(s), None) => (s, '*'),
            (Some(_), Some(_)) => unreachable!(), // Logically not possible
            (None, Some(sl)) => (sl, '/'),
            (None, None) => {
                inlines.push(Inline::Text(remaining.to_string()));
                break;
            }
        };

        // Check if there's a closing matching char
        if let Some(close_pos) = remaining[marker_pos + 1..].find(marker_char) {
            // If char is not from position 0, then push normal text until first char
            if marker_pos > 0 {
                inlines.push(Inline::Text(remaining[..marker_pos].to_string()));
            }
            // Find formatted text
            // Ranges is excluding "end" in slices [start..end]
            let formatted_text = &remaining[marker_pos + 1..marker_pos + 1 + close_pos];

            // Push correct format
            match marker_char {
                '*' => inlines.push(Inline::Bold(formatted_text.to_string())),
                '/' => inlines.push(Inline::Italic(formatted_text.to_string())),
                _ => unreachable!(),
            }
            // Continue for the rest of the line
            remaining = &remaining[marker_pos + 2 + close_pos..];
        } else {
            // No formatted text - treat everything as text
            inlines.push(Inline::Text(remaining.to_string()));
            break;
        }
    }

    inlines
}
