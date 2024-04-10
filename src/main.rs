mod parser;
mod tests;
use std::fs::File;
use std::io::Read;
fn main() {
    let expr = "( ( 1 + 2 ) * 3 / 4 ) ^ 5";
    let parser = parser::Parser::new(expr.to_string()).unwrap();
    let result = parser.eval().unwrap();
    println!("prefix: {}, postfix: {}, eval: {}", parser.get_expr(), parser.get_postfix(), result);
}
