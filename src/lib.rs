#[derive(Debug, PartialEq, PartialOrd)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Token {
    Number(u32),
    Op(Operator),
    Bracket(char),
}

#[derive(Debug)]
pub enum CalcError {
    BadToken(char),
    MismatchedParens,
}

pub struct Calculator {}

impl Calculator {
    pub fn parse<T: AsRef<str>>(expr: T) -> Result<Vec<Token>, CalcError> {
        let expr = expr.as_ref();

        let chars = expr.chars();

        const RADIX: u32 = 10;

        let mut tokens: Vec<Token> = Vec::new();

        let mut parens: Vec<char> = Vec::new();

        for c in chars {
            match c {
                '0'..='9' => match tokens.last_mut() {
                    Some(Token::Number(n)) => {
                        let digit = c.to_digit(RADIX).unwrap();
                        // convert number to base 10 and digit
                        *n = *n * RADIX + digit;
                    }
                    _ => {
                        let digit = c.to_digit(RADIX).unwrap();
                        tokens.push(Token::Number(digit));
                    }
                },
                '(' => {
                    tokens.push(Token::Bracket('('));
                    parens.push(c);
                }
                ')' => {
                    tokens.push(Token::Bracket(')'));

                    if let Some(p) = parens.pop() {
                        if p != '(' {
                            return Err(CalcError::MismatchedParens);
                        }
                    } else {
                        return Err(CalcError::MismatchedParens);
                    }
                }
                '+' => tokens.push(Token::Op(Operator::Add)),
                '-' => tokens.push(Token::Op(Operator::Sub)),
                '*' => tokens.push(Token::Op(Operator::Mul)),
                '/' => tokens.push(Token::Op(Operator::Div)),
                ' ' => {}
                '\n' => {}

                _ => return Err(CalcError::BadToken(c)),
            }
        }

        if parens.len() > 0 {
            return Err(CalcError::MismatchedParens);
        }

        Ok(tokens)
    }

    pub fn expression(mut tokens: Vec<Token>) -> Vec<Token> {
        tokens.reverse();

        let mut queue: Vec<Token> = Vec::new();

        let mut stack: Vec<Token> = Vec::new();

        while let Some(token) = tokens.pop() {
            match token {
                Token::Number(_) => queue.push(token),
                Token::Op(_) => {
                    while !stack.is_empty() && *stack.last().unwrap() >= token {
                        queue.push(stack.pop().unwrap());
                    }
                    stack.push(token);
                }
                Token::Bracket('(') => stack.push(token),
                Token::Bracket(')') => {
                    while !stack.is_empty() && *stack.last().unwrap() != Token::Bracket('(') {
                        queue.push(stack.pop().unwrap());
                    }
                    stack.pop();
                }
                _ => {}
            }
        }

        while stack.len() > 0 {
            queue.push(stack.pop().unwrap());
        }

        queue
    }

    pub fn evaluate(mut tokens: Vec<Token>) -> Option<f32> {
        tokens.reverse();

        let mut stack: Vec<f32> = Vec::new();
        while let Some(token) = tokens.pop() {
            match token {
                Token::Number(num) => stack.push(num as f32),
                Token::Op(Operator::Add) => {
                    let left = stack.pop().unwrap();
                    let right = stack.pop().unwrap();

                    stack.push(left + right)
                }
                Token::Op(Operator::Sub) => {
                    let left = stack.pop().unwrap();
                    let right = stack.pop().unwrap();

                    stack.push(left - right)
                }
                Token::Op(Operator::Mul) => {
                    let left = stack.pop().unwrap();
                    let right = stack.pop().unwrap();

                    stack.push(left * right)
                }
                Token::Op(Operator::Div) => {
                    let left = stack.pop().unwrap();
                    let right = stack.pop().unwrap();

                    stack.push(left / right)
                }
                _ => {}
            }
        }

        if stack.len() > 1 {
            None
        } else {
            stack.pop()
        }
    }
}
