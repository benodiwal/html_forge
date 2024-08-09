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
}
