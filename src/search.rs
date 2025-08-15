use tree_sitter::Node;

pub fn find_node<'a>(root: Node<'a>, mut predicate: impl FnMut(Node) -> bool) -> Option<Node<'a>> {
    let mut cursor = root.walk();

    'outer: loop {
        let node = cursor.node();

        // we're at our desired node. search succeded
        if predicate(node) {
            break Some(node);
        }

        // try to go deeper into the child
        if cursor.goto_first_child() {
            continue;
        }

        // else, go to the sibling if we can
        if cursor.goto_next_sibling() {
            continue;
        }

        // else, climb back up till we can go sideways again
        while cursor.goto_parent() {
            // we're at the root node. search failed
            if cursor.node() == root {
                break 'outer None;
            }

            if cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

pub fn find_config_node<'a>(root: Node<'a>, source: &[u8]) -> Option<Node<'a>> {
    find_node(root, |n| explicit_config_predicate(n, source))
        .or_else(|| find_node(root, implicit_config_predicate))
}

fn binding_identifier_text<'a>(binding: Node, source: &'a [u8]) -> Option<&'a str> {
    let name = binding
        .child_by_field_name("attrpath")?
        .child_by_field_name("attr")?;
    name.utf8_text(source).ok()
}

// WARN: fragile
pub fn implicit_config_predicate(binding: Node) -> bool {
    binding.kind() == "binding_set"
}

pub fn explicit_config_predicate(binding: Node, source: &[u8]) -> bool {
    if binding.kind() != "binding" {
        return false;
    }

    matches!(binding_identifier_text(binding, source), Some("config"))
}

#[cfg(test)]
mod tests {
    use crate::new_parser;
    use paste::paste;

    use super::*;

    fn run_test_case(source: &[u8]) {
        let tree = new_parser()
            .parse(source, None)
            .expect("could not parse file");

        find_config_node(tree.root_node(), source).expect("could not find node");
    }

    macro_rules! generate_test_case {
        ($case:expr) => {
            paste! {
                #[test]
                fn [<test_search_ $case>]() {
                    let source = include_bytes!(
                        concat!(env!("CARGO_MANIFEST_DIR"), "/test-cases/", $case, ".nix")
                    );
                    run_test_case(source);
                }
            }
        };
    }

    generate_test_case!("1");
    generate_test_case!("2");
    generate_test_case!("3");
    generate_test_case!("4");
    generate_test_case!("5");
    generate_test_case!("6");
    generate_test_case!("7");
    generate_test_case!("8");
    generate_test_case!("9");
    generate_test_case!("10");
}
