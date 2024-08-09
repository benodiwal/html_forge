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

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    fn consume_while<F>(&mut self, test: F) -> String
    where
       F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.position..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.position += next_pos;
        cur_char
    }

    fn next_char(&self) -> char {
        self.input[self.position..].chars().next().unwrap()
    }

    fn eof(&self) -> bool {
        self.position >= self.input.len()
    }
}
