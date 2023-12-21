use crate::annotations::Annotation;
use crate::macros::ast_node;
use crate::types::Type;
use crate::{SyntaxKind, SyntaxToken};
use rowan::ast::{support, AstChildren};

ast_node!(AliasDefinition, SyntaxKind::AliasDefinition);

impl AliasDefinition {
    pub fn annotations(&self) -> AstChildren<Annotation> {
        support::children(&self.0)
    }

    pub fn name(&self) -> SyntaxToken {
        support::token(&self.0, SyntaxKind::Identifier).unwrap()
    }

    pub fn ty(&self) -> Type {
        support::child(&self.0).unwrap()
    }
}
