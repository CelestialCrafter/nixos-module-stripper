use tree_sitter::Node;

fn binding_identifier_text<'a>(binding: Node, source: &'a [u8]) -> Option<&'a str> {
    let name = binding
        .child_by_field_name("attrpath")?
        .child_by_field_name("attr")?;
    name.utf8_text(source).ok()
}

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

pub fn is_config_node(binding: Node, source: &[u8]) -> bool {
    if binding.kind() != "binding" {
        return false;
    }

    if let Some("config") = binding_identifier_text(binding, source) {
        return false;
    }

    let mut cursor = {
        let binding_set = match binding.parent() {
            Some(v) => v,
            None => return false,
        };
        debug_assert_eq!(binding_set.kind(), "binding_set");

        binding_set.walk()
    };

    cursor.goto_first_child();
    loop {
        if let Some("options") = binding_identifier_text(cursor.node(), source) {
            break true;
        }

        if !cursor.goto_next_sibling() {
            break false;
        }
    }
}
