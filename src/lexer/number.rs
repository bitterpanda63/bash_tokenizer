pub fn tokenize_number(content: &String, start: usize) -> usize {
    let mut pointer: usize = start;
    let mut char = content.chars().nth(pointer).unwrap();
    while char.is_numeric() {
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
    use crate::lexer::number::tokenize_number;

    #[test]
    pub fn test_tokenize_number_basic() {
        assert_eq!(2, tokenize_number(&String::from("NotANumber"), 2));
        assert_eq!(0, tokenize_number(&String::from("NotANumber"), 0));
        assert_eq!(9, tokenize_number(&String::from("NotANumber"), 9)); // Last character

        assert_eq!(1, tokenize_number(&String::from("8"), 0));
        assert_eq!(1, tokenize_number(&String::from("8-"), 0));
        assert_eq!(1, tokenize_number(&String::from("8o"), 0));
        assert_eq!(1, tokenize_number(&String::from("8\n9"), 0));
        assert_eq!(1, tokenize_number(&String::from("8 9"), 0));
        assert_eq!(1, tokenize_number(&String::from("8.9"), 0));
        assert_eq!(3, tokenize_number(&String::from("801"), 0));
        assert_eq!(3, tokenize_number(&String::from("801 - 902"), 0));
    }
}
