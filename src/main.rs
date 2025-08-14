mod config;
mod search;

use std::{
    fs,
    io::{Write, stdout},
};

use tree_sitter::Parser;

use crate::{
    config::CONFIG,
    search::{find_node, is_config_node},
};

fn main() {
    let mut source = fs::read(&CONFIG.path).expect("could not read file");

    let mut parser = Parser::new();
    parser
        .set_language(&CONFIG.language)
        .expect("could not load nix grammar");
    let tree = parser.parse(&source, None).expect("could not parse file");

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
