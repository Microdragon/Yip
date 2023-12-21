use crate::lexer::{LexerToken, LexerTokenSet};
use ast::event::Event;
use ast::SyntaxKind;
use drop_bomb::DropBomb;
use logos::{Logos, Span};
use text_size::{TextRange, TextSize};

pub struct Parser {
    tokens: Vec<(LexerToken, Span)>,
    position: usize,
    events: Vec<Event>,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let tokens = LexerToken::lexer(input)
            .spanned()
            .map(|(token, span)| (token.unwrap(), span))
            .collect();

        Parser {
            tokens,
            position: 0,
            events: Vec::new(),
        }
    }

    pub fn finish(self) -> Vec<Event> {
        self.events
    }

    pub fn current(&self) -> LexerToken {
        self.tokens
            .get(self.position)
            .map(|(token, _)| *token)
            .unwrap_or(LexerToken::EndOfFile)
    }

    pub fn peek(&self) -> LexerToken {
        self.tokens
            .get(self.position + 1)
            .map(|(token, _)| *token)
            .unwrap_or(LexerToken::EndOfFile)
    }

    pub fn span(&self) -> Span {
        self.tokens
            .get(self.position)
            .map(|(_, span)| span)
            .cloned()
            .unwrap_or_default()
    }

    pub fn at(&self, token: LexerToken) -> bool {
        self.current() == token
    }

    pub fn peek_at(&self, token: LexerToken) -> bool {
        self.peek() == token
    }

    pub fn at_set(&self, set: LexerTokenSet) -> bool {
        set.contains(self.current())
    }

    pub fn peek_at_set(&self, set: LexerTokenSet) -> bool {
        set.contains(self.peek())
    }

    pub fn eat(&mut self, token: LexerToken) -> bool {
        if self.at(token) {
            self.do_bump(token.into());

            true
        } else {
            false
        }
    }

    pub fn eat_set(&mut self, set: LexerTokenSet) -> bool {
        let current = self.current();
        if set.contains(current) {
            self.do_bump(current.into());

            true
        } else {
            false
        }
    }

    pub fn start(&mut self) -> NodeStub {
        let position = self.events.len();
        self.events.push(Event::Stub);

        NodeStub::new(position)
    }

    pub fn bump(&mut self, token: LexerToken) {
        assert!(self.eat(token));
    }

    pub fn bump_any(&mut self) {
        let token = self.current();
        if token != LexerToken::EndOfFile {
            self.do_bump(token.into());
        }
    }

    pub fn error(&mut self, expected: &'static str) {
        let span = self.span();
        let range = TextRange::new(
            TextSize::new(span.start as u32),
            TextSize::new(span.end as u32),
        );

        self.events.push(Event::Error { expected, range })
    }

    pub fn expect(&mut self, token: LexerToken) -> bool {
        if self.eat(token) {
            true
        } else {
            self.error(token.expected_str());
            false
        }
    }

    pub fn do_bump(&mut self, kind: SyntaxKind) {
        let span = self.span();
        let range = TextRange::new(
            TextSize::new(span.start as u32),
            TextSize::new(span.end as u32),
        );

        self.position += 1;
        self.events.push(Event::Token { kind, range })
    }
}

#[must_use = "A node stub must either be completed or abandoned"]
pub struct NodeStub {
    position: usize,
    bomb: DropBomb,
}

impl NodeStub {
    fn new(position: usize) -> Self {
        NodeStub {
            position,
            bomb: DropBomb::new("NodeStub must be either completed or abandoned"),
        }
    }

    pub fn complete(mut self, parser: &mut Parser, kind: SyntaxKind) -> CompletedNodeStub {
        self.bomb.defuse();
        assert!(matches!(&parser.events[self.position], Event::Stub));

        parser.events[self.position] = Event::StartNode { kind, parent: 0 };
        parser.events.push(Event::FinishNode);

        CompletedNodeStub::new(self.position, kind)
    }

    pub fn abandon(mut self, parser: &mut Parser) {
        self.bomb.defuse();

        if self.position == parser.events.len() - 1 {
            match parser.events.pop() {
                Some(Event::Stub) => {}
                _ => unreachable!(),
            }
        }
    }
}

pub struct CompletedNodeStub {
    position: usize,
    kind: SyntaxKind,
}

impl CompletedNodeStub {
    fn new(position: usize, kind: SyntaxKind) -> Self {
        CompletedNodeStub { position, kind }
    }

    pub fn precede(self, p: &mut Parser) -> NodeStub {
        let stub = p.start();

        match &mut p.events[self.position] {
            Event::StartNode { parent, .. } => *parent = stub.position - self.position,
            _ => unreachable!(),
        }

        stub
    }

    pub fn kind(&self) -> SyntaxKind {
        self.kind
    }
}
