use crate::lexer::metacharacter::tokenize_metacharacter;
use crate::lexer::number::tokenize_number;
use crate::lexer::single_quotes::tokenize_single_quotes;
use crate::lexer::whitespace::tokenize_whitespace;
use crate::tokens::tokens::Token;
use std::error::Error;
use crate::lexer::dollar_single_quotes::tokenize_dollar_single_quotes;

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
        let single_quotes_result = tokenize_single_quotes(&s, pointer);
        if single_quotes_result.is_err() {
            return Err(single_quotes_result.unwrap_err());
        }
        let single_quotes_pointer = single_quotes_result?;
        if single_quotes_pointer != pointer {
            result.push(Token::SingleQuote);
            pointer = single_quotes_pointer;
            continue;
        }
        
        // Dollar-Single-Quote strings :
        let dsq_result = tokenize_dollar_single_quotes(&s, pointer);
        if dsq_result.is_err() {
            return Err(dsq_result.unwrap_err());
        }
        let dsq_pointer = dsq_result?;
        if dsq_pointer != pointer {
            result.push(Token::DollarSingleQuote);
            pointer = single_quotes_pointer;
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
