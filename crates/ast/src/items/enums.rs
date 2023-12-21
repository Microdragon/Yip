use crate::annotations::Annotation;
use crate::expression::Expression;
use crate::macros::ast_node;
use crate::{SyntaxKind, SyntaxToken};
use rowan::ast::{support, AstChildren};

ast_node!(EnumDefinition, SyntaxKind::EnumDefinition);

impl EnumDefinition {
    pub fn annotations(&self) -> AstChildren<Annotation> {
        support::children(&self.0)
    }

    pub fn name(&self) -> SyntaxToken {
        support::token(&self.0, SyntaxKind::Identifier).unwrap()
    }

    pub fn members(&self) -> AstChildren<EnumMember> {
        support::children(&self.0)
    }
}

ast_node!(EnumMember, SyntaxKind::EnumMember);

impl EnumMember {
    pub fn annotations(&self) -> AstChildren<Annotation> {
        support::children(&self.0)
    }

    pub fn name(&self) -> SyntaxToken {
        support::token(&self.0, SyntaxKind::Identifier).unwrap()
    }

    pub fn expression(&self) -> Expression {
        support::child(&self.0).unwrap()
    }
}
