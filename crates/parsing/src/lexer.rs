use ast::SyntaxKind;
use logos::Logos;

#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq)]
#[logos(skip r"//[^\n]*\n?")]
pub enum LexerToken {
    EndOfFile,
    #[regex(r"[ \t]+")]
    Whitespace,
    #[regex(r"\n")]
    Newline,
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
    #[regex(r#""(?:[^"\\]|\\"|\\\\|\\u[0-9a-fA-F]{4})*""#)]
    #[regex(r"'(?:[^'\\]|\\'|\\\\|\\u[0-9a-fA-F]{4})*'")]
    String,
    #[regex(r"[0-9]+")]
    Integer,
    #[regex(r"[0-9]+[.][0-9]+(?:[eE][+-]?[0-9]+)?")]
    Float,
    #[token(":")]
    Colon,
    #[token("::")]
    ColonColon,
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,
    #[token("@")]
    AtSign,
    #[token("=")]
    Equals,
    #[token("==")]
    EqualsEquals,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("!")]
    Bang,
    #[token("!=")]
    BangEquals,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token("<")]
    LessThan,
    #[token("<<")]
    LessThanLessThan,
    #[token("<=")]
    LessThanEquals,
    #[token(">")]
    GreaterThan,
    #[token(">>")]
    GreaterThanGreaterThan,
    #[token(">=")]
    GreaterThanEquals,
    #[token("&")]
    Ampersand,
    #[token("&&")]
    AmpersandAmpersand,
    #[token("^")]
    Caret,
    #[token("|")]
    Bar,
    #[token("||")]
    BarBar,
    #[token("?")]
    Question,
    #[token("??")]
    QuestionQuestion,

    #[token("(")]
    LParentheses,
    #[token(")")]
    RParentheses,
    #[token("{")]
    LBraces,
    #[token("}")]
    RBraces,
    #[token("[")]
    LBrackets,
    #[token("]")]
    RBrackets,

    #[token("true")]
    KeywordTrue,
    #[token("false")]
    KeywordFalse,
    #[token("import")]
    KeywordImport,
    #[token("as")]
    KeywordAs,
    #[token("plugin")]
    KeywordPlugin,
    #[token("type")]
    KeywordType,
    #[token("struct")]
    KeywordStruct,
    #[token("union")]
    KeywordUnion,
    #[token("enum")]
    KeywordEnum,
    #[token("result")]
    KeywordResult,
    #[token("error")]
    KeywordError,
    #[token("send")]
    KeywordSend,
    #[token("recv")]
    KeywordRecv,
    #[token("interface")]
    KeywordInterface,
    #[token("fn")]
    KeywordFn,
    #[token("const")]
    KeywordConst,
    #[token("mod")]
    KeywordMod,
    #[token("use")]
    KeywordUse,
    #[token("if")]
    KeywordIf,
    #[token("then")]
    KeywordThen,
    #[token("else")]
    KeywordElse,
    #[token("none")]
    KeywordNone,
}

impl LexerToken {
    pub fn expected_str(self) -> &'static str {
        match self {
            LexerToken::EndOfFile => "End of File",
            LexerToken::Whitespace => "Spacing",
            LexerToken::Newline => "Newline",
            LexerToken::Identifier => "Identifier",
            LexerToken::String => "String",
            LexerToken::Integer => "Integer",
            LexerToken::Float => "Float",
            LexerToken::Colon => ":",
            LexerToken::ColonColon => "::",
            LexerToken::Semicolon => ";",
            LexerToken::Comma => ",",
            LexerToken::AtSign => "@",
            LexerToken::Equals => "=",
            LexerToken::EqualsEquals => "==",
            LexerToken::Plus => "+",
            LexerToken::Minus => "-",
            LexerToken::Bang => "!",
            LexerToken::BangEquals => "!=",
            LexerToken::Star => "*",
            LexerToken::Slash => "/",
            LexerToken::Percent => "%",
            LexerToken::LessThan => "<",
            LexerToken::LessThanLessThan => "<<",
            LexerToken::LessThanEquals => "<=",
            LexerToken::GreaterThan => ">",
            LexerToken::GreaterThanGreaterThan => ">>",
            LexerToken::GreaterThanEquals => ">=",
            LexerToken::Ampersand => "&",
            LexerToken::AmpersandAmpersand => "&&",
            LexerToken::Caret => "^",
            LexerToken::Bar => "|",
            LexerToken::BarBar => "||",
            LexerToken::Question => "?",
            LexerToken::QuestionQuestion => "??",
            LexerToken::LParentheses => "(",
            LexerToken::RParentheses => ")",
            LexerToken::LBraces => "{",
            LexerToken::RBraces => "}",
            LexerToken::LBrackets => "[",
            LexerToken::RBrackets => "]",
            LexerToken::KeywordTrue => "true",
            LexerToken::KeywordFalse => "false",
            LexerToken::KeywordImport => "import",
            LexerToken::KeywordAs => "as",
            LexerToken::KeywordPlugin => "plugin",
            LexerToken::KeywordType => "type",
            LexerToken::KeywordStruct => "struct",
            LexerToken::KeywordUnion => "union",
            LexerToken::KeywordEnum => "enum",
            LexerToken::KeywordResult => "result",
            LexerToken::KeywordError => "error",
            LexerToken::KeywordSend => "send",
            LexerToken::KeywordRecv => "recv",
            LexerToken::KeywordInterface => "interface",
            LexerToken::KeywordFn => "fn",
            LexerToken::KeywordConst => "const",
            LexerToken::KeywordMod => "mod",
            LexerToken::KeywordUse => "use",
            LexerToken::KeywordIf => "if",
            LexerToken::KeywordThen => "then",
            LexerToken::KeywordElse => "else",
            LexerToken::KeywordNone => "none",
        }
    }
}

impl From<LexerToken> for SyntaxKind {
    fn from(value: LexerToken) -> Self {
        match value {
            LexerToken::EndOfFile => SyntaxKind::EndOfFile,
            LexerToken::Whitespace => SyntaxKind::Whitespace,
            LexerToken::Newline => SyntaxKind::Newline,
            LexerToken::Identifier => SyntaxKind::Identifier,
            LexerToken::String => SyntaxKind::String,
            LexerToken::Integer => SyntaxKind::Integer,
            LexerToken::Float => SyntaxKind::Float,
            LexerToken::Colon
            | LexerToken::ColonColon
            | LexerToken::Semicolon
            | LexerToken::Comma
            | LexerToken::AtSign
            | LexerToken::Equals
            | LexerToken::EqualsEquals
            | LexerToken::Plus
            | LexerToken::Minus
            | LexerToken::Bang
            | LexerToken::BangEquals
            | LexerToken::Star
            | LexerToken::Slash
            | LexerToken::Percent
            | LexerToken::LessThan
            | LexerToken::LessThanLessThan
            | LexerToken::LessThanEquals
            | LexerToken::GreaterThan
            | LexerToken::GreaterThanGreaterThan
            | LexerToken::GreaterThanEquals
            | LexerToken::Ampersand
            | LexerToken::AmpersandAmpersand
            | LexerToken::Caret
            | LexerToken::Bar
            | LexerToken::BarBar
            | LexerToken::Question
            | LexerToken::QuestionQuestion
            | LexerToken::LParentheses
            | LexerToken::RParentheses
            | LexerToken::LBraces
            | LexerToken::RBraces
            | LexerToken::LBrackets
            | LexerToken::RBrackets => SyntaxKind::Punctuation,
            LexerToken::KeywordTrue
            | LexerToken::KeywordFalse
            | LexerToken::KeywordImport
            | LexerToken::KeywordAs
            | LexerToken::KeywordPlugin
            | LexerToken::KeywordType
            | LexerToken::KeywordStruct
            | LexerToken::KeywordUnion
            | LexerToken::KeywordEnum
            | LexerToken::KeywordResult
            | LexerToken::KeywordError
            | LexerToken::KeywordSend
            | LexerToken::KeywordRecv
            | LexerToken::KeywordInterface
            | LexerToken::KeywordFn
            | LexerToken::KeywordConst
            | LexerToken::KeywordMod
            | LexerToken::KeywordUse
            | LexerToken::KeywordIf
            | LexerToken::KeywordThen
            | LexerToken::KeywordElse
            | LexerToken::KeywordNone => SyntaxKind::Keyword,
        }
    }
}

pub struct LexerTokenSet(u64);

impl LexerTokenSet {
    pub const EMPTY: LexerTokenSet = LexerTokenSet(0);

    pub const fn new(tokens: &[LexerToken]) -> Self {
        let mut result = 0;

        let mut idx = 0;
        while idx < tokens.len() {
            result |= mask(tokens[idx]);
            idx += 1;
        }

        LexerTokenSet(result)
    }

    pub const fn contains(&self, token: LexerToken) -> bool {
        (self.0 & mask(token)) != 0
    }
}

const fn mask(token: LexerToken) -> u64 {
    1 << token as usize
}

#[cfg(test)]
mod test {
    use crate::LexerToken;
    use logos::Logos;

    macro_rules! expect_tokens {
        ($input:literal, [ $($item:expr),+ $(,)? ]) => {{
            let mut actual = Vec::new();
            let mut lexer = LexerToken::lexer($input);
            while let Some(token) = lexer.next() {
                match token {
                    Ok(token) => actual.push(token),
                    Err(()) => panic!("Unrecognized input: {}", lexer.remainder()),
                }
            }
            assert_eq!(actual, &[$($item),+]);
        }};
    }

    #[test]
    fn test_comment() {
        expect_tokens!("//this is a comment\ntest", [LexerToken::Identifier]);
    }

    #[test]
    fn test_identifier() {
        expect_tokens!(
            "hello world",
            [
                LexerToken::Identifier,
                LexerToken::Whitespace,
                LexerToken::Identifier,
            ]
        );
    }

    #[test]
    fn test_string() {
        expect_tokens!("\"This is a string!\"", [LexerToken::String]);
        expect_tokens!("'This is a also string!'", [LexerToken::String]);

        expect_tokens!("\"This is a string with a \\\"!\"", [LexerToken::String]);
        expect_tokens!("'This is a also string with a \\'!'", [LexerToken::String]);

        expect_tokens!("\"This is a string with a \\\\!\"", [LexerToken::String]);
        expect_tokens!("'This is a also string with a \\\\!'", [LexerToken::String]);

        expect_tokens!("\"This is a string with a \\u0000!\"", [LexerToken::String]);
        expect_tokens!(
            "'This is a also string with a \\u0000!'",
            [LexerToken::String]
        );
    }

    #[test]
    fn test_integer() {
        expect_tokens!(
            "123 456 789",
            [
                LexerToken::Integer,
                LexerToken::Whitespace,
                LexerToken::Integer,
                LexerToken::Whitespace,
                LexerToken::Integer
            ]
        );
    }

    #[test]
    fn test_float() {
        expect_tokens!(
            "123.456 789.0e+5 144.0e9 33.0e-15 0.55",
            [
                LexerToken::Float,
                LexerToken::Whitespace,
                LexerToken::Float,
                LexerToken::Whitespace,
                LexerToken::Float,
                LexerToken::Whitespace,
                LexerToken::Float,
                LexerToken::Whitespace,
                LexerToken::Float
            ]
        );
    }

    #[test]
    fn test_punctuation() {
        expect_tokens!(
            ": :: , @ = == + - ! != * / % < << <= > >> >= & && ^ | || ?? ( ) { } [ ]",
            [
                LexerToken::Colon,
                LexerToken::Whitespace,
                LexerToken::ColonColon,
                LexerToken::Whitespace,
                LexerToken::Comma,
                LexerToken::Whitespace,
                LexerToken::AtSign,
                LexerToken::Whitespace,
                LexerToken::Equals,
                LexerToken::Whitespace,
                LexerToken::EqualsEquals,
                LexerToken::Whitespace,
                LexerToken::Plus,
                LexerToken::Whitespace,
                LexerToken::Minus,
                LexerToken::Whitespace,
                LexerToken::Bang,
                LexerToken::Whitespace,
                LexerToken::BangEquals,
                LexerToken::Whitespace,
                LexerToken::Star,
                LexerToken::Whitespace,
                LexerToken::Slash,
                LexerToken::Whitespace,
                LexerToken::Percent,
                LexerToken::Whitespace,
                LexerToken::LessThan,
                LexerToken::Whitespace,
                LexerToken::LessThanLessThan,
                LexerToken::Whitespace,
                LexerToken::LessThanEquals,
                LexerToken::Whitespace,
                LexerToken::GreaterThan,
                LexerToken::Whitespace,
                LexerToken::GreaterThanGreaterThan,
                LexerToken::Whitespace,
                LexerToken::GreaterThanEquals,
                LexerToken::Whitespace,
                LexerToken::Ampersand,
                LexerToken::Whitespace,
                LexerToken::AmpersandAmpersand,
                LexerToken::Whitespace,
                LexerToken::Caret,
                LexerToken::Whitespace,
                LexerToken::Bar,
                LexerToken::Whitespace,
                LexerToken::BarBar,
                LexerToken::Whitespace,
                LexerToken::QuestionQuestion,
                LexerToken::Whitespace,
                LexerToken::LParentheses,
                LexerToken::Whitespace,
                LexerToken::RParentheses,
                LexerToken::Whitespace,
                LexerToken::LBraces,
                LexerToken::Whitespace,
                LexerToken::RBraces,
                LexerToken::Whitespace,
                LexerToken::LBrackets,
                LexerToken::Whitespace,
                LexerToken::RBrackets
            ]
        );
    }

    #[test]
    fn test_keyword() {
        expect_tokens!(
            "true false import plugin type struct union enum result error send recv interface fn const mod use if then else none",
            [
                LexerToken::KeywordTrue,
                LexerToken::Whitespace,
                LexerToken::KeywordFalse,
                LexerToken::Whitespace,
                LexerToken::KeywordImport,
                LexerToken::Whitespace,
                LexerToken::KeywordPlugin,
                LexerToken::Whitespace,
                LexerToken::KeywordType,
                LexerToken::Whitespace,
                LexerToken::KeywordStruct,
                LexerToken::Whitespace,
                LexerToken::KeywordUnion,
                LexerToken::Whitespace,
                LexerToken::KeywordEnum,
                LexerToken::Whitespace,
                LexerToken::KeywordResult,
                LexerToken::Whitespace,
                LexerToken::KeywordError,
                LexerToken::Whitespace,
                LexerToken::KeywordSend,
                LexerToken::Whitespace,
                LexerToken::KeywordRecv,
                LexerToken::Whitespace,
                LexerToken::KeywordInterface,
                LexerToken::Whitespace,
                LexerToken::KeywordFn,
                LexerToken::Whitespace,
                LexerToken::KeywordConst,
                LexerToken::Whitespace,
                LexerToken::KeywordMod,
                LexerToken::Whitespace,
                LexerToken::KeywordUse,
                LexerToken::Whitespace,
                LexerToken::KeywordIf,
                LexerToken::Whitespace,
                LexerToken::KeywordThen,
                LexerToken::Whitespace,
                LexerToken::KeywordElse,
                LexerToken::Whitespace,
                LexerToken::KeywordNone
            ]
        );
    }

    #[test]
    fn test_struct() {
        expect_tokens!(
            r"struct Test {
                a: bool
                b: [int]
                c: (bool, bool)
            }",
            [
                LexerToken::KeywordStruct,
                LexerToken::Whitespace,
                LexerToken::Identifier,
                LexerToken::Whitespace,
                LexerToken::LBraces,
                LexerToken::Newline,
                LexerToken::Whitespace,
                LexerToken::Identifier,
                LexerToken::Colon,
                LexerToken::Whitespace,
                LexerToken::Identifier,
                LexerToken::Newline,
                LexerToken::Whitespace,
                LexerToken::Identifier,
                LexerToken::Colon,
                LexerToken::Whitespace,
                LexerToken::LBrackets,
                LexerToken::Identifier,
                LexerToken::RBrackets,
                LexerToken::Newline,
                LexerToken::Whitespace,
                LexerToken::Identifier,
                LexerToken::Colon,
                LexerToken::Whitespace,
                LexerToken::LParentheses,
                LexerToken::Identifier,
                LexerToken::Comma,
                LexerToken::Whitespace,
                LexerToken::Identifier,
                LexerToken::RParentheses,
                LexerToken::Newline,
                LexerToken::Whitespace,
                LexerToken::RBraces,
            ]
        )
    }
}
