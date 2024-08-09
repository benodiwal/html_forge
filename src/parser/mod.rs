use std::string::ParseError;

use crate::dom::Node;

pub struct Parser {
    input: String,
    position: usize
}

impl Parser {
    pub fn new(input: String) -> Self {
        Parser {
            input,
            position: 0,
        }
    }

    fn parse_node(&mut self) -> Result<Node, ParseError> {   
        todo!()
    }
}
