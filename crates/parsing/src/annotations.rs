use crate::{expression, utils, LexerToken, Parser};
use ast::SyntaxKind;

pub fn parse(p: &mut Parser) {
    while p.at(LexerToken::AtSign) {
        parse_annotation(p);
        utils::whitespace_newline(p);
    }
}

fn parse_annotation(p: &mut Parser) {
    let stub = p.start();

    p.bump(LexerToken::AtSign);
    p.eat(LexerToken::Whitespace);
    p.expect(LexerToken::Identifier);

    if p.at(LexerToken::LParentheses)
        || (p.at(LexerToken::Whitespace) && p.peek_at(LexerToken::LParentheses))
    {
        p.eat(LexerToken::Whitespace);
        p.bump(LexerToken::LParentheses);
        p.eat(LexerToken::Whitespace);

        while !p.eat(LexerToken::RParentheses) {
            expression::parse(p);
            p.eat(LexerToken::Whitespace);

            if !p.at(LexerToken::RParentheses) {
                p.expect(LexerToken::Comma);
                p.eat(LexerToken::Whitespace);
            }
        }
    }

    stub.complete(p, SyntaxKind::Annotation);
}

#[cfg(test)]
mod test {
    use ast::SyntaxKind;

    use super::parse;
    use crate::testing::expect_events;

    #[test]
    fn test_parse() {
        expect_events!(
            "@Test",
            (start SyntaxKind::Annotation),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Identifier),
            finish
        );

        expect_events!(
            "@Test @Yip()",
            (start SyntaxKind::Annotation),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Identifier),
            finish,
            (token SyntaxKind::Whitespace),
            (start SyntaxKind::Annotation),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Punctuation),
            finish
        );

        expect_events!(
            "@Yap(1, 2, 3)",
            (start SyntaxKind::Annotation),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Punctuation),
            stub,
            (start SyntaxKind::Literal),
            (token SyntaxKind::Integer),
            finish,
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Whitespace),
            stub,
            (start SyntaxKind::Literal),
            (token SyntaxKind::Integer),
            finish,
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Whitespace),
            stub,
            (start SyntaxKind::Literal),
            (token SyntaxKind::Integer),
            finish,
            (token SyntaxKind::Punctuation),
            finish
        );
    }
}
