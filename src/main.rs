trait Code {
    fn eat(&mut self) -> Option<char>;
    fn head(&self) -> Option<char>;
}

struct Lexer<T: Code>{
    code: T,
}

impl<T: Code> Lexer<T> {
    fn new(code: T) -> Self{
        Lexer{code}
    }

    fn next(&mut self) -> Snippet{
        match self.code.head() {
            Some(c) => match c {
                ' ' => {
                    while let Some(' ') = self.code.eat(){}
                    self.next()
                }

                sign @ ('(' | ')') => {
                    self.code.eat();
                    Snippet::Sign(sign)
                },

                '"' => {
                    let mut str = String::new();
                    while let Some(c) = self.code.eat() {
                        if c == '"' {
                            break;
                        }else{
                            str.push(c);
                        }
                    }
                    self.code.eat();
                    Snippet::String(str)
                }

                ch @ _ => {
                    let mut str = String::from(ch);
                    while let Some(c) = self.code.eat() {
                        match c {
                            ' ' | ')' | '(' => break,
                            _ => str.push(c)
                        }
                    }
                    Snippet::Other(str)
                }
            }

            None => Snippet::EOF
        }
    }
}

#[derive(Debug)]
enum Snippet{
    String(String),
    Other(String),
    Sign(char),
    EOF
}

fn as_number_char(ch: char) -> Option<f64> {
    match ch {
        '0'..='9' => Some(((ch as u8) - ('0' as u8)) as f64),
        // 'a'..='z' => Some(((ch as u8) - ('a' as u8)) as f64),
        // 'A'..='Z' => Some(((ch as u8) - ('A' as u8)) as f64),
        _ => None
    }
}

fn as_number_string(str: &String) -> Option<f64> {
    let mut sum = 0.0;
    let mut i = 1.0;
    for x in str.chars().rev() {
        match as_number_char(x) {
            Some(x) => sum += x * i,
            None => return None
        }
        i *= 10.0;
    }
    Some(sum)
}

impl Snippet {
    fn token(self) -> Token {
        match self{
            Self::Sign(ch) => Token::Sign(ch),
            Self::String(str) => Token::String(str),
            Self::Other(str) => {
                match as_number_string(&str) {
                    Some(x) => Token::Number(x),
                    None => Token::Atom(str)
                }
            }
            Self::EOF => Token::EOF
        }
    }
}

#[derive(Debug)]
enum Token{
    Number(f64),
    String(String),
    Atom(String),
    Sign(char),
    EOF,
}

struct TestCode{
    code: String,
    head: usize,
}
impl TestCode{
    fn new(code: &str) -> Self{
        let mut code = code.to_string();
        let mut str = String::new();
        while let Some(c) =code.pop() {
            str.push(c);
        }
        TestCode { head: str.len() ,code: str}
    }
}

impl Code for TestCode{
    fn eat(&mut self) -> Option<char> {
        self.head -= 1;
        self.code = String::from(&self.code[..self.head]);
        self.head()
    }

    fn head(&self) -> Option<char> {
        self.code.chars().last()
    }
}

fn main() {
    let first = TestCode::new(r#"(abc (1234 224(1234(1234 asf qfq)("12(3      4)1") "1432" )) )"#);
    let mut lexer = Lexer::new(first);
    loop{
        match lexer.next() {
            Snippet::EOF => break,
            snippet @ _ => println!("{:?}",snippet.token())
        }
    }
}

