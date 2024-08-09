# html_forge

A robust and efficient HTML parsing library for Rust

## 🚀 Features
- `Element Parsing`: Easily parse HTML elements with attributes and nested children.
- `Text Parsing`: Extract and manage text nodes within your documents.
- `Attribute Parsing`: Handle element attributes with both single and double quotes.
- `Comment Parsing`: Parse comments safely, even those with special characters or nested hyphens.
- `Error Handling`: Gracefully manage parsing errors such as mismatched tags and unexpected end-of-file (EOF).

## 📦 Installation

To use `html_forge`, add the following to your Cargo.toml:

```toml
[dependencies]
html_forge = "0.1.0"
```

## Usage

### 1. Basic Parsing Example:

Here’s how you can parse a simple HTML snippet using html_forge:

```rust
use html_forge::parser::Parser;
use html_forge::dom::Node;

fn main() {
    let input = "<div class='container'><p>Hello, world!</p></div>".to_string();
    let mut parser = Parser::new(input);
    
    match parser.parse() {
        Ok(node) => println!("Parsed Node: {:?}", node),
        Err(err) => eprintln!("Parsing error: {:?}", err),
    }
}
```

### 2.  Handling Errors:
html_forge gracefully handles common errors during parsing:

```rust
use html_forge::{parser::Parser, errors::ParseError};

fn main() {
    let input = "<div><p>Unclosed div".to_string();
    let mut parser = Parser::new(input);

    match parser.parse() {
        Err(ParseError::UnexpectedEOF) => eprintln!("Error: Unexpected end of file"),
        Err(ParseError::MismatchedClosingTag) => eprintln!("Error: Mismatched closing tag"),
        Ok(node) => println!("Parsed Node: {:?}", node),
        Err(err) => eprintln!("Other parsing error: {:?}", err),
    }
}
```

## 🧪 Testing
To run the tests, use:

```bash
cargo test
```

## 👥 Contributing
Contributions are welcome! Feel free to open issues, submit pull requests, or fork the repository to make improvements.

## 📝 License
This library is open-source and available under the [MIT LICENSE](LICENSE).

`Happy forging with html_forge! 🛠️🚀`
