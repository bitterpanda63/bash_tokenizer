use crate::tokens::tokens::Token;

/// tokenize_metacharacter
/// > [metacharacter](https://www.gnu.org/software/bash/manual/bash.html#Single-Quotes)
/// > A character that, when unquoted, separates words. A metacharacter is a space, tab, newline,
/// > **or one of the following characters: ‘|’, ‘&’, ‘;’, ‘(’, ‘)’, ‘<’, or ‘>’.**
/// TO DO: This function should in the future also check for Redirection operators, ...
pub fn tokenize_metacharacter(content: &str, start: usize) -> Option<(Token, usize)> {
    let mut pointer: usize = start;
    let char = content.chars().nth(pointer).unwrap();
    // We loop here for double characters (in the future) :
    loop {
        pointer += 1;
        if char == '|' {
            return Some((Token::Pipe, pointer));
        }
        if char == '&' {
            return Some((Token::Ampersand, pointer));
        }
        if char == ';' {
            return Some((Token::Semicolon, pointer));
        }
        if char == '(' {
            return Some((Token::LParenthesis, pointer));
        }
        if char == ')' {
            return Some((Token::RParenthesis, pointer));
        }
        if char == '<' {
            return Some((Token::LessAnd, pointer));
        }
        if char == '>' {
            return Some((Token::GreatAnd, pointer));
        }
        break;
    }

    None // Default is None, no token found
}
#[cfg(test)]
mod tests {
    use crate::lexer::metacharacter::tokenize_metacharacter;
    use crate::tokens::tokens::Token;

    #[test]
    fn test_tokenize_metacharacter_basic() {
        // Test for each metacharacter
        assert_eq!(tokenize_metacharacter("|", 0), Some((Token::Pipe, 1)));
        assert_eq!(tokenize_metacharacter("&", 0), Some((Token::Ampersand, 1)));
        assert_eq!(tokenize_metacharacter(";", 0), Some((Token::Semicolon, 1)));
        assert_eq!(
            tokenize_metacharacter("(", 0),
            Some((Token::LParenthesis, 1))
        );
        assert_eq!(
            tokenize_metacharacter(")", 0),
            Some((Token::RParenthesis, 1))
        );
        assert_eq!(tokenize_metacharacter("<", 0), Some((Token::LessAnd, 1)));
        assert_eq!(tokenize_metacharacter(">", 0), Some((Token::GreatAnd, 1)));
    }

    #[test]
    fn test_tokenize_metacharacter_no_match() {
        // Test for characters that are not metacharacters
        assert_eq!(tokenize_metacharacter("a", 0), None);
        assert_eq!(tokenize_metacharacter(" ", 0), None); // Space is not handled in the current implementation
        assert_eq!(tokenize_metacharacter("abc", 0), None);
    }

    #[test]
    fn test_tokenize_metacharacter_multiple_characters() {
        // Test for a string with multiple characters, only the first metacharacter should be tokenized
        assert_eq!(tokenize_metacharacter("|&;", 0), Some((Token::Pipe, 1)));
        assert_eq!(tokenize_metacharacter("&;", 1), Some((Token::Semicolon, 2)));
        assert_eq!(
            tokenize_metacharacter("();", 1),
            Some((Token::RParenthesis, 2))
        );
        assert_eq!(tokenize_metacharacter("<>", 0), Some((Token::LessAnd, 1)));
    }
}
