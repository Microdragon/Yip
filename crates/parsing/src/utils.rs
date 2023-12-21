use crate::lexer::{LexerToken, LexerTokenSet};
use crate::Parser;

const WHITESPACE_NEWLINE: LexerTokenSet =
    LexerTokenSet::new(&[LexerToken::Whitespace, LexerToken::Newline]);

pub fn whitespace_newline(p: &mut Parser) {
    while p.eat_set(WHITESPACE_NEWLINE) {}
}

pub fn path(p: &mut Parser) {
    p.expect(LexerToken::Identifier);

    while p.at(LexerToken::ColonColon)
        || (p.at(LexerToken::Whitespace) && p.peek_at(LexerToken::ColonColon))
    {
        p.eat(LexerToken::Whitespace);
        p.bump(LexerToken::ColonColon);
        p.eat(LexerToken::Whitespace);
        p.expect(LexerToken::Identifier);
    }
}
