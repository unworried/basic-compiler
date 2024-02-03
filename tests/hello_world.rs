use flipc::{diagnostics::DiagnosticBag, lexer::Lexer, parser::Parser};

use self::common::read_source_file;

mod common;

//#[test]
fn hello_world() {
    let src = read_source_file("helloworld.fl");
    let mut lex = Lexer::new(src);
    let diagnostics = DiagnosticBag::new();
    let mut parser = Parser::new(&mut lex, diagnostics);

    let actual = parser.parse();
    println!("{:#?}", actual);
}
