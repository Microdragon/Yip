use super::structure;
use crate::lexer::LexerToken;
use crate::{annotations, types, utils, NodeStub, Parser};
use ast::SyntaxKind;

pub fn parse(p: &mut Parser) {
    let stub = p.start();

    annotations::parse(p);

    p.bump(LexerToken::KeywordUnion);
    p.expect(LexerToken::Whitespace);

    p.expect(LexerToken::Identifier);
    utils::whitespace_newline(p);

    p.expect(LexerToken::LBraces);
    utils::whitespace_newline(p);

    while !p.eat(LexerToken::RBraces) {
        union_member_or_anonymous_struct(p);
        utils::whitespace_newline(p);
    }

    stub.complete(p, SyntaxKind::UnionDefinition);
}

fn union_member_or_anonymous_struct(p: &mut Parser) {
    let stub = p.start();

    annotations::parse(p);

    p.expect(LexerToken::Identifier);
    p.eat(LexerToken::Whitespace);

    if p.eat(LexerToken::LParentheses) {
        union_member(p, stub);
    } else {
        union_anonymous_struct(p, stub);
    }
}

fn union_member(p: &mut Parser, stub: NodeStub) {
    p.eat(LexerToken::Whitespace);
    types::parse(p);

    p.eat(LexerToken::Whitespace);
    p.expect(LexerToken::RParentheses);

    stub.complete(p, SyntaxKind::UnionMember);
}

fn union_anonymous_struct(p: &mut Parser, stub: NodeStub) {
    utils::whitespace_newline(p);
    structure::struct_members(p);

    stub.complete(p, SyntaxKind::UnionAnonymousStruct);
}

#[cfg(test)]
mod test {
    use super::parse;
    use crate::testing::expect_events;
    use ast::SyntaxKind;

    #[test]
    fn test_parse() {
        expect_events!(
            "union Test {}",
            (start SyntaxKind::UnionDefinition),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Punctuation),
            finish
        );

        expect_events!(
            "union Test {
                A(Test1)
            }",
            (start SyntaxKind::UnionDefinition),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Newline),
            (token SyntaxKind::Whitespace),
            (start SyntaxKind::UnionMember),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Punctuation),
            (start SyntaxKind::NamedType),
            (token SyntaxKind::Identifier),
            finish,
            (token SyntaxKind::Punctuation),
            finish,
            (token SyntaxKind::Newline),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            finish
        );

        expect_events!(
            "union Test {
                A {
                    a: bool
                }
            }",
            (start SyntaxKind::UnionDefinition),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Newline),
            (token SyntaxKind::Whitespace),
            (start SyntaxKind::UnionAnonymousStruct),
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
            (token SyntaxKind::Punctuation),
            finish,
            (token SyntaxKind::Newline),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            finish
        );
    }
}
