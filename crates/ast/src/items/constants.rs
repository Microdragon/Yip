use crate::annotations::Annotation;
use crate::expression::Expression;
use crate::macros::ast_node;
use crate::types::Type;
use crate::{SyntaxKind, SyntaxToken};
use rowan::ast::{support, AstChildren};

ast_node!(ConstDefinition, SyntaxKind::ConstDefinition);

impl ConstDefinition {
    pub fn annotations(&self) -> AstChildren<Annotation> {
        support::children(&self.0)
    }

    pub fn name(&self) -> SyntaxToken {
        support::token(&self.0, SyntaxKind::Identifier).unwrap()
    }

    pub fn ty(&self) -> Option<Type> {
        support::child(&self.0)
    }

    pub fn expression(&self) -> Expression {
        support::child(&self.0).unwrap()
    }
}
