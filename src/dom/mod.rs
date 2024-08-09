#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Element(Element),
    Text(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Element {
    pub tag_name: String,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<Node>,
}

impl Element {
    pub fn new(tag_name: String) -> Self {
        Element {
            tag_name,
            attributes: Vec::new(),
            children: Vec::new(),            
        }
    }   
}
