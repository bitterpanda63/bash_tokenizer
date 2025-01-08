use crate::lexer::metacharacter::tokenize_metacharacter;
use crate::lexer::number::tokenize_number;
use crate::lexer::single_quote_string::tokenize_single_quote_string;
use crate::lexer::whitespace::tokenize_whitespace;
use crate::tokens::tokens::Token;
use std::error::Error;
pub mod lexer;
pub mod tokens;

pub fn tokenize(s: String) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut result: Vec<Token> = Vec::new();
    let mut pointer = 0;
    while pointer < s.chars().count() {
        // It's important to follow a certain order, whitespace first, keyword before number, ...
        let whitespace_pointer = tokenize_whitespace(&s, pointer);
        if whitespace_pointer != pointer {
            // Whitespace detected
            result.push(Token::WhiteSpace);
            pointer = whitespace_pointer;
            continue;
        }

        let number_pointer = tokenize_number(&s, pointer);
        if number_pointer != pointer {
            // Number detected
            result.push(Token::Number);
            pointer = number_pointer;
            continue;
        }

        // Single-Quote strings :
        let sq_string_result = tokenize_single_quote_string(&s, pointer);
        if sq_string_result.is_err() {
            return Err(sq_string_result.unwrap_err());
        }
        let sq_string_pointer = sq_string_result?;
        if sq_string_pointer != pointer {
            result.push(Token::SingleQuoteString);
            pointer = sq_string_pointer;
            continue;
        }

        // Check for metacharacters ‘|’, ‘&’, ‘;’, ‘(’, ‘)’, ‘<’, or ‘>’
        let metacharacters_token_opt = tokenize_metacharacter(&s, pointer);
        if let Some((metacharacters_token, metacharacters_pointer)) = metacharacters_token_opt {
            result.push(metacharacters_token);
            pointer = metacharacters_pointer;
            continue;
        }
    }

    Ok(result)
}
