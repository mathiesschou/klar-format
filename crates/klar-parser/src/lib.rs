mod block;
mod inline;

use block::parse_blocks;
use klar_ast::Document;

pub fn parse(input: &str) -> Document {
    Document {
        blocks: parse_blocks(input),
    }
}
