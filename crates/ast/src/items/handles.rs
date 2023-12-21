use crate::annotations::Annotation;
use crate::macros::ast_node;
use crate::{SyntaxKind, SyntaxToken};
use rowan::ast::{support, AstChildren};

ast_node!(HandleDefinition, SyntaxKind::HandleDefinition);

impl HandleDefinition {
    pub fn annotations(&self) -> AstChildren<Annotation> {
        support::children(&self.0)
    }

    pub fn name(&self) -> SyntaxToken {
        support::token(&self.0, SyntaxKind::Identifier).unwrap()
    }
}
