mod lexer;
mod code;

fn main() {
    let mut first = code::TestCode::new(r#"1234 #b1101 #o770 #xafed #36rabcdefghijklmnopqrstuvwxyz #0002r 123412"#);
    loop{
        match lexer::get_token(&mut first) {
            lexer::Token::EOF => break,
            token @ _ => println!("{:?}",token)
        }
    }
}