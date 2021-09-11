use std::iter::IntoIterator;

/// Represents a node in a parse tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    kind: String,
    value: String,
    children: Vec<Node> 
}

impl Node {
    /// Yields the immediate children of this node.
    pub fn children(&self) -> impl Iterator<Item = &Node> {
        self.children.iter()
    }

    /// Returns a string slice representation of what kind of value this node contains (e.g. comma, identifier, keyword, number, string, etc.)
    pub fn kind(&self) -> &str {
        &self.kind
    }

    /// Returns a new Node with no children
    /// 
    /// Use `Node::new()` if the node should have children.
    /// 
    /// # Arguments
    /// * `kind`     - an `AsRef<str>` containing what kind of value this node contains (e.g. comma, identifier, keyword, number, string, etc.)
    /// * `value`    - an `Into<String>` containing the contents of the node
    pub fn leaf<A: AsRef<str>, B: Into<String>>(kind: A, value: B) -> Self {
        Node {
            kind: kind.as_ref().to_string(),
            value: value.into(),
            children: vec![]
        }
    }

    /// Returns a new Node
    /// 
    /// # Arguments
    /// * `kind`     - an `AsRef<str>` containing what kind of value this node contains (e.g. comma, identifier, keyword, number, string, etc.)
    /// * `value`    - an `Into<String>` containing the contents of the node
    /// * `children` - an `IntoIter<Node>` containing the children of the node. Use `Node::leaf()` if there are no children of this node.
    pub fn new<A: AsRef<str>, B: Into<String>, I: IntoIterator<Item = Node>>(kind: A, value: B, children: I) -> Self {
        let mut val = Self::leaf(kind, value);
        val.children = children.into_iter().collect();
        val
    }

    /// Returns the value in the node.
    pub fn value(&self) -> &String {
        &self.value
    }
}