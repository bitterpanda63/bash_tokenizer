macro_rules! increment_pointer {
    ($pointer:expr, $content:expr, $start:expr, $char:expr) => {
        {
            $pointer += 1;
            if $pointer >= $content.len() {
                return Err(format!("Unterminated dollar-single-quote at index {}", $start).into());
            }
            $char = $content.chars().nth($pointer).unwrap();

        }
    };
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
    if char != '$' || !(pointer+1 < content.len()) {
        // Check it's a dollar single quoted string and that we can fetch the following character.
        return Ok(start);
    }
    pointer += 1; // Skipped `$`
    let mut char = content.chars().nth(pointer).unwrap();
    if char != '\'' {
        // A dollar-single-quote always starts with `$'`
        return Ok(start);
    }

    while char != '\'' || pointer == start+2 { // Add 2 for the offset of `$'`
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
                return Err(format!("\\c not followed by a Circumflex Control Character at index {}", pointer).into());
            }

            // \xXX yields the byte whose value is the hexadecimal value XX (one or more hexadecimal digits) :
            if char == 'x' {
                increment_pointer!(pointer, content, start, char);
                if !char.is_ascii_hexdigit() {
                    return Err(format!("\\x not followed by a hexadecimal character at index {}", pointer).into());
                }
                increment_pointer!(pointer, content, start, char);
                if !char.is_ascii_hexdigit() {
                    pointer -= 1 ; // Reset pointer here.
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
                pointer -= 1; // Go back one character (pointer should end on octal)
                continue;
            }

            // Bash-specific : \E, \?, \uHHHH, \UHHHHHHHH currently not supported.

            // Backslash matched nothing :
            return Err(format!("\\ followed by an invalid character at index {}", pointer).into());

        }
    }

    Ok(pointer)
}

fn is_ascii_octal(c: char) -> bool {
    c >= '0' && c <= '7'
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_dollar_single_quotes() {

    }
}