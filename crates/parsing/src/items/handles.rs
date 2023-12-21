use crate::lexer::LexerToken;
use crate::{Parser, annotations};
use ast::SyntaxKind;

pub fn parse(p: &mut Parser) {
    let stub = p.start();

    annotations::parse(p);

    p.bump(LexerToken::KeywordHandle);
    p.expect(LexerToken::Whitespace);

    p.expect(LexerToken::Identifier);

    stub.complete(p, SyntaxKind::HandleDefinition);
}

#[cfg(test)]
mod test {
    use super::parse;
    use crate::testing::expect_events;
    use ast::SyntaxKind;

    #[test]
    fn test_parse() {
        expect_events!(
            "handle Test",
            (start SyntaxKind::HandleDefinition),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            finish
        );
    }
}
