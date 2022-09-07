
pub trait Code {
    fn eat(&mut self) -> Option<char>;
    fn head(&self) -> Option<char>;
}

pub struct TestCode{
    code: String,
    head: usize,
}
impl TestCode{
    pub fn new(code: &str) -> Self{
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
