#[derive(Debug, PartialEq)]
pub enum Token {
    WhiteSpace,
    SingleQuoteString,
    DoubleQuoteString,
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
