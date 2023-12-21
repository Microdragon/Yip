pub mod alias;
pub mod constants;
pub mod directives;
pub mod enums;
pub mod handles;
pub mod interfaces;
pub mod structure;
pub mod unions;
pub mod uses;

use crate::lexer::LexerToken;
use crate::{utils, Parser, annotations};
use ast::SyntaxKind;

pub fn parse(p: &mut Parser) {
    match p.current() {
        LexerToken::KeywordImport => directives::import_directive(p),
        LexerToken::KeywordPlugin => directives::plugin_directive(p),
        LexerToken::KeywordStruct => structure::parse(p),
        LexerToken::KeywordUnion => unions::parse(p),
        LexerToken::KeywordEnum => enums::parse(p),
        LexerToken::KeywordType => alias::parse(p),
        LexerToken::KeywordHandle => handles::parse(p),
        LexerToken::KeywordInterface => interfaces::parse(p),
        LexerToken::KeywordConst => constants::parse(p),
        LexerToken::KeywordMod => parse_module(p),
        LexerToken::KeywordUse => uses::parse(p),
        _ => p.error("Item"),
    }
}

pub fn parse_file(p: &mut Parser) {
    let stub = p.start();

    utils::whitespace_newline(p);

    while !p.at(LexerToken::EndOfFile) {
        parse(p);
        utils::whitespace_newline(p);
    }

    stub.complete(p, SyntaxKind::File);
}

fn parse_module(p: &mut Parser) {
    let stub = p.start();

    annotations::parse(p);

    p.bump(LexerToken::KeywordMod);
    p.expect(LexerToken::Whitespace);

    p.expect(LexerToken::Identifier);
    utils::whitespace_newline(p);

    p.expect(LexerToken::LBraces);
    utils::whitespace_newline(p);

    while !p.eat(LexerToken::RBraces) {
        parse(p);
        utils::whitespace_newline(p);
    }

    stub.complete(p, SyntaxKind::ModuleDefinition);
}

#[cfg(test)]
mod test {
    use crate::testing::expect_events;
    use ast::SyntaxKind;

    #[test]
    fn test_parse_module() {
        use super::parse_module as parse;

        expect_events!(
            "mod Test {}",
            (start SyntaxKind::ModuleDefinition),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Punctuation),
            finish
        );

        expect_events!(
            "mod Test {
                handle Yap
            }",
            (start SyntaxKind::ModuleDefinition),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Newline),
            (token SyntaxKind::Whitespace),
            (start SyntaxKind::HandleDefinition),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            finish,
            (token SyntaxKind::Newline),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            finish
        );

        expect_events!(
            "mod Test {
                handle Yap

                enum Test {}
            }",
            (start SyntaxKind::ModuleDefinition),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Newline),
            (token SyntaxKind::Whitespace),
            (start SyntaxKind::HandleDefinition),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            finish,
            (token SyntaxKind::Newline),
            (token SyntaxKind::Newline),
            (token SyntaxKind::Whitespace),
            (start SyntaxKind::EnumDefinition),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Punctuation),
            finish,
            (token SyntaxKind::Newline),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            finish
        );
    }

    #[test]
    fn test_parse_file() {
        use super::parse_file as parse;

        expect_events!(
            "handle Yap",
            (start SyntaxKind::File),
            (start SyntaxKind::HandleDefinition),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            finish,
            finish
        );

        expect_events!(
            "handle Yap

            enum Test {}",
            (start SyntaxKind::File),
            (start SyntaxKind::HandleDefinition),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            finish,
            (token SyntaxKind::Newline),
            (token SyntaxKind::Newline),
            (token SyntaxKind::Whitespace),
            (start SyntaxKind::EnumDefinition),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Punctuation),
            finish,
            finish
        );
    }
}
