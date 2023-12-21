mod alias;
mod constants;
mod directives;
mod enums;
mod handles;
mod interfaces;
mod structure;
mod unions;
mod uses;

pub use alias::*;
pub use constants::*;
pub use directives::*;
pub use enums::*;
pub use handles::*;
pub use interfaces::*;
pub use structure::*;
pub use unions::*;
pub use uses::*;

use crate::annotations::Annotation;
use crate::macros::{ast_multi_node, ast_node};
use crate::{SyntaxKind, SyntaxToken};
use rowan::ast::{support, AstChildren};

ast_multi_node! {
    Item,
    Import(ImportDirective) => SyntaxKind::ImportDirective,
    Plugin(PluginDirective) => SyntaxKind::PluginDirective,
    Struct(StructDefinition) => SyntaxKind::StructDefinition,
    Union(UnionDefinition) => SyntaxKind::UnionDefinition,
    Enum(EnumDefinition) => SyntaxKind::EnumDefinition,
    Alias(AliasDefinition) => SyntaxKind::AliasDefinition,
    Handle(HandleDefinition) => SyntaxKind::HandleDefinition,
    Interface(InterfaceDefinition) => SyntaxKind::InterfaceDefinition,
    Const(ConstDefinition) => SyntaxKind::ConstDefinition,
    Module(ModuleDefinition) => SyntaxKind::ModuleDefinition,
    Use(UseDirective) => SyntaxKind::UseDirective,
}

ast_node!(File, SyntaxKind::File);

impl File {
    pub fn items(&self) -> AstChildren<Item> {
        support::children(&self.0)
    }
}

ast_node!(ModuleDefinition, SyntaxKind::ModuleDefinition);

impl ModuleDefinition {
    pub fn annotations(&self) -> AstChildren<Annotation> {
        support::children(&self.0)
    }

    pub fn name(&self) -> SyntaxToken {
        support::token(&self.0, SyntaxKind::Identifier).unwrap()
    }

    pub fn items(&self) -> AstChildren<Item> {
        support::children(&self.0)
    }
}
