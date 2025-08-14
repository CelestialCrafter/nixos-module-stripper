mod search;

use std::io::{Read, Write, stdin, stdout};

use tree_sitter::Parser;
use tree_sitter_nix::LANGUAGE;

use crate::search::{find_node, is_config_node};

fn new_parser() -> Parser {
    let mut parser = Parser::new();
    parser
        .set_language(&LANGUAGE.into())
        .expect("could not load nix grammar");

    parser
}

fn main() {
    let mut source = Vec::with_capacity(8192);
    stdin()
        .read_to_end(&mut source)
        .expect("could not read from stdin");

    let tree = new_parser()
        .parse(&source, None)
        .expect("could not parse file");

    match find_node(tree.root_node(), |node| is_config_node(node, &source)) {
        Some(config_binding) => {
            source.drain(config_binding.byte_range());
        }
        None => eprintln!("could not find config node (is this a nixos module?)"),
    }

    stdout()
        .write_all(&source)
        .expect("could not write to stdout");
}
