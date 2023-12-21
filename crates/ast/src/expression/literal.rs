use crate::macros::ast_node;
use crate::{SyntaxKind, SyntaxNode, SyntaxToken};
use rowan::ast::{support, AstNode};
use text_size::TextRange;

#[derive(PartialEq, Eq, Hash)]
pub enum Literal {
    Boolean(BooleanLiteral),
    None(NoneLiteral),
    String(StringLiteral),
    Integer(IntegerLiteral),
    Float(FloatLiteral),
    Path(PathLiteral),
}

impl Literal {
    pub fn text_range(&self) -> TextRange {
        match self {
            Literal::Boolean(x) => x.syntax().text_range(),
            Literal::None(x) => x.syntax().text_range(),
            Literal::String(x) => x.syntax().text_range(),
            Literal::Integer(x) => x.syntax().text_range(),
            Literal::Float(x) => x.syntax().text_range(),
            Literal::Path(x) => x.syntax().text_range(),
        }
    }
}

impl AstNode for Literal {
    type Language = crate::Yip;

    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::Literal
    }

    fn cast(node: SyntaxNode) -> Option<Self> {
        if node.kind() == SyntaxKind::Literal {
            let first = node.first_token()?;

            match first.kind() {
                SyntaxKind::Boolean => Some(Literal::Boolean(BooleanLiteral::cast(node)?)),
                SyntaxKind::NoneValue => Some(Literal::None(NoneLiteral::cast(node)?)),
                SyntaxKind::String => Some(Literal::String(StringLiteral::cast(node)?)),
                SyntaxKind::Integer => Some(Literal::Integer(IntegerLiteral::cast(node)?)),
                SyntaxKind::Float => Some(Literal::Float(FloatLiteral::cast(node)?)),
                SyntaxKind::Identifier => Some(Literal::Path(PathLiteral::cast(node)?)),
                _ => None,
            }
        } else {
            None
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        match self {
            Literal::Boolean(x) => x.syntax(),
            Literal::None(x) => x.syntax(),
            Literal::String(x) => x.syntax(),
            Literal::Integer(x) => x.syntax(),
            Literal::Float(x) => x.syntax(),
            Literal::Path(x) => x.syntax(),
        }
    }
}

ast_node!(BooleanLiteral, SyntaxKind::Literal);

impl BooleanLiteral {
    pub fn token(&self) -> SyntaxToken {
        support::token(&self.0, SyntaxKind::Boolean).unwrap()
    }
}

ast_node!(NoneLiteral, SyntaxKind::Literal);

impl NoneLiteral {
    pub fn token(&self) -> SyntaxToken {
        support::token(&self.0, SyntaxKind::NoneValue).unwrap()
    }
}

ast_node!(StringLiteral, SyntaxKind::Literal);

impl StringLiteral {
    pub fn token(&self) -> SyntaxToken {
        support::token(&self.0, SyntaxKind::String).unwrap()
    }
}

ast_node!(IntegerLiteral, SyntaxKind::Literal);

impl IntegerLiteral {
    pub fn token(&self) -> SyntaxToken {
        support::token(&self.0, SyntaxKind::Integer).unwrap()
    }
}

ast_node!(FloatLiteral, SyntaxKind::Literal);

impl FloatLiteral {
    pub fn token(&self) -> SyntaxToken {
        support::token(&self.0, SyntaxKind::Float).unwrap()
    }
}

ast_node!(PathLiteral, SyntaxKind::Literal);

impl PathLiteral {
    pub fn segments(&self) -> impl Iterator<Item = SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(|x| x.into_token())
            .filter(|x| x.kind() == SyntaxKind::Identifier)
    }
}
