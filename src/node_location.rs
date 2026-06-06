#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeLocation {
    pub line: usize,
    pub column: usize,
}

impl std::fmt::Display for NodeLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[line:{},col:{}]", self.line, self.column)
    }
}

impl<T: syn::spanned::Spanned> From<&T> for NodeLocation {
    fn from(node: &T) -> Self {
        let start = node.span().start();
        NodeLocation {
            line: start.line,
            column: start.column,
        }
    }
}
