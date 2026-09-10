use std::fmt;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    LParen,
    RParen,
}

#[derive(Debug)]
pub struct CalcError(pub String);

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn tokenize(input: &str) -> Result<Vec<Token>, CalcError> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        if c.is_whitespace() {
            i += 1;
            continue;
        }

        match c {
            '+' => {
                tokens.push(Token::Plus);
                i += 1;
            }
            '-' => {
                tokens.push(Token::Minus);
                i += 1;
            }
            '*' => {
                tokens.push(Token::Star);
                i += 1;
            }
            '/' => {
                tokens.push(Token::Slash);
                i += 1;
            }
            '%' => {
                tokens.push(Token::Percent);
                i += 1;
            }
            '^' => {
                tokens.push(Token::Caret);
                i += 1;
            }
            '(' => {
                tokens.push(Token::LParen);
                i += 1;
            }
            ')' => {
                tokens.push(Token::RParen);
                i += 1;
            }
            c if c.is_ascii_digit() || c == '.' => {
                let start = i;
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    i += 1;
                }
                let text: String = chars[start..i].iter().collect();
                let value: f64 = text
                    .parse()
                    .map_err(|_| CalcError(format!("invalid number '{text}'")))?;
                tokens.push(Token::Number(value));
            }
            other => {
                return Err(CalcError(format!("unexpected character '{other}'")));
            }
        }
    }

    Ok(tokens)
}

/// Recursive-descent parser/evaluator following standard precedence:
/// unary (+/-) > power (^, right-assoc) > mul/div/mod > add/sub
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<Token> {
        let tok = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        tok
    }

    fn parse_expr(&mut self) -> Result<f64, CalcError> {
        let mut value = self.parse_term()?;
        loop {
            match self.peek() {
                Some(Token::Plus) => {
                    self.next();
                    value += self.parse_term()?;
                }
                Some(Token::Minus) => {
                    self.next();
                    value -= self.parse_term()?;
                }
                _ => break,
            }
        }
        Ok(value)
    }

    fn parse_term(&mut self) -> Result<f64, CalcError> {
        let mut value = self.parse_power()?;
        loop {
            match self.peek() {
                Some(Token::Star) => {
                    self.next();
                    value *= self.parse_power()?;
                }
                Some(Token::Slash) => {
                    self.next();
                    let rhs = self.parse_power()?;
                    if rhs == 0.0 {
                        return Err(CalcError("division by zero".to_string()));
                    }
                    value /= rhs;
                }
                Some(Token::Percent) => {
                    self.next();
                    let rhs = self.parse_power()?;
                    if rhs == 0.0 {
                        return Err(CalcError("division by zero".to_string()));
                    }
                    value %= rhs;
                }
                _ => break,
            }
        }
        Ok(value)
    }

    fn parse_power(&mut self) -> Result<f64, CalcError> {
        let base = self.parse_unary()?;
        if let Some(Token::Caret) = self.peek() {
            self.next();
            // right-associative: 2^3^2 == 2^(3^2)
            let exponent = self.parse_power()?;
            return Ok(base.powf(exponent));
        }
        Ok(base)
    }

    fn parse_unary(&mut self) -> Result<f64, CalcError> {
        match self.peek() {
            Some(Token::Minus) => {
                self.next();
                Ok(-self.parse_unary()?)
            }
            Some(Token::Plus) => {
                self.next();
                self.parse_unary()
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<f64, CalcError> {
        match self.next() {
            Some(Token::Number(n)) => Ok(n),
            Some(Token::LParen) => {
                let value = self.parse_expr()?;
                match self.next() {
                    Some(Token::RParen) => Ok(value),
                    _ => Err(CalcError("expected ')'".to_string())),
                }
            }
            Some(other) => Err(CalcError(format!("unexpected token '{other:?}'"))),
            None => Err(CalcError("unexpected end of expression".to_string())),
        }
    }
}

/// Evaluates a math expression string and returns the numeric result.
///
/// Supports `+ - * / % ^`, parentheses, unary minus/plus, and decimals,
/// with standard operator precedence.
pub fn evaluate(input: &str) -> Result<f64, CalcError> {
    let tokens = tokenize(input)?;
    if tokens.is_empty() {
        return Err(CalcError("empty expression".to_string()));
    }
    let mut parser = Parser::new(tokens);
    let result = parser.parse_expr()?;
    if parser.pos != parser.tokens.len() {
        return Err(CalcError("unexpected trailing input".to_string()));
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eval(s: &str) -> f64 {
        evaluate(s).unwrap()
    }

    #[test]
    fn basic_ops() {
        assert_eq!(eval("2 + 3"), 5.0);
        assert_eq!(eval("10 - 4"), 6.0);
        assert_eq!(eval("6 * 7"), 42.0);
        assert_eq!(eval("8 / 2"), 4.0);
        assert_eq!(eval("7 % 3"), 1.0);
    }

    #[test]
    fn precedence() {
        assert_eq!(eval("2 + 3 * 4"), 14.0);
        assert_eq!(eval("(2 + 3) * 4"), 20.0);
        assert_eq!(eval("2 * 3 + 4 * 5"), 26.0);
    }

    #[test]
    fn power_right_assoc() {
        assert_eq!(eval("2 ^ 3"), 8.0);
        assert_eq!(eval("2 ^ 3 ^ 2"), 512.0); // 2^(3^2), not (2^3)^2
    }

    #[test]
    fn unary_minus() {
        assert_eq!(eval("-5 + 3"), -2.0);
        assert_eq!(eval("-(2 + 3)"), -5.0);
        assert_eq!(eval("3 - -2"), 5.0);
    }

    #[test]
    fn decimals() {
        assert_eq!(eval("1.5 + 2.25"), 3.75);
    }

    #[test]
    fn division_by_zero() {
        assert!(evaluate("1 / 0").is_err());
    }

    #[test]
    fn invalid_syntax() {
        assert!(evaluate("2 + ").is_err());
        assert!(evaluate("(2 + 3").is_err());
        assert!(evaluate("2 + + ").is_err());
        assert!(evaluate("2 3").is_err());
    }
}
