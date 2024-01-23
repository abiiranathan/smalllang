// file: src/tokenizer.rs
#[derive(Debug, PartialEq)]
pub(crate) enum TokenType {
    NumberLiteral, // 3, 3, 17
    Identifier,    // a,b,c variableName
    Equal,
    Plus,
    Minus,
    Star,
    Slash,
    LeftParen,
    RightParen,
    NewLine,
}

#[derive(Debug)]
pub(crate) struct Token {
    pub token_type: TokenType,
    pub lexeme: String, // 123 -> lexeme: "123"
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut position = 0;
    let mut result = Vec::new();

    while position < source.len() {
        let current_char: char = source.chars().nth(position).unwrap();
        match current_char {
            '=' => result.push(Token {
                token_type: TokenType::Equal,
                lexeme: "=".to_string(),
            }),
            '+' => result.push(Token {
                token_type: TokenType::Plus,
                lexeme: "+".to_string(),
            }),
            '-' => result.push(Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
            }),
            '*' => result.push(Token {
                token_type: TokenType::Star,
                lexeme: "*".to_string(),
            }),
            '/' => result.push(Token {
                token_type: TokenType::Slash,
                lexeme: "/".to_string(),
            }),
            '(' => result.push(Token {
                token_type: TokenType::LeftParen,
                lexeme: "(".to_string(),
            }),
            ')' => result.push(Token {
                token_type: TokenType::RightParen,
                lexeme: "(".to_string(),
            }),
            '\n' => result.push(Token {
                token_type: TokenType::NewLine,
                lexeme: "\n".to_string(),
            }),
            x if x.is_digit(10) => {
                let mut number_lexeme = x.to_string();
                position += 1;

                while position < source.len() {
                    let next_char = source.chars().nth(position).unwrap();
                    if next_char == ' ' || next_char == ')' || next_char == '\n' {
                        break;
                    }

                    if next_char.is_digit(10) {
                        number_lexeme.push(next_char);
                    } else {
                        panic!("Invalid character: '{}'", next_char);
                    }
                    position += 1;
                }

                result.push(Token {
                    token_type: TokenType::NumberLiteral,
                    lexeme: number_lexeme,
                });
                continue; // we don't want to consume the last char /after/ the number
            }
            ' ' => {}
            c => {
                //Assume Ident.
                let mut lexeme = c.to_string();
                position += 1;

                while position < source.len() {
                    let next_char = source.chars().nth(position).unwrap();
                    if !is_valid_identifier_char(next_char) {
                        break;
                    }

                    lexeme.push(next_char);
                    position += 1;
                }
                result.push(Token {
                    token_type: TokenType::Identifier,
                    lexeme,
                });
                continue;
            }
        }

        position += 1;
    }

    return result;
}

fn is_valid_identifier_char(ch: char) -> bool {
    return ch.is_alphanumeric() || ch == '_';
}
