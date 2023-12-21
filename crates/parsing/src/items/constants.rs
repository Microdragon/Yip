use crate::lexer::LexerToken;
use crate::{expression, types, Parser, annotations};
use ast::SyntaxKind;

pub fn parse(p: &mut Parser) {
    let stub = p.start();

    annotations::parse(p);

    p.bump(LexerToken::KeywordConst);
    p.expect(LexerToken::Whitespace);

    p.expect(LexerToken::Identifier);
    p.eat(LexerToken::Whitespace);

    if p.eat(LexerToken::Colon) {
        p.eat(LexerToken::Whitespace);
        types::parse(p);

        p.eat(LexerToken::Whitespace);
    }

    p.expect(LexerToken::Equals);

    p.eat(LexerToken::Whitespace);
    expression::parse(p);

    stub.complete(p, SyntaxKind::ConstDefinition);
}

#[cfg(test)]
mod test {
    use super::parse;
    use crate::testing::expect_events;
    use ast::SyntaxKind;

    #[test]
    fn test_parse() {
        expect_events!(
            "const TEST: bool = true",
            (start SyntaxKind::ConstDefinition),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Whitespace),
            (start SyntaxKind::NamedType),
            (token SyntaxKind::Identifier),
            finish,
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Whitespace),
            stub,
            (start SyntaxKind::Literal),
            (token SyntaxKind::Boolean),
            finish,
            finish
        );

        expect_events!(
            "const TEST = none",
            (start SyntaxKind::ConstDefinition),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Whitespace),
            stub,
            (start SyntaxKind::Literal),
            (token SyntaxKind::NoneValue),
            finish,
            finish
        );

        expect_events!(
            "const TEST: list bool = []",
            (start SyntaxKind::ConstDefinition),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Whitespace),
            (start SyntaxKind::ListType),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (start SyntaxKind::NamedType),
            (token SyntaxKind::Identifier),
            finish,
            finish,
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Whitespace),
            stub,
            (start SyntaxKind::ListExpression),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Punctuation),
            finish,
            finish
        );

        expect_events!(
            "const TEST: yip::yap::yop = FOO",
            (start SyntaxKind::ConstDefinition),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Whitespace),
            (start SyntaxKind::NamedType),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Identifier),
            finish,
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Whitespace),
            stub,
            (start SyntaxKind::Literal),
            (token SyntaxKind::Identifier),
            finish,
            finish
        );
    }
}
