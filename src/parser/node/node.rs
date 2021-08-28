use std::iter::IntoIterator;

/// Represents a node in a parse tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node<TValue> {
    kind: String,
    value: TValue,
    children: Vec<Node<TValue>> 
}

impl<TValue> Node<TValue> {
    /// Yields the immediate children of this node.
    pub fn children(&self) -> impl Iterator<Item = &Node<TValue>> {
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
    /// * `value`    - an `Into<TValue>` containing the contents of the node
    pub fn leaf<A: AsRef<str>, B: Into<TValue>>(kind: A, value: B) -> Self {
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
    /// * `value`    - an `Into<TValue>` containing the contents of the node
    /// * `children` - an `IntoIter<Node>` containing the children of the node. Use `Node::leaf()` if there are no children of this node.
    pub fn new<A: AsRef<str>, B: Into<TValue>, I: IntoIterator<Item = Node<TValue>>>(kind: A, value: B, children: I) -> Self {
        let mut val = Self::leaf(kind, value);
        val.children = children.into_iter().collect();
        val
    }

    /// Returns the value in the node.
    pub fn value(&self) -> &TValue {
        &self.value
    }
}