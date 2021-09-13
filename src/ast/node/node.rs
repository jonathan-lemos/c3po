use std::iter::IntoIterator;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Leaf(Leaf),
    Branch(Branch)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Leaf {
    kind: String,
    value: String
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Branch {
    kind: String,
    children: Vec<Node>
}

impl Node {
    /// Returns a new Node with no children
    /// 
    /// Use `Node::new()` if the node should have children.
    /// 
    /// # Arguments
    /// * `kind`     - an `AsRef<str>` containing what kind of value this node contains (e.g. comma, identifier, keyword, number, string, etc.)
    /// * `value`    - an `Into<String>` containing the contents of the node
    pub fn leaf<A: Into<String>, B: Into<String>>(kind: A, value: B) -> Self {
        Node::Leaf(Leaf {
            kind: kind.into(),
            value: value.into(),
        })
    }

    /// Returns a new Node
    /// 
    /// # Arguments
    /// * `kind`     - an `AsRef<str>` containing what kind of value this node contains (e.g. comma, identifier, keyword, number, string, etc.)
    /// * `value`    - an `Into<String>` containing the contents of the node
    /// * `children` - an `IntoIter<Node>` containing the children of the node. Use `Node::leaf()` if there are no children of this node.
    pub fn branch<A: Into<String>, I: IntoIterator<Item = Node>>(kind: A, children: I) -> Self {
        Node::Branch(Branch {
            kind: kind.into(),
            children: children.into_iter().collect()
        })
    }

    /// Returns the kind of the node.
    pub fn kind(&self) -> &str {
        match self {
            Node::Branch(n) => &n.kind,
            Node::Leaf(n) => &n.kind
        }
    }
}