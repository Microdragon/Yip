use super::StructMember;
use crate::annotations::Annotation;
use crate::macros::{ast_multi_node, ast_node};
use crate::types::Type;
use crate::{SyntaxKind, SyntaxToken};
use rowan::ast::{support, AstChildren};

ast_node!(UnionDefinition, SyntaxKind::UnionDefinition);

impl UnionDefinition {
    pub fn annotations(&self) -> AstChildren<Annotation> {
        support::children(&self.0)
    }

    pub fn name(&self) -> SyntaxToken {
        support::token(&self.0, SyntaxKind::Identifier).unwrap()
    }

    pub fn members(&self) -> AstChildren<UnionMember> {
        support::children(&self.0)
    }
}

ast_multi_node! {
    UnionMember,
    Simple(SimpleUnionMember) => SyntaxKind::UnionMember,
    Anonymous(UnionAnonymousStruct) => SyntaxKind::UnionAnonymousStruct,
}

impl UnionMember {
    pub fn annotations(&self) -> AstChildren<Annotation> {
        match self {
            UnionMember::Simple(x) => x.annotations(),
            UnionMember::Anonymous(x) => x.annotations(),
        }
    }

    pub fn name(&self) -> SyntaxToken {
        match self {
            UnionMember::Simple(x) => x.name(),
            UnionMember::Anonymous(x) => x.name(),
        }
    }
}

ast_node!(SimpleUnionMember, SyntaxKind::UnionMember);

impl SimpleUnionMember {
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

ast_node!(UnionAnonymousStruct, SyntaxKind::UnionAnonymousStruct);

impl UnionAnonymousStruct {
    pub fn annotations(&self) -> AstChildren<Annotation> {
        support::children(&self.0)
    }

    pub fn name(&self) -> SyntaxToken {
        support::token(&self.0, SyntaxKind::Identifier).unwrap()
    }

    pub fn members(&self) -> AstChildren<StructMember> {
        support::children(&self.0)
    }
}
