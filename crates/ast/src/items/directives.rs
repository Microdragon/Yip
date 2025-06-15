use crate::annotations::Annotation;
use crate::macros::ast_node;
use crate::{SyntaxKind, SyntaxToken};
use rowan::ast::{support, AstChildren};

ast_node!(ImportDirective, SyntaxKind::ImportDirective);

impl ImportDirective {
    pub fn annotations(&self) -> AstChildren<Annotation> {
        support::children(&self.0)
    }

    pub fn url(&self) -> SyntaxToken {
        support::token(&self.0, SyntaxKind::String).unwrap()
    }
}

ast_node!(PluginDirective, SyntaxKind::PluginDirective);

impl PluginDirective {
    pub fn annotations(&self) -> AstChildren<Annotation> {
        support::children(&self.0)
    }

    pub fn url(&self) -> SyntaxToken {
        support::token(&self.0, SyntaxKind::String).unwrap()
    }

    pub fn module_name(&self) -> Option<SyntaxToken> {
        support::token(&self.0, SyntaxKind::Identifier)
    }
}
