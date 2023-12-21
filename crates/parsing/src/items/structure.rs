use crate::lexer::LexerToken;
use crate::{types, utils, Parser, annotations};
use ast::SyntaxKind;

pub fn parse(p: &mut Parser) {
    let stub = p.start();

    annotations::parse(p);

    p.bump(LexerToken::KeywordStruct);
    p.expect(LexerToken::Whitespace);

    p.expect(LexerToken::Identifier);
    utils::whitespace_newline(p);

    struct_members(p);

    stub.complete(p, SyntaxKind::StructDefinition);
}

pub(crate) fn struct_members(p: &mut Parser) {
    p.expect(LexerToken::LBraces);
    utils::whitespace_newline(p);

    while !p.eat(LexerToken::RBraces) {
        struct_member(p);
        utils::whitespace_newline(p);
    }
}

fn struct_member(p: &mut Parser) {
    let stub = p.start();

    annotations::parse(p);

    p.expect(LexerToken::Identifier);
    p.eat(LexerToken::Whitespace);

    p.expect(LexerToken::Colon);
    p.eat(LexerToken::Whitespace);

    types::parse(p);

    stub.complete(p, SyntaxKind::StructMember);
}

#[cfg(test)]
mod test {
    use crate::testing::expect_events;
    use ast::SyntaxKind;
    use super::parse;

    #[test]
    fn test_parse() {
        expect_events!(
            "struct Test {}",
            (start SyntaxKind::StructDefinition),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Punctuation),
            finish
        );

        expect_events!(
            "struct Test {
                a: bool
                b: bool
            }",
            (start SyntaxKind::StructDefinition),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Newline),
            (token SyntaxKind::Whitespace),
            (start SyntaxKind::StructMember),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Whitespace),
            (start SyntaxKind::NamedType),
            (token SyntaxKind::Identifier),
            finish,
            finish,
            (token SyntaxKind::Newline),
            (token SyntaxKind::Whitespace),
            (start SyntaxKind::StructMember),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Whitespace),
            (start SyntaxKind::NamedType),
            (token SyntaxKind::Identifier),
            finish,
            finish,
            (token SyntaxKind::Newline),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            finish
        );
    }
}
