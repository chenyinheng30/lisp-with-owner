use crate::code;
use math::as_number;

#[derive(Debug)]
pub enum Token{
    Number(f64),
    String(String),
    Atom(String),
    Sign(char),
    EOF,
}

pub fn get_token<T>(code: &mut T) -> Token
    where T: code::Code
{
    match code.head() {
        Some(c) => match c {
            ' ' => {
                while let Some(' ') = code.eat(){}
                get_token(code)
            }
            sign @ ('(' | ')' | '\'' | '`') => {
                code.eat();
                Token::Sign(sign)
            },
            '"' => {
                let mut str = String::new();
                while let Some(c) = code.eat() {
                    if c == '"' {
                        break;
                    }else{
                        str.push(c);
                    }
                }
                code.eat();
                Token::String(str)
            }
            ch @ _ => {
                let mut str = String::from(ch);
                if let ch @ '#' = ch{
                    match code.eat() {
                        Some(' ' | '(' | ')' | '\'' | '`') => return Token::Sign(ch),
                        Some(ch @ _) => str.push(ch),
                        _ => return Token::Sign(ch)
                    }
                }
                while let Some(c) = code.eat() {
                    match c {
                        ' ' | ')' | '(' => break,
                        _ => str.push(c)
                    }
                }
                match as_number::from_lisp(&str) {
                    Some(x) => Token::Number(x),
                    None => Token::Atom(str)
                }
            }
        }
        None => Token::EOF
    }
}
