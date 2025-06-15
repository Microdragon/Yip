/// The kinds of nodes and tokens making up the AST.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
pub enum SyntaxKind {
    EndOfFile,
    Whitespace,
    Newline,
    Keyword,
    Punctuation,
    Identifier,
    String,
    Integer,
    Float,

    IfExpression,
    InfixOperator,
    UnaryOperator,
    TupleExpression,
    ListExpression,
    Literal,
    Boolean,
    NoneValue,
    ErrorValue,

    Annotation,

    ImportDirective,
    PluginDirective,
    UseDirective,
    AliasDefinition,
    HandleDefinition,
    ModuleDefinition,
    ConstDefinition,

    StructMember,
    StructDefinition,

    UnionDefinition,
    UnionMember,
    UnionAnonymousStruct,

    EnumDefinition,
    EnumMember,

    InterfaceDefinition,
    InterfaceFunction,
    InterfaceFunctionParameter,

    NamedType,
    ListType,
    TupleType,
    OptionType,
    ResultType,
    SendType,
    RecvType,

    File,
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(value: SyntaxKind) -> Self {
        Self(value as u16)
    }
}
