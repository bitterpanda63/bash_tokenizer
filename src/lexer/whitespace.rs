pub fn tokenize_whitespace(content: &String, start: usize) -> usize {
    let mut pointer: usize = start;
    let mut char = content.chars().nth(pointer).unwrap();
    while char.is_ascii_whitespace() {
        pointer += 1;
        if pointer >= content.len() {
            break;
        }

        char = content.chars().nth(pointer).unwrap();
    }
    pointer
}

#[cfg(test)]
mod tests {
    use crate::lexer::whitespace::tokenize_whitespace;

    #[test]
    pub fn test_tokenize_whitespace_basic() {
        assert_eq!(2, tokenize_whitespace(&String::from("NoWhitespace"), 2));
        assert_eq!(0, tokenize_whitespace(&String::from("NoWhitespace"), 0));
        assert_eq!(11, tokenize_whitespace(&String::from("NoWhitespace"), 11)); // Last character

        assert_eq!(1, tokenize_whitespace(&String::from(" "), 0));
        assert_eq!(1, tokenize_whitespace(&String::from(" |"), 0));
        assert_eq!(1, tokenize_whitespace(&String::from("\t"), 0));
        assert_eq!(0, tokenize_whitespace(&String::from("-"), 0));
        assert_eq!(2, tokenize_whitespace(&String::from("  "), 0));
        assert_eq!(4, tokenize_whitespace(&String::from("    "), 0));
        assert_eq!(4, tokenize_whitespace(&String::from("    |"), 0));
    }
}
