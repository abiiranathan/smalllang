use crate::tokenize::{Token, TokenType};

pub struct BinaryOpImpl {
    pub lhs: Box<Expr>,
    pub operation: Token,
    pub rhs: Box<Expr>,
}

pub struct NumberImpl {
    pub value: i32,
    pub token: Token,
}

pub struct VariableImpl {
    pub name: Token,
}
pub struct AssignmentImpl {
    pub target: VariableImpl,
    pub value: Box<Expr>,
}

pub struct FunCallImpl {
    pub name: VariableImpl,
    pub arg: Box<Expr>,
}

pub enum Expr {
    Assignment(AssignmentImpl),
    BinaryOperation(BinaryOpImpl),
    FunCall(FunCallImpl),
    Number(NumberImpl),
    Variable(VariableImpl),
}

pub fn parse(mut tokens: Vec<Token>) -> Vec<Expr> {
    tokens.reverse();

    let mut result = Vec::new();

    // Recursive descent parser
    while tokens.len() > 0 {
        let expr = parse_expr(&mut tokens);
        expect(TokenType::NewLine, &mut tokens);
        result.push(expr);
    }

    return result;
}

fn expect(token_type: TokenType, tokens: &mut Vec<Token>) {
    match tokens.pop() {
        None => {} // EOF
        Some(token) => {
            if token.token_type != token_type {
                panic!("Expected token {:?} but got {:?}", token_type, token);
            }
        }
    }
}

fn parse_factor(tokens: &mut Vec<Token>) -> Expr {
    let mut lhs = parse_primary(tokens);

    while tokens.len() > 1 {
        let next_token = &tokens[tokens.len() - 1];
        match next_token.token_type {
            TokenType::Star | TokenType::Slash => {
                let op_token = tokens.pop().unwrap();
                let rhs = parse_primary(tokens);
                lhs = Expr::BinaryOperation(BinaryOpImpl {
                    lhs: Box::new(lhs),
                    operation: op_token,
                    rhs: Box::new(rhs),
                });
            }
            _ => break,
        }
    }
    return lhs;
}

fn parse_primary(tokens: &mut Vec<Token>) -> Expr {
    let token = tokens.pop().unwrap();
    match token.token_type {
        TokenType::NumberLiteral => {
            return Expr::Number(NumberImpl {
                value: parse_number(&token.lexeme),
                token,
            });
        }
        TokenType::Identifier => {
            if tokens.len() > 0 {
                // It might be a function call
                let next_token = &tokens[tokens.len() - 1];
                if next_token.token_type == TokenType::LeftParen {
                    let fun_name = VariableImpl { name: token };

                    // Remove left paren
                    tokens.pop().unwrap();
                    let arg = parse_expr(tokens);
                    expect(TokenType::RightParen, tokens);
                    return Expr::FunCall(FunCallImpl {
                        name: fun_name,
                        arg: Box::new(arg),
                    });
                }
            }
            return Expr::Variable(VariableImpl { name: token });
        }
        TokenType::LeftParen => {
            let expr = parse_expr(tokens);
            expect(TokenType::RightParen, tokens);
            return expr;
        }
        _ => panic!("Unexpected token {:?}", token),
    }
}

fn parse_number(lexeme: &str) -> i32 {
    return lexeme.parse::<i32>().unwrap();
}

fn parse_expr(tokens: &mut Vec<Token>) -> Expr {
    return parse_assignment(tokens);
}

fn parse_assignment(tokens: &mut Vec<Token>) -> Expr {
    // look ahead
    if tokens.len() > 1 && &tokens[tokens.len() - 2].token_type == &TokenType::Equal {
        let target = parse_variable(tokens);
        expect(TokenType::Equal, tokens);
        let value = Box::new(parse_expr(tokens));
        return Expr::Assignment(AssignmentImpl { target, value });
    } else {
        return parse_term(tokens);
    }
}

fn parse_variable(tokens: &mut Vec<Token>) -> VariableImpl {
    let token = tokens.pop().unwrap();
    if token.token_type != TokenType::Identifier {
        panic!("Expected identifier but got {:?}", token);
    }
    return VariableImpl { name: token };
}

fn parse_term(tokens: &mut Vec<Token>) -> Expr {
    let mut lhs = parse_factor(tokens);

    while tokens.len() > 0 {
        let next_token = &tokens[tokens.len() - 1];
        match next_token.token_type {
            TokenType::Plus | TokenType::Minus => {
                let op_token = tokens.pop().unwrap();
                let rhs = parse_factor(tokens);
                lhs = Expr::BinaryOperation(BinaryOpImpl {
                    lhs: Box::new(lhs),
                    operation: op_token,
                    rhs: Box::new(rhs),
                });
            }
            _ => break,
        }
    }
    return lhs;
}
