use crate::lexer::LexerToken;
use crate::{annotations, types, utils, Parser};
use ast::SyntaxKind;

pub fn parse(p: &mut Parser) {
    let stub = p.start();

    annotations::parse(p);

    p.bump(LexerToken::KeywordInterface);
    p.expect(LexerToken::Whitespace);

    p.expect(LexerToken::Identifier);
    utils::whitespace_newline(p);

    p.expect(LexerToken::LBraces);
    utils::whitespace_newline(p);

    while !p.eat(LexerToken::RBraces) {
        interface_function(p);
        utils::whitespace_newline(p);
    }

    stub.complete(p, SyntaxKind::InterfaceDefinition);
}

fn interface_function(p: &mut Parser) {
    let stub = p.start();

    annotations::parse(p);

    p.expect(LexerToken::KeywordFn);
    p.expect(LexerToken::Whitespace);

    p.expect(LexerToken::Identifier);
    p.eat(LexerToken::Whitespace);

    p.expect(LexerToken::LParentheses);
    p.eat(LexerToken::Whitespace);

    while !p.eat(LexerToken::RParentheses) {
        interface_function_parameter(p);
        p.eat(LexerToken::Whitespace);

        if !p.at(LexerToken::RParentheses) {
            p.expect(LexerToken::Comma);
            p.eat(LexerToken::Whitespace);
        }
    }

    if p.at(LexerToken::Colon) || (p.at(LexerToken::Whitespace) && p.peek_at(LexerToken::Colon)) {
        p.eat(LexerToken::Whitespace);

        p.bump(LexerToken::Colon);
        p.eat(LexerToken::Whitespace);

        types::parse(p);
    }

    if p.at(LexerToken::Equals) || (p.at(LexerToken::Whitespace) && p.peek_at(LexerToken::Equals)) {
        p.eat(LexerToken::Whitespace);

        p.bump(LexerToken::Equals);
        p.eat(LexerToken::Whitespace);

        p.expect(LexerToken::Integer);
    }

    stub.complete(p, SyntaxKind::InterfaceFunction);
}

fn interface_function_parameter(p: &mut Parser) {
    let stub = p.start();

    annotations::parse(p);

    p.expect(LexerToken::Identifier);
    p.eat(LexerToken::Whitespace);

    p.expect(LexerToken::Colon);
    p.eat(LexerToken::Whitespace);

    types::parse(p);

    stub.complete(p, SyntaxKind::InterfaceFunctionParameter);
}

#[cfg(test)]
mod test {
    use super::parse;
    use crate::testing::expect_events;
    use ast::SyntaxKind;

    #[test]
    fn test_parse() {
        expect_events!(
            "interface Test {}",
            (start SyntaxKind::InterfaceDefinition),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Punctuation),
            finish
        );

        expect_events!(
            "interface Test {
                fn unit()
            }",
            (start SyntaxKind::InterfaceDefinition),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Newline),
            (token SyntaxKind::Whitespace),
            (start SyntaxKind::InterfaceFunction),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Punctuation),
            finish,
            (token SyntaxKind::Newline),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            finish
        );

        expect_events!(
            "interface Test {
                fn test(a: bool, b: bool): bool
            }",
            (start SyntaxKind::InterfaceDefinition),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Newline),
            (token SyntaxKind::Whitespace),
            (start SyntaxKind::InterfaceFunction),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Punctuation),
            (start SyntaxKind::InterfaceFunctionParameter),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Whitespace),
            (start SyntaxKind::NamedType),
            (token SyntaxKind::Identifier),
            finish,
            finish,
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Whitespace),
            (start SyntaxKind::InterfaceFunctionParameter),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Whitespace),
            (start SyntaxKind::NamedType),
            (token SyntaxKind::Identifier),
            finish,
            finish,
            (token SyntaxKind::Punctuation),
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
