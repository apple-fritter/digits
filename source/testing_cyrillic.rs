fn sanitize_input(
    input: &str,
    allow_alphabetic: bool,
    allow_numeric: bool,
    allow_punctuational: bool,
    allow_unicode: bool,
    allow_cyrillic: bool,
) -> String {
    let mut sanitized = String::new();

    let mut prev_whitespace = false;
    let collapse_whitespace = |c: char| -> bool { c.is_ascii_whitespace() };

    for c in input.chars() {
        if c.is_ascii_digit() && allow_numeric {
            sanitized.push(c);
            prev_whitespace = false;
        } else if c.is_ascii_alphabetic() && allow_alphabetic {
            sanitized.push(c);
            prev_whitespace = false;
        } else if c.is_ascii_punctuation() && allow_punctuational {
            sanitized.push(c);
            prev_whitespace = false;
        } else if !c.is_ascii() && allow_unicode {
            sanitized.push(c);
            prev_whitespace = false;
        } else if c.is_cyrillic() && allow_cyrillic {
            sanitized.push(c);
            prev_whitespace = false;
        } else if collapse_whitespace(c) && !prev_whitespace {
            sanitized.push(' ');
            prev_whitespace = true;
        }
    }

    sanitized
}
