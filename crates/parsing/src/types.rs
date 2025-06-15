use crate::lexer::LexerToken;
use crate::{utils, Parser};
use ast::SyntaxKind;

pub fn parse(p: &mut Parser) {
    match p.current() {
        LexerToken::KeywordRecv => recv_type(p),
        LexerToken::KeywordSend => send_type(p),
        LexerToken::Question => option_type(p),
        LexerToken::KeywordResult => result_type(p),
        LexerToken::LParentheses => tuple_type(p),
        LexerToken::LBrackets => list_type(p),
        LexerToken::Identifier => named_type(p),
        _ => p.error("Type"),
    }
}

fn recv_type(p: &mut Parser) {
    let stub = p.start();

    p.bump(LexerToken::KeywordRecv);
    p.expect(LexerToken::Whitespace);
    utils::path(p);

    stub.complete(p, SyntaxKind::RecvType);
}

fn send_type(p: &mut Parser) {
    let stub = p.start();

    p.bump(LexerToken::KeywordSend);
    p.expect(LexerToken::Whitespace);
    utils::path(p);

    stub.complete(p, SyntaxKind::SendType);
}

fn result_type(p: &mut Parser) {
    let stub = p.start();

    p.bump(LexerToken::KeywordResult);
    p.expect(LexerToken::Whitespace);
    parse(p);

    if p.at(LexerToken::Whitespace) && p.peek_at(LexerToken::KeywordError) {
        p.bump(LexerToken::Whitespace);
        p.bump(LexerToken::KeywordError);
        p.expect(LexerToken::Whitespace);
        parse(p);
    }

    stub.complete(p, SyntaxKind::ResultType);
}

fn option_type(p: &mut Parser) {
    let stub = p.start();

    p.bump(LexerToken::Question);
    p.eat(LexerToken::Whitespace);
    parse(p);

    stub.complete(p, SyntaxKind::OptionType);
}

fn tuple_type(p: &mut Parser) {
    let stub = p.start();

    p.bump(LexerToken::LParentheses);
    while !p.eat(LexerToken::RParentheses) {
        p.eat(LexerToken::Whitespace);
        parse(p);
        p.eat(LexerToken::Whitespace);
        if !p.at(LexerToken::RParentheses) {
            p.expect(LexerToken::Comma);
        }
    }

    stub.complete(p, SyntaxKind::TupleType);
}

fn list_type(p: &mut Parser) {
    let stub = p.start();

    p.bump(LexerToken::LBrackets);
    p.eat(LexerToken::Whitespace);
    parse(p);
    p.eat(LexerToken::Whitespace);

    if p.eat(LexerToken::Semicolon) {
        p.eat(LexerToken::Whitespace);
        p.expect(LexerToken::Integer);
        p.eat(LexerToken::Whitespace);
    }

    p.expect(LexerToken::RBrackets);

    stub.complete(p, SyntaxKind::ListType);
}

fn named_type(p: &mut Parser) {
    let stub = p.start();

    utils::path(p);

    stub.complete(p, SyntaxKind::NamedType);
}

#[cfg(test)]
mod test {
    use super::parse;
    use crate::testing::expect_events;
    use ast::SyntaxKind;

    #[test]
    fn test_borrow() {
        expect_events!(
            "recv test",
            (start SyntaxKind::RecvType),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            finish
        );
    }

    #[test]
    fn test_own() {
        expect_events!(
            "send test",
            (start SyntaxKind::SendType),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Identifier),
            finish
        );
    }

    #[test]
    fn test_result() {
        expect_events!(
            "result bool",
            (start SyntaxKind::ResultType),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (start SyntaxKind::NamedType),
            (token SyntaxKind::Identifier),
            finish,
            finish
        );

        expect_events!(
            "result bool error yip::bool",
            (start SyntaxKind::ResultType),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (start SyntaxKind::NamedType),
            (token SyntaxKind::Identifier),
            finish,
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            (start SyntaxKind::NamedType),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Identifier),
            finish,
            finish
        );
    }

    #[test]
    fn test_option() {
        expect_events!(
            "?bool",
            (start SyntaxKind::OptionType),
            (token SyntaxKind::Punctuation),
            (start SyntaxKind::NamedType),
            (token SyntaxKind::Identifier),
            finish,
            finish
        );
    }

    #[test]
    fn test_tuple() {
        expect_events!(
            "(bool, yip::bool)",
            (start SyntaxKind::TupleType),
            (token SyntaxKind::Punctuation),
            (start SyntaxKind::NamedType),
            (token SyntaxKind::Identifier),
            finish,
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Whitespace),
            (start SyntaxKind::NamedType),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Identifier),
            finish,
            (token SyntaxKind::Punctuation),
            finish
        );

        expect_events!(
            "(bool)",
            (start SyntaxKind::TupleType),
            (token SyntaxKind::Punctuation),
            (start SyntaxKind::NamedType),
            (token SyntaxKind::Identifier),
            finish,
            (token SyntaxKind::Punctuation),
            finish
        );

        expect_events!(
            "()",
            (start SyntaxKind::TupleType),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Punctuation),
            finish
        );
    }

    #[test]
    fn test_list() {
        expect_events!(
            "[bool]",
            (start SyntaxKind::ListType),
            (token SyntaxKind::Punctuation),
            (start SyntaxKind::NamedType),
            (token SyntaxKind::Punctuation),
            finish,
            finish
        );

        expect_events!(
            "[yip::bool]",
            (start SyntaxKind::ListType),
            (token SyntaxKind::Punctuation),
            (start SyntaxKind::NamedType),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Identifier),
            finish,
            (token SyntaxKind::Punctuation),
            finish
        );
    }

    #[test]
    fn test_named() {
        expect_events!(
            "bool",
            (start SyntaxKind::NamedType),
            (token SyntaxKind::Identifier),
            finish
        );

        expect_events!(
            "yip::bool",
            (start SyntaxKind::NamedType),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Identifier),
            finish
        );

        expect_events!(
            "very::long::ty",
            (start SyntaxKind::NamedType),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Identifier),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Identifier),
            finish
        );
    }
}
