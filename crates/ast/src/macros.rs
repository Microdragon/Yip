macro_rules! ast_node {
    ($type:ident, $kind:expr) => {
        #[derive(PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct $type(crate::SyntaxNode);

        impl rowan::ast::AstNode for $type {
            type Language = crate::Yip;

            fn can_cast(kind: crate::SyntaxKind) -> bool {
                kind == ($kind)
            }

            fn cast(node: crate::SyntaxNode) -> Option<Self> {
                if node.kind() == ($kind) {
                    Some(Self(node))
                } else {
                    None
                }
            }

            fn syntax(&self) -> &crate::SyntaxNode {
                &self.0
            }
        }
    };
}

macro_rules! ast_multi_node {
    ($enum:ident , $($name:ident($type:ty) => $kind:pat),+ $(,)?) => {
        #[derive(PartialEq, Eq, Hash)]
        pub enum $enum {
            $($name($type)),+
        }

        impl rowan::ast::AstNode for $enum {
            type Language = crate::Yip;

            fn can_cast(kind: crate::SyntaxKind) -> bool {
                matches!(kind, $($kind)|+)
            }

            fn cast(node: crate::SyntaxNode) -> Option<Self> {
                match node.kind() {
                    $($kind => Some(Self::$name(<$type>::cast(node)?))),+,
                    _ => None
                }
            }

            fn syntax(&self) -> &crate::SyntaxNode {
                match self {
                    $(Self::$name(x) => x.syntax()),+
                }
            }
        }
    };
}

pub(crate) use ast_multi_node;
pub(crate) use ast_node;
