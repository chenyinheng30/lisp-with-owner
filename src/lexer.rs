use crate::code;

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
                match as_number_lisp(&str) {
                    Some(x) => Token::Number(x),
                    None => Token::Atom(str)
                }
            }
        }
        None => Token::EOF
    }
}

fn as_number_char(ch: char) -> Option<f64> {
    match ch {
        '0'..='9' => Some(((ch as u8) - ('0' as u8)) as f64),
        'a'..='z' => Some(((ch as u8) - ('a' as u8) + 10) as f64),
        'A'..='Z' => Some(((ch as u8) - ('A' as u8) + 10) as f64),
        _ => None
    }
}

fn as_number_string(str: &str,base: u8) -> Option<f64>{
    if base > 36 && base < 2{
        return None;
    }
    let base = base as f64;
    let mut sum = 0.0;
    for ch in str.chars() {
        sum *= base;
        match as_number_char(ch) {
            Some(x) if x < base => sum += x,
            _ => return None
        }
    }
    Some(sum)
}

fn as_number_lisp(string: &str) -> Option<f64> {
    let mut itr = string.chars();
    match itr.next() {
        Some('0'..='9') => as_number_string(string, 10),
        Some('#') =>{
            match itr.next() {
                Some('b' | 'B') => as_number_string(&itr.as_str(), 2),
                Some('o' | 'O') => as_number_string(&itr.as_str(), 8),
                Some('x' | 'X') => as_number_string(&itr.as_str(), 16),
                Some('0'..='9') => {
                    let mut i = 2;
                    let str = loop{
                        match itr.next() {
                            Some('0'..='9') => i += 1,
                            Some('r') => break &string[1..i],
                            _ => return None,
                        }
                    };
                    if let Some(base) = as_number_string(&str, 10){
                        as_number_string(&itr.as_str(), base as u8)
                    }else {
                        None
                    }
                }
                _ => None
            }
        }
        _ => None
    }
}