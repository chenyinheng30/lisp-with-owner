
pub fn from_char(ch: char) -> Option<f64> {
    match ch {
        '0'..='9' => Some(((ch as u8) - ('0' as u8)) as f64),
        'a'..='z' => Some(((ch as u8) - ('a' as u8) + 10) as f64),
        'A'..='Z' => Some(((ch as u8) - ('A' as u8) + 10) as f64),
        _ => None
    }
}

pub fn from_string(str: &str,base: u8) -> Option<f64>{
    if base > 36 && base < 2{
        return None;
    }
    let base = base as f64;
    let mut sum = 0.0;
    for ch in str.chars() {
        sum *= base;
        match from_char(ch) {
            Some(x) if x < base => sum += x,
            _ => return None
        }
    }
    Some(sum)
}

pub fn from_lisp(string: &str) -> Option<f64> {
    let mut itr = string.chars();
    match itr.next() {
        Some('0'..='9') => from_string(string, 10),
        Some('#') =>{
            match itr.next() {
                Some('b' | 'B') => from_string(&itr.as_str(), 2),
                Some('o' | 'O') => from_string(&itr.as_str(), 8),
                Some('x' | 'X') => from_string(&itr.as_str(), 16),
                Some('0'..='9') => {
                    let mut i = 2;
                    let str = loop{
                        match itr.next() {
                            Some('0'..='9') => i += 1,
                            Some('r') => break &string[1..i],
                            _ => return None,
                        }
                    };
                    if let Some(base) = from_string(&str, 10){
                        from_string(&itr.as_str(), base as u8)
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