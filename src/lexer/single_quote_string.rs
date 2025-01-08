/// tokenize_single_quote_string
/// > 3.1.2.2 Single Quotes
/// > Enclosing characters in single quotes (‘'’) preserves the literal value of each character within the quotes.
/// > A single quote may not occur between single quotes, even when preceded by a backslash.
/// This means that we do not have to check for anything except for the end of the string.
pub fn tokenize_single_quote_string(
    content: &String,
    start: usize,
) -> Result<usize, Box<dyn std::error::Error>> {
    let mut pointer: usize = start;
    let mut char = content.chars().nth(pointer).unwrap();
    if char != '\'' {
        return Ok(start);
    }
    while char != '\'' || pointer == start {
        pointer += 1;
        if pointer >= content.len() {
            return Err(format!("Unterminated string literal at index {}", start).into());
        }

        char = content.chars().nth(pointer).unwrap();
    }
    Ok(pointer + 1) // We always point to the character that is excluded
}

#[cfg(test)]
mod tests {
    use crate::lexer::single_quote_string::tokenize_single_quote_string;

    #[test]
    pub fn test_tokenize_single_quote_string_basic() {
        assert_eq!(
            2,
            tokenize_single_quote_string(&String::from("StartsNotWithSingleQuote"), 2).unwrap()
        );
        assert_eq!(
            0,
            tokenize_single_quote_string(&String::from("S'''t'a'rtsNotWithSingleQuote"), 0)
                .unwrap()
        );
        assert_eq!(
            23,
            tokenize_single_quote_string(&String::from("StartsNotWithSingleQuote"), 23).unwrap()
        ); // Last character

        assert_eq!(
            2,
            tokenize_single_quote_string(&String::from("''"), 0).unwrap()
        );
        assert_eq!(
            13,
            tokenize_single_quote_string(&String::from("'Hello World'"), 0).unwrap()
        );

        assert_eq!(
            "Unterminated string literal at index 5",
            tokenize_single_quote_string(&String::from("echo 'Hello ; world"), 5)
                .unwrap_err()
                .to_string()
        );
    }
}
