use crate::lexer::LexerToken;
use crate::{types, Parser, annotations};
use ast::SyntaxKind;

pub fn parse(p: &mut Parser) {
    let stub = p.start();

    annotations::parse(p);

    p.bump(LexerToken::KeywordType);
    p.expect(LexerToken::Whitespace);

    p.expect(LexerToken::Identifier);
    p.eat(LexerToken::Whitespace);

    p.expect(LexerToken::Equals);
    p.eat(LexerToken::Whitespace);
    types::parse(p);

    stub.complete(p, SyntaxKind::AliasDefinition);
}

#[cfg(test)]
mod test {
    use super::parse;
    use crate::testing::expect_events;
    use ast::SyntaxKind;

    #[test]
    fn test_parse() {
        expect_events!(
            "type Test = bool",
            (start SyntaxKind::AliasDefinition),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Whitespace),
            (start SyntaxKind::NamedType),
            (token SyntaxKind::Identifier),
            finish,
            finish
        );

        expect_events!(
            "type Test = list bool",
            (start SyntaxKind::AliasDefinition),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Whitespace),
            (start SyntaxKind::ListType),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (start SyntaxKind::NamedType),
            (token SyntaxKind::Identifier),
            finish,
            finish,
            finish
        );

        expect_events!(
            "type Test = very::long::ty",
            (start SyntaxKind::AliasDefinition),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Whitespace),
            (start SyntaxKind::NamedType),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Identifier),
            finish,
            finish
        );
    }
}
