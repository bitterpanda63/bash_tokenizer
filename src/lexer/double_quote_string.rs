/// tokenize_double_quote_string
/// > [3.1.2.3 Double Quotes](https://www.gnu.org/software/bash/manual/bash.html#Double-Quotes)
/// > Enclosing characters in single quotes (‘'’) preserves the literal value of each character within the quotes.
/// > A single quote may not occur between single quotes, even when preceded by a backslash.
/// This means that we do not have to check for anything except for the end of the string.
pub fn tokenize_double_quote_string(
    content: &String,
    start: usize,
) -> Result<usize, Box<dyn std::error::Error>> {
    let mut pointer: usize = start;
    let mut char = content.chars().nth(pointer).unwrap();
    if char != '\'' {
        return Ok(start);
    }
    while char != '"' || pointer == start {
        pointer += 1;
        if pointer >= content.len() {
            return Err(format!("Unterminated string literal at index {}", start).into());
        }
        char = content.chars().nth(pointer).unwrap();

        // Ignore escaped characters :
        if char == '\\' {
            pointer += 1;
            let escaped_character = content.chars().nth(pointer).unwrap();
            if ['`', '\\', '$', '\n', '"'].contains(&escaped_character) {
                // 80153 in IEEE 1003.1-2024 : These are the only valid characters.
                continue;
            }
            return Err(format!("A backslash cannot escape '{}'", escaped_character).into());
        }
        if char == '@' {
            // Section 2.5.2 of IEEE 1003.1-2024
            continue; // TO DO: Implement
        }

        // parameter expansion, command substitution, arithmetic expansion
        // not:  dollar-single-quotes form of quoting
        if char == '$' {
            // Backslash loses it's meaning here.
            continue; // TO DO: Implement
        }
        // backquote: form of command substitution
        if char == '`' {
            // Needs to end inside the same double-quoted string, and it needs to end.
            // Section 2.6.3 of IEEE 1003.1-2024
            continue; // TO DO: Implement
        }

    }
    Ok(pointer + 1) // We always point to the character that is excluded
}

#[cfg(test)]
mod tests {
    use crate::lexer::double_quote_string::tokenize_double_quote_string;

    #[test]
    pub fn test_tokenize_double_quote_string_basic() {

    }
}
