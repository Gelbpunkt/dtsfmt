use tree_sitter::{Parser, Tree};
use tree_sitter_devicetree::language;

pub fn parse(source: String) -> Tree {
    let mut parser = Parser::new();

    parser.set_language(&language()).unwrap();
    return parser.parse(source, None).unwrap();
}
