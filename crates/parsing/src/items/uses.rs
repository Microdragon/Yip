use crate::lexer::LexerToken;
use crate::{utils, Parser, annotations};
use ast::SyntaxKind;

pub fn parse(p: &mut Parser) {
    let stub = p.start();

    annotations::parse(p);

    p.bump(LexerToken::KeywordUse);
    p.expect(LexerToken::Whitespace);

    loop {
        p.eat(LexerToken::Whitespace);

        if p.eat(LexerToken::LBraces) {
            while !p.at(LexerToken::RBraces) {
                utils::whitespace_newline(p);
                p.expect(LexerToken::Identifier);

                utils::whitespace_newline(p);
                if !p.at(LexerToken::RBraces) {
                    p.expect(LexerToken::Comma);
                    utils::whitespace_newline(p);
                }
            }

            p.eat(LexerToken::Whitespace);
            p.bump(LexerToken::RBraces);
            break;
        }

        p.expect(LexerToken::Identifier);
        p.eat(LexerToken::Whitespace);

        if !p.eat(LexerToken::ColonColon) {
            break;
        }
    }

    stub.complete(p, SyntaxKind::UseDirective);
}

#[cfg(test)]
mod test {
    use super::parse;
    use crate::testing::expect_events;
    use ast::SyntaxKind;

    #[test]
    fn test_parse() {
        expect_events!(
            "use yip::yap::yop",
            (start SyntaxKind::UseDirective),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Identifier),
            finish
        );

        expect_events!(
            "use yip::yap::yop::{}",
            (start SyntaxKind::UseDirective),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Punctuation),
            finish
        );

        expect_events!(
            "use yip::yap::yop::{test1, test2}",
            (start SyntaxKind::UseDirective),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Punctuation),
            finish
        );
    }
}
