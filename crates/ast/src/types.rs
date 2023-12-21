use crate::macros::{ast_multi_node, ast_node};
use crate::{SyntaxKind, SyntaxToken};
use rowan::ast::{support, AstChildren};

ast_multi_node! {
    Type,
    Borrow(BorrowType) => SyntaxKind::BorrowType,
    Own(OwnType) => SyntaxKind::OwnType,
    Result(ResultType) => SyntaxKind::ResultType,
    Option(OptionType) => SyntaxKind::OptionType,
    Tuple(TupleType) => SyntaxKind::TupleType,
    List(ListType) => SyntaxKind::ListType,
    Named(NamedType) => SyntaxKind::NamedType,
}

ast_node!(BorrowType, SyntaxKind::BorrowType);

impl BorrowType {
    pub fn name(&self) -> SyntaxToken {
        support::token(&self.0, SyntaxKind::Identifier).unwrap()
    }
}

ast_node!(OwnType, SyntaxKind::OwnType);

impl OwnType {
    pub fn name(&self) -> SyntaxToken {
        support::token(&self.0, SyntaxKind::Identifier).unwrap()
    }
}

ast_node!(ResultType, SyntaxKind::ResultType);

impl ResultType {
    pub fn inner(&self) -> Type {
        support::child(&self.0).unwrap()
    }
}

ast_node!(OptionType, SyntaxKind::OptionType);

impl OptionType {
    pub fn inner(&self) -> Type {
        support::child(&self.0).unwrap()
    }
}

ast_node!(TupleType, SyntaxKind::TupleType);

impl TupleType {
    pub fn types(&self) -> AstChildren<Type> {
        support::children(&self.0)
    }
}

ast_node!(ListType, SyntaxKind::ListType);

impl ListType {
    pub fn inner(&self) -> Type {
        support::child(&self.0).unwrap()
    }
}

ast_node!(NamedType, SyntaxKind::NamedType);

impl NamedType {
    pub fn segments(&self) -> impl Iterator<Item = SyntaxToken> {
        self.0
            .children_with_tokens()
            .filter_map(|x| x.into_token())
            .filter(|x| x.kind() == SyntaxKind::Identifier)
    }
}
