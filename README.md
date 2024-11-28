# Parser Combinators in Rust

This project is a Rust implementation of a mini XML parser using parser combinators, inspired by [Bodil Stokke's guide](https://bodil.lol/parser-combinators/).

## Installation
1. Clone the repository:
```bash
git clone https://github.com/truthixify/mini-xml-parser.git
cd parser-combinators
```

2. Build the project:
```bash
cargo build
```

3. Run the tests:
```bash
cargo test
```

## Usage
You can parse an example XML file using:
```bash
cargo run -- -file example.xml
```

The repository includes parsers for parsing simple XML file with no namespaces, text node and escape quotes.

## Learning Resources
- Original Article: [Bodil's guide](https://bodil.lol/parser-combinators/) explains the concepts in detail.
- Rust Documentation: Learn about Rust's functional programming features and iterators [here](https://doc.rust-lang.org/book/).

## Contributing

Contributions are welcome! If you have ideas, bug fixes, or improvements, feel free to fork this repository and submit a pull request.