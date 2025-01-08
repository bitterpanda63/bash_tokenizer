macro_rules! increment_pointer {
    ($pointer:expr, $content:expr, $start:expr, $char:expr) => {{
        $pointer += 1;
        if $pointer >= $content.len() {
            return Err(format!("Unterminated dollar-single-quote at index {}", $start).into());
        }
        $char = $content.chars().nth($pointer).unwrap();
    }};
}
macro_rules! decrement_pointer {
    ($pointer:expr, $content:expr, $char:expr) => {{
        $pointer -= 1;
        $char = $content.chars().nth($pointer).unwrap();
    }};
}

/// tokenize_dollar_single_quotes
/// > [3.1.2.4 ANSI-C Quoting](https://www.gnu.org/software/bash/manual/bash.html#ANSI_002dC-Quoting)
/// > Character sequences of the form `$’string’` are treated as a special kind of single quotes
/// It's basically the same as single_quotes.rs except we need to validate backslashes.
pub fn tokenize_dollar_single_quotes(
    content: &String,
    start: usize,
) -> Result<usize, Box<dyn std::error::Error>> {
    let mut pointer = start;
    let mut char = content.chars().nth(pointer).unwrap();

    // Validation of `$'` start :
    if char != '$' || !(pointer + 1 < content.len()) {
        // Check it's a dollar single quoted string and that we can fetch the following character.
        return Ok(start);
    }
    pointer += 1; // Skipped `$`
    char = content.chars().nth(pointer).unwrap();
    if char != '\'' {
        // A dollar-single-quote always starts with `$'`
        return Ok(start);
    }

    while char != '\'' || pointer == start + 1 {
        // Add 1 for the offset of `$`
        increment_pointer!(pointer, content, start, char);
        char = content.chars().nth(pointer).unwrap();

        // Check for backslashes :
        if char == '\\' {
            increment_pointer!(pointer, content, start, char);
            if ['a', 'b', 'e', 'f', 'n', 'r', 't', 'v', '\\', '\'', '"'].contains(&char) {
                continue; // Specified in IEEE 1003.1-2024
            }

            // \cX yields the control character :
            if char == 'c' {
                increment_pointer!(pointer, content, start, char);
                // Allows a-Z, [, ], ^, _, ? and a backslash
                if char.is_ascii_alphabetic() || ['[', ']', '^', '_', '?'].contains(&char) {
                    continue;
                }
                if char == '\\' {
                    increment_pointer!(pointer, content, start, char);
                    if char == '\\' {
                        continue; // For a backslash you have to put \c\\, so we need to check twice for the backslash.
                    }
                }

                // Not an allowed character :
                return Err(format!(
                    r"\c not followed by a Circumflex Control Character at index {}",
                    pointer
                )
                .into());
            }

            // \xXX yields the byte whose value is the hexadecimal value XX (one or more hexadecimal digits) :
            if char == 'x' {
                increment_pointer!(pointer, content, start, char);
                if !char.is_ascii_hexdigit() {
                    return Err(format!(
                        r"\x not followed by a hexadecimal character at index {}",
                        pointer
                    )
                    .into());
                }
                increment_pointer!(pointer, content, start, char);
                if !char.is_ascii_hexdigit() {
                    decrement_pointer!(pointer, content, char); // Reset pointer here.
                }
                continue;
            }

            // \ddd yields the byte whose value is the octal value ddd (one to three octal digits) :
            let mut octal_counter = 0;
            while is_ascii_octal(char) && octal_counter < 3 {
                octal_counter += 1;
                increment_pointer!(pointer, content, start, char);
            }
            if octal_counter != 0 {
                // This means it matched \ddd, so we can safely continue :
                decrement_pointer!(pointer, content, char); // Go back one character (pointer should end on octal)
                continue;
            }

            // Bash-specific : \E, \?, \uHHHH, \UHHHHHHHH currently not supported.

            // Backslash matched nothing :
            return Err(format!(r"\ followed by an invalid character at index {}", pointer).into());
        }
    }

    Ok(pointer + 1) // We always point to the character that is excluded
}

fn is_ascii_octal(c: char) -> bool {
    c >= '0' && c <= '7'
}

#[cfg(test)]
mod tests {

    use crate::lexer::dollar_single_quotes::tokenize_dollar_single_quotes;
    macro_rules! test {
        ($string:expr, $start:expr, $end:expr) => {{
            assert_eq!(
                $end,
                tokenize_dollar_single_quotes(&String::from($string), $start).unwrap()
            );
        }};
    }
    macro_rules! test_throws {
        ($string:expr, $start:expr, $throws:expr) => {{
            assert_eq!(
                $throws,
                tokenize_dollar_single_quotes(&String::from($string), $start)
                    .unwrap_err()
                    .to_string()
            );
        }};
    }
    // Test tip : Selecting inside the test string from start to end `'` should match with the pointer
    // value that you get returned (length-1 = pointer, your pointer should point 1 char after `'`)
    #[test]
    fn test_dollar_single_quotes_simple() {
        test!(r"$'Hello, World!\nThis is a new line.'", 0, 37);
        test!(r"$'Hello, World!\nThis is a new line.'", 1, 1);
        test!(r" $'Column1\t' is valid", 1, 13);
        test!(r"echo $'This is a backslash: \\' Alrighty", 5, 31);
        test!(r"$'Line1\nLine2\tTabbed'", 0, 23);
        test!(r"$'It'\''s a test'", 0, 5);
        test!(r"$'Hello, $USER!'", 0, 16);
        test!(r"$'Special chars: !@#$%^&*()'", 0, 28);
        test!(r"$'Hello World'", 0, 14);
    }

    #[test]
    fn test_backslash_simple_characters() {
        // \t \n \a etc.
        // To check pointers are correct we can put \c0 immediately after which, if the \ is
        // detected it should throw an error due to c0 thus verifying pointer is correct.
        test_throws!(
            r"$'Hello \t\c0'",
            0,
            r"\c not followed by a Circumflex Control Character at index 12"
        );
        test_throws!(
            r"$'Hello World \n\c0'",
            0,
            r"\c not followed by a Circumflex Control Character at index 18"
        );
        test_throws!(
            r"$'Hello World \n \f \r \c0'",
            0,
            r"\c not followed by a Circumflex Control Character at index 25"
        );
        test_throws!(
            r"$'Hello World \n\f\r\c0'",
            0,
            r"\c not followed by a Circumflex Control Character at index 22"
        );

        test!(r"$'Hello World \n\ca' OK", 0, 20);
        test!(r"$'Hello World \n\f\r'", 0, 21);
    }

    #[test]
    fn test_backslash_control_char() {
        // test a-Z :
        test!(r"$'Hello World \ca' OK", 0, 18);
        test!(r"$'Hello World \cA' OK", 0, 18);
        test!(r"$'Hello World \cW' OK", 0, 18);
        test!(r"$'Hello World \cw' OK", 0, 18);
        test!(r"$'Hello World \cz' OK", 0, 18);
        test!(r"$'Hello World \cZ' OK", 0, 18);
        test_throws!(
            r"$'Hello World \c0' OK",
            0,
            r"\c not followed by a Circumflex Control Character at index 16"
        );
        test_throws!(
            r"$'Hello World \c9' OK",
            0,
            r"\c not followed by a Circumflex Control Character at index 16"
        );
        test_throws!(
            r"$'Hello World \c-' OK",
            0,
            r"\c not followed by a Circumflex Control Character at index 16"
        );

        // Test valid more complex :
        test!(r"$'Hello World \c_' OK", 0, 18);
        test!(r"$'Hello World \c_______' OK", 0, 24);
        test!(r"$'Hello World \c?' OK", 0, 18);
        test!(r"$'Hello World \c??\c?' OK", 0, 22);
        test!(r"$'Hello World \c[\c]' OK", 0, 21);
        test!(r"$'Hello World \c]' OK", 0, 18);
        test!(r"$'Hello World \c[][][]' OK", 0, 23);
        test!(r"$'Hello World \c^\n' OK", 0, 20);
        test!(r"$'Hello World \c^6' OK", 0, 19);
        test!(r"$'Hello World \c^^^^' OK", 0, 21);

        // Test it sets pointer correctly :
        test_throws!(
            r"$'Hello World \ca\c0' OK",
            0,
            r"\c not followed by a Circumflex Control Character at index 19"
        );
        test_throws!(
            r"$'Hello World \c_\c0' OK",
            0,
            r"\c not followed by a Circumflex Control Character at index 19"
        );

        // Test backslash error and valid :
        test_throws!(
            r"$'Hello World \c\' OK",
            0,
            r"\c not followed by a Circumflex Control Character at index 17"
        );
        test_throws!(
            r"$'Hello World \c\\\c0' OK",
            0,
            r"\c not followed by a Circumflex Control Character at index 20"
        );
        test!(r"$'Hello World \c\\ \n \t' OK", 0, 25);
    }

    #[test]
    fn test_hexadecimal() {
        // test invalid ones :
        test_throws!(
            r"$'Hello \xGG' OK",
            0,
            r"\x not followed by a hexadecimal character at index 10"
        );
        test_throws!(
            r"$'Hello \x\n' OK",
            0,
            r"\x not followed by a hexadecimal character at index 10"
        );
        test_throws!(
            r"$'Hello \x' OK",
            0,
            r"\x not followed by a hexadecimal character at index 10"
        );
        test_throws!(
            r"$'Hello \x-1' OK",
            0,
            r"\x not followed by a hexadecimal character at index 10"
        );
        test_throws!(
            r"$'Hello \xH' OK",
            0,
            r"\x not followed by a hexadecimal character at index 10"
        );

        // Test the 2nd one is optional :
        test!(r"$'Hello \xFG' OK", 0, 13);
        test!(r"$'Hello \x9G' OK", 0, 13);
        test!(r"$'Hello \xF1' OK", 0, 13);
        test!(r"$'Hello \x7' OK", 0, 12);
        test!(r"$'Hello \x7 ' OK", 0, 13);
        test!(r"$'Hello \x0' OK", 0, 12);
        test_throws!(
            r"$'Hello \x7\c0' OK",
            0,
            r"\c not followed by a Circumflex Control Character at index 13"
        );
        test_throws!(
            r"$'Hello \x11\c0' OK",
            0,
            r"\c not followed by a Circumflex Control Character at index 14"
        );
        test_throws!(
            r"$'Hello \x1G\c0' OK",
            0,
            r"\c not followed by a Circumflex Control Character at index 14"
        );
    }
}
