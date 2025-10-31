//viper/src/main.rs
use viper_lexer::lexer::Lexer;
fn main() {
    let programm: String = String::from("let a = 11;");

    let mut lexer = Lexer::new(programm);

    println!("{:#?}", lexer.tokenize());
}
