//! ## Yip IDL Ast
//!
//! This crate implements the Abstract Syntax Tree (AST) for the Yip Interface Definition Language (IDL).
//! The AST uses [`rowan`] internally to represent the tree, without needing other crates to depend on it.
//! The AST is constructed using [`event::Event`]s, decoupling parser and AST.
//!
pub mod annotations;
pub mod event;
pub mod expression;
pub mod items;
mod kind;
mod macros;
pub mod types;

pub use kind::SyntaxKind;
pub use rowan::ast::AstPtr;

/// A SyntaxNode tied to our language.
pub type SyntaxNode = rowan::SyntaxNode<Yip>;

/// A SyntaxToken tied to our language.
pub type SyntaxToken = rowan::SyntaxToken<Yip>;

/// A SyntaxElement tied to our language.
pub type SyntaxElement = rowan::SyntaxElement<Yip>;

/// The rowan language definition, used for conversion of [`SyntaxKind`]s inside rowan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Yip {}

impl rowan::Language for Yip {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 <= SyntaxKind::File as u16);
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}
