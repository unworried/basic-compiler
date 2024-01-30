use crate::{
    lexer::Token,
    parser::{Parse, Parser},
};

#[derive(Debug, PartialEq)]
pub enum Primitive {
    Int(i64),
    // Add more
}

// TODO: Add error handling 

impl<'a> Parse<'a> for Primitive {
    fn parse(parser: &mut Parser<'a>) -> Self {
        let primitive = match &parser.current_token {
            Token::Int(value) => Self::Int(value.parse().unwrap()),
            value => unimplemented!("Unexpected token {:?}", value),
        };

        parser.step();
        primitive
    }
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    String(String),
    // Add more
}

impl<'a> Parse<'a> for Literal {
    fn parse(parser: &mut Parser<'a>) -> Self {
        let literal = match &parser.current_token {
            Token::String(value) => Self::String(value.to_owned()),
            value => unimplemented!("Unexpected token {:?}", value),
        };

        parser.step();
        literal
    }
}
