use crate::parser::{self, NodeType};

pub fn simplify(node: &parser::Node) -> parser::Node {
    if node.node_type == NodeType::Root {
        return simplify(&node.children[0]);
    }

    let mut new_node = parser::Node {
        node_type: node.node_type.clone(),
        children: Vec::new(),
        token: node.token.clone(),
    };

    for child in &node.children {
        new_node.children.push(simplify(child).clone());
    }

    if node.node_type == NodeType::Body {
        if node.children.len() == 1 {
            return simplify(&node.children[0]);
        }
    }

    return new_node;
}
