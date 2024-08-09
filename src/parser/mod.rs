use crate::{dom::{Element, Node}, errors::ParseError};

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

    pub fn parse(&mut self) -> Result<Node, ParseError> {
        self.parse_node()
    }

    fn parse_node(&mut self) -> Result<Node, ParseError> {   
        self.consume_whitespace();
        if self.starts_with("<") {
            self.parse_element()
        } else {
            self.parse_text()
        }
    }

    fn parse_element(&mut self) -> Result<Node, ParseError> {
        self.consume_char();
        let tag_name = self.parse_tag_name();
        let attributes = self.parse_attributes();
        self.consume_char();

        let mut element = Element::new(tag_name.clone());
        element.attributes = attributes;

        loop {
            self.consume_whitespace();
            if self.starts_with("</") {
                break;
            }
            let child = self.parse_node()?;
            element.children.push(child);
        }
        self.consume_char();
        self.consume_char();
        let closing_tag_name = self.parse_tag_name();
        if closing_tag_name != tag_name {
            return Err(ParseError::MismatchedClosingTag);
        }
        self.consume_char();

        Ok(Node::Element(element))
    }

    fn parse_text(&mut self) -> Result<Node, ParseError> {
        let text = self.consume_while(|c| c != '<');
        Ok(Node::Text(text))
    }

    fn parse_tag_name(&mut self) -> String {
        self.consume_while(|c| c.is_alphanumeric())
    }

    fn parse_attributes(&mut self) -> Vec<(String, String)> {
        let mut attributes = Vec::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let name = self.parse_tag_name();
            self.consume_char();
            let value = self.parse_attribute_value();
            attributes.push((name, value));
        }
        attributes
    }

    fn parse_attribute_value(&mut self) -> String {
        let quote = self.consume_char();
        if quote != '"' && quote != '\'' {
            panic!("Invalid attribute value")
        }
        let value = self.consume_while(|c| c != quote);
        self.consume_char();
        value
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


    fn starts_with(&self, s: &str) -> bool {
        self.input[self.position..].starts_with(s)
    }

    fn eof(&self) -> bool {
        self.position >= self.input.len()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::dom::Node;
    use crate::errors::ParseError;

    #[test]
    fn test_parse_valid_single_element() {
        let input = "<div></div>".to_string();
        let mut parser = Parser::new(input);
        
        match parser.parse() {
            Ok(Node::Element(element)) => {
                assert_eq!(element.tag_name, "div");
                assert!(element.children.is_empty());
            }
            _ => panic!("Expected a valid div element"),
        }
    }

    #[test]
    fn test_parse_valid_nested_elements() {
        let input = "<div><p></p></div>".to_string();
        let mut parser = Parser::new(input);
        
        match parser.parse() {
            Ok(Node::Element(element)) => {
                assert_eq!(element.tag_name, "div");
                assert_eq!(element.children.len(), 1);
                if let Node::Element(child) = &element.children[0] {
                    assert_eq!(child.tag_name, "p");
                    assert!(child.children.is_empty());
                } else {
                    panic!("Expected a p element inside div");
                }
            }
            _ => panic!("Expected a valid nested element structure"),
        }
    }

    #[test]
    fn test_parse_text_node() {
        let input = "Hello, world!".to_string();
        let mut parser = Parser::new(input);
        
        match parser.parse() {
            Ok(Node::Text(text)) => {
                assert_eq!(text, "Hello, world!");
            }
            _ => panic!("Expected a text node"),
        }
    }

    #[test]
    fn test_mismatched_closing_tag() {
        let input = "<div><p></div></p>".to_string();
        let mut parser = Parser::new(input);
        
        match parser.parse() {
            Err(ParseError::MismatchedClosingTag) => (),
            _ => panic!("Expected MismatchedClosingTag error"),
        }
    }

    #[test]
    fn test_empty_input() {
        let input = "".to_string();
        let mut parser = Parser::new(input);
        
        match parser.parse() {
            Err(ParseError::UnexpectedEOF) => (),
            _ => panic!("Expected UnexpectedEOF error"),
        }
    }

    #[test]
    fn test_parse_element_with_attributes() {
        let input = r#"<div class="container" id='main'></div>"#.to_string();
        let mut parser = Parser::new(input);
        
        match parser.parse() {
            Ok(Node::Element(element)) => {
                assert_eq!(element.tag_name, "div");
                assert_eq!(element.attributes.len(), 2);
                assert_eq!(element.attributes[0], ("class".to_string(), "container".to_string()));
                assert_eq!(element.attributes[1], ("id".to_string(), "main".to_string()));
            }
            _ => panic!("Expected a div element with attributes"),
        }
    }

    #[test]
    fn test_parse_self_closing_element() {
        let input = "<img src='image.jpg' />".to_string();
        let mut parser = Parser::new(input);
        
        match parser.parse() {
            Ok(Node::Element(element)) => {
                assert_eq!(element.tag_name, "img");
                assert_eq!(element.attributes.len(), 1);
                assert_eq!(element.attributes[0], ("src".to_string(), "image.jpg".to_string()));
                assert!(element.children.is_empty());
            }
            _ => panic!("Expected a self-closing img element"),
        }
    }
}
