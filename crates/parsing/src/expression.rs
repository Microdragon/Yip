use crate::lexer::{LexerToken, LexerTokenSet};
use crate::{utils, NodeStub, Parser};
use ast::SyntaxKind;

const OPERATORS: LexerTokenSet = LexerTokenSet::new(&[
    LexerToken::QuestionQuestion,
    LexerToken::Ampersand,
    LexerToken::AmpersandAmpersand,
    LexerToken::Caret,
    LexerToken::Bar,
    LexerToken::BarBar,
    LexerToken::EqualsEquals,
    LexerToken::BangEquals,
    LexerToken::LessThan,
    LexerToken::GreaterThan,
    LexerToken::LessThanEquals,
    LexerToken::GreaterThanEquals,
    LexerToken::LessThanLessThan,
    LexerToken::GreaterThanGreaterThan,
    LexerToken::Plus,
    LexerToken::Minus,
    LexerToken::Star,
    LexerToken::Slash,
    LexerToken::Percent,
]);

fn infix_binding_power(token: LexerToken) -> (u8, u8) {
    match token {
        LexerToken::QuestionQuestion => (1, 2),
        LexerToken::BarBar => (3, 4),
        LexerToken::AmpersandAmpersand => (5, 6),
        LexerToken::Bar => (7, 8),
        LexerToken::Caret => (9, 10),
        LexerToken::Ampersand => (11, 12),
        LexerToken::EqualsEquals | LexerToken::BangEquals => (13, 14),
        LexerToken::LessThan
        | LexerToken::GreaterThan
        | LexerToken::LessThanEquals
        | LexerToken::GreaterThanEquals => (15, 16),
        LexerToken::LessThanLessThan | LexerToken::GreaterThanGreaterThan => (17, 18),
        LexerToken::Plus | LexerToken::Minus => (19, 20),
        LexerToken::Star | LexerToken::Slash | LexerToken::Percent => (21, 22),
        _ => (0, 0),
    }
}

pub fn parse(p: &mut Parser) {
    if p.at(LexerToken::KeywordIf) {
        if_expression(p);
    } else {
        infix_operator(p, 0);
    }
}

fn if_expression(p: &mut Parser) {
    let stub = p.start();

    p.bump(LexerToken::KeywordIf);
    p.expect(LexerToken::Whitespace);
    infix_operator(p, 0);

    utils::whitespace_newline(p);
    p.expect(LexerToken::KeywordThen);
    p.expect(LexerToken::Whitespace);
    infix_operator(p, 0);

    utils::whitespace_newline(p);
    p.expect(LexerToken::KeywordElse);
    p.expect(LexerToken::Whitespace);
    infix_operator(p, 0);

    stub.complete(p, SyntaxKind::IfExpression);
}

fn infix_operator(p: &mut Parser, min: u8) {
    let mut stub = p.start();

    unary_operator(p);
    loop {
        let op = if p.at(LexerToken::Whitespace) {
            p.peek()
        } else {
            p.current()
        };

        if op == LexerToken::EndOfFile {
            break;
        }

        if !OPERATORS.contains(op) {
            break;
            //if op == LexerToken::EndOfFile {
            //stub.abandon(p);
            //} else {
            //p.error("Operator");
            //stub.complete(p, SyntaxKind::InfixOperator);
            //}
            //return;
        }

        let (lbp, rbp) = infix_binding_power(op);
        if lbp < min {
            break;
        }

        p.eat(LexerToken::Whitespace);
        p.do_bump(op.into());
        p.eat(LexerToken::Whitespace);
        infix_operator(p, rbp);

        let completed = stub.complete(p, SyntaxKind::InfixOperator);
        stub = completed.precede(p);
    }

    stub.abandon(p);
}

fn unary_operator(p: &mut Parser) {
    let stub = p.start();

    let current = p.current();
    match current {
        LexerToken::Plus | LexerToken::Minus | LexerToken::Bang => {
            p.do_bump(current.into());
            literal(p, None);
            stub.complete(p, SyntaxKind::UnaryOperator);
        }
        LexerToken::KeywordError => {
            p.do_bump(current.into());
            p.expect(LexerToken::Whitespace);
            parse(p);
            stub.complete(p, SyntaxKind::ErrorValue);
        }
        LexerToken::KeywordTrue | LexerToken::KeywordFalse => {
            p.do_bump(SyntaxKind::Boolean);
            stub.complete(p, SyntaxKind::Literal);
        }
        LexerToken::KeywordNone => {
            p.do_bump(SyntaxKind::NoneValue);
            stub.complete(p, SyntaxKind::Literal);
        }
        LexerToken::LParentheses => {
            p.do_bump(current.into());

            p.eat(LexerToken::Whitespace);
            while !p.eat(LexerToken::RParentheses) {
                parse(p);
                p.eat(LexerToken::Whitespace);
                if !p.at(LexerToken::RParentheses) {
                    p.expect(LexerToken::Comma);
                    p.eat(LexerToken::Whitespace);
                }
            }

            stub.complete(p, SyntaxKind::TupleExpression);
        }
        LexerToken::LBrackets => {
            p.do_bump(current.into());

            p.eat(LexerToken::Whitespace);
            while !p.eat(LexerToken::RBrackets) {
                parse(p);
                p.eat(LexerToken::Whitespace);
                if !p.at(LexerToken::RBrackets) {
                    p.expect(LexerToken::Comma);
                    p.eat(LexerToken::Whitespace);
                }
            }

            stub.complete(p, SyntaxKind::ListExpression);
        }
        _ => literal(p, Some(stub)),
    }
}

fn literal(p: &mut Parser, stub: Option<NodeStub>) {
    let stub = stub.unwrap_or_else(|| p.start());

    let current = p.current();
    match current {
        LexerToken::Integer | LexerToken::Float | LexerToken::String => p.do_bump(current.into()),
        LexerToken::Identifier => utils::path(p),
        _ => p.error("Expression"),
    }

    stub.complete(p, SyntaxKind::Literal);
}

#[cfg(test)]
mod test {
    use crate::testing::expect_events;
    use crate::Parser;
    use ast::SyntaxKind;

    #[test]
    fn test_if_expression() {
        use super::if_expression as parse;

        expect_events!(
            "if 1 then 2 else 3",
            (start SyntaxKind::IfExpression),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            stub,
            (start SyntaxKind::Literal),
            (token SyntaxKind::Integer),
            finish,
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            stub,
            (start SyntaxKind::Literal),
            (token SyntaxKind::Integer),
            finish,
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Keyword),
            (token SyntaxKind::Whitespace),
            stub,
            (start SyntaxKind::Literal),
            (token SyntaxKind::Integer),
            finish,
            finish
        );
    }

    fn infix_operator(p: &mut Parser) {
        super::infix_operator(p, 0)
    }

    #[test]
    fn test_infix_operator() {
        use self::infix_operator as parse;

        expect_events!(
            "1 + 2 * 'test' - true",
            (start SyntaxKind::InfixOperator, 20),
            (start SyntaxKind::Literal, 0),
            (token SyntaxKind::Integer),
            finish,
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Whitespace),
            (start SyntaxKind::InfixOperator, 12),
            (start SyntaxKind::Literal, 0),
            (token SyntaxKind::Integer),
            finish,
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Whitespace),
            stub,
            (start SyntaxKind::Literal, 0),
            (token SyntaxKind::String),
            finish,
            finish,
            finish,
            (start SyntaxKind::InfixOperator, 9),
            (token SyntaxKind::Whitespace),
            (token SyntaxKind::Punctuation),
            (token SyntaxKind::Whitespace),
            stub,
            (start SyntaxKind::Literal, 0),
            (token SyntaxKind::Boolean),
            finish,
            finish
        );
    }
}
