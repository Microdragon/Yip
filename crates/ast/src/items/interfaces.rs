use crate::annotations::Annotation;
use crate::macros::ast_node;
use crate::types::Type;
use crate::{SyntaxKind, SyntaxToken};
use rowan::ast::{support, AstChildren};

ast_node!(InterfaceDefinition, SyntaxKind::InterfaceDefinition);

impl InterfaceDefinition {
    pub fn annotations(&self) -> AstChildren<Annotation> {
        support::children(&self.0)
    }

    pub fn name(&self) -> SyntaxToken {
        support::token(&self.0, SyntaxKind::Identifier).unwrap()
    }

    pub fn functions(&self) -> AstChildren<InterfaceFunction> {
        support::children(&self.0)
    }
}

ast_node!(InterfaceFunction, SyntaxKind::InterfaceFunction);

impl InterfaceFunction {
    pub fn annotations(&self) -> AstChildren<Annotation> {
        support::children(&self.0)
    }

    pub fn name(&self) -> SyntaxToken {
        support::token(&self.0, SyntaxKind::Identifier).unwrap()
    }

    pub fn parameters(&self) -> AstChildren<InterfaceFunctionParameter> {
        support::children(&self.0)
    }

    pub fn return_type(&self) -> Option<Type> {
        support::child(&self.0)
    }
}

ast_node!(
    InterfaceFunctionParameter,
    SyntaxKind::InterfaceFunctionParameter
);

impl InterfaceFunctionParameter {
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
