use crate::lexer::LexerToken;
use crate::{Parser, annotations};
use ast::SyntaxKind;

pub fn plugin_directive(p: &mut Parser) {
    let stub = p.start();

    annotations::parse(p);

    p.bump(LexerToken::KeywordPlugin);
    p.expect(LexerToken::Whitespace);

    p.expect(LexerToken::String);

    stub.complete(p, SyntaxKind::PluginDirective);
}

pub fn import_directive(p: &mut Parser) {
    let stub = p.start();

    annotations::parse(p);

    p.bump(LexerToken::KeywordImport);
    p.expect(LexerToken::Whitespace);

    p.expect(LexerToken::String);

    if p.at(LexerToken::Whitespace) && p.peek_at(LexerToken::KeywordAs) {
        p.bump(LexerToken::Whitespace);
        p.bump(LexerToken::KeywordAs);
        p.expect(LexerToken::Whitespace);
        p.expect(LexerToken::Identifier);
    }

    stub.complete(p, SyntaxKind::ImportDirective);
}

#[cfg(test)]
mod test {
    use crate::testing::expect_events;
    use ast::SyntaxKind;

    #[test]
    fn test_plugin_directive() {
        use super::plugin_directive as parse;

        expect_events!(
            r#"plugin "https://yap.microdragon.rs/microdragon/rust-backend/0.0.0/plugin.wasm""#,
            (start SyntaxKind::PluginDirective),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::String),
            finish
        )
    }

    #[test]
    fn test_import_directive() {
        use super::import_directive as parse;

        expect_events!(
            r#"import "https://yap.microdragon.rs/microdragon/rust-backend/0.0.0/index.yip""#,
            (start SyntaxKind::ImportDirective),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::String),
            finish
        )
    }
}
