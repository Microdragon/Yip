use crate::lexer::LexerToken;
use crate::{annotations, expression, utils, Parser};
use ast::SyntaxKind;

pub fn parse(p: &mut Parser) {
    let stub = p.start();

    annotations::parse(p);

    p.bump(LexerToken::KeywordEnum);
    p.expect(LexerToken::Whitespace);

    p.expect(LexerToken::Identifier);
    utils::whitespace_newline(p);

    p.expect(LexerToken::LBraces);
    utils::whitespace_newline(p);

    while !p.eat(LexerToken::RBraces) {
        enum_member(p);
        utils::whitespace_newline(p);
    }

    stub.complete(p, SyntaxKind::EnumDefinition);
}

fn enum_member(p: &mut Parser) {
    let stub = p.start();

    annotations::parse(p);

    p.expect(LexerToken::Identifier);

    if p.at(LexerToken::Equals) || (p.at(LexerToken::Whitespace) && p.peek_at(LexerToken::Equals)) {
        p.eat(LexerToken::Whitespace);

        p.bump(LexerToken::Equals);
        p.eat(LexerToken::Whitespace);

        expression::parse(p);
    }

    stub.complete(p, SyntaxKind::EnumMember);
}

#[cfg(test)]
mod test {
    use super::parse;
    use crate::testing::expect_events;
    use ast::SyntaxKind;

    #[test]
    fn test_parse() {
        expect_events!(
            "enum Test {}",
            (start SyntaxKind::EnumDefinition),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Punctuation),
            finish
        );

        expect_events!(
            "enum Test { A B C = 5 }",
            (start SyntaxKind::EnumDefinition),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Whitespace),
            (start SyntaxKind::EnumMember),
            (token SyntaxKind::Identifier),
            finish,
            (token SyntaxKind::Whitespace),
            (start SyntaxKind::EnumMember),
            (token SyntaxKind::Identifier),
            finish,
            (token SyntaxKind::Whitespace),
            (start SyntaxKind::EnumMember),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Whitespace),
            stub,
            (start SyntaxKind::Literal),
            (token SyntaxKind::Integer),
            finish,
            finish,
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            finish
        );
    }
}
