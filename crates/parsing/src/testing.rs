macro_rules! make_event {
    ((start $kind:expr, $parent:expr)) => {
        ast::event::Event::StartNode {
            kind: $kind,
            parent: $parent,
        }
    };
    ((start $kind:expr)) => {
        ast::event::Event::StartNode {
            kind: $kind,
            parent: 0,
        }
    };
    (finish) => {
        ast::event::Event::FinishNode
    };
    ((token $kind:expr)) => {
        ast::event::Event::Token {
            kind: $kind,
            range: Default::default(),
        }
    };
    (stub) => {
        ast::event::Event::Stub
    };
}

macro_rules! expect_events {
    ($input:literal, $( $event:tt ),+ $(,)? ) => {{
        let mut p = $crate::Parser::new($input);
        parse(&mut p);
        for (idx, (actual, expected)) in p.finish().into_iter().zip([
            $( $crate::testing::make_event!($event) ),+
        ].into_iter()).enumerate() {
            match (actual, expected) {
                (
                    ast::event::Event::Token { kind: actual, .. },
                    ast::event::Event::Token { kind: expected, .. },
                ) => {
                    assert_eq!(actual, expected, "At Item: {}", idx)
                },
                (actual, expected) => assert_eq!(actual, expected, "At Item: {}", idx),
            }
        }
    }};
}

pub(crate) use expect_events;
pub(crate) use make_event;
