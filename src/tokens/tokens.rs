#[derive(Debug, PartialEq)]
pub enum Token {
    WhiteSpace,
    SingleQuote,
    DoubleQuote,
    DollarSingleQuote,
    Keyword,
    Number,
    // Metacharacters :
    Pipe,         // |
    Ampersand,    // &
    Semicolon,    // ;
    LParenthesis, // (
    RParenthesis, // )
    LessAnd,      // <
    GreatAnd,     // >
}
