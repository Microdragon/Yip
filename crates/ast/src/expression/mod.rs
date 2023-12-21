mod literal;

pub use literal::*;

use crate::macros::{ast_multi_node, ast_node};
use crate::{SyntaxKind, SyntaxToken};
use rowan::ast::{support, AstChildren, AstNode};
use text_size::TextRange;

ast_multi_node! {
    Expression,
    If(IfExpression) => SyntaxKind::IfExpression,
    Infix(InfixOperator) => SyntaxKind::InfixOperator,
    Unary(UnaryOperator) => SyntaxKind::UnaryOperator,
    Error(ErrorValue) => SyntaxKind::ErrorValue,
    Tuple(TupleExpression) => SyntaxKind::TupleExpression,
    List(ListExpression) => SyntaxKind::ListExpression,
    Literal(Literal) => SyntaxKind::Literal,
}

impl Expression {
    pub fn text_range(&self) -> TextRange {
        match self {
            Expression::If(x) => x.syntax().text_range(),
            Expression::Infix(x) => x.syntax().text_range(),
            Expression::Unary(x) => x.syntax().text_range(),
            Expression::Error(x) => x.syntax().text_range(),
            Expression::Tuple(x) => x.syntax().text_range(),
            Expression::List(x) => x.syntax().text_range(),
            Expression::Literal(x) => x.syntax().text_range(),
        }
    }
}

ast_node!(IfExpression, SyntaxKind::IfExpression);

impl IfExpression {
    pub fn condition(&self) -> Expression {
        support::child(&self.0).unwrap()
    }

    pub fn then(&self) -> Expression {
        support::children(&self.0).nth(1).unwrap()
    }

    pub fn else_(&self) -> Expression {
        support::children(&self.0).nth(2).unwrap()
    }

    pub fn text_range(&self) -> TextRange {
        self.0.text_range()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InfixOperatorType {
    OptionCoalescing,
    BitwiseAnd,
    LogicalAnd,
    BitwiseXor,
    BitwiseOr,
    LogicalOr,
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanEquals,
    GreaterThanEquals,
    ShiftLeft,
    ShiftRight,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
}

ast_node!(InfixOperator, SyntaxKind::InfixOperator);

impl InfixOperator {
    pub fn lhs(&self) -> Expression {
        support::child(&self.0).unwrap()
    }

    pub fn op(&self) -> SyntaxToken {
        support::token(&self.0, SyntaxKind::Punctuation).unwrap()
    }

    pub fn op_type(&self) -> InfixOperatorType {
        match self.op().text() {
            "??" => InfixOperatorType::OptionCoalescing,
            "&" => InfixOperatorType::BitwiseAnd,
            "&&" => InfixOperatorType::LogicalAnd,
            "^" => InfixOperatorType::BitwiseXor,
            "|" => InfixOperatorType::BitwiseOr,
            "||" => InfixOperatorType::LogicalOr,
            "==" => InfixOperatorType::Equals,
            "!=" => InfixOperatorType::NotEquals,
            "<" => InfixOperatorType::LessThan,
            ">" => InfixOperatorType::GreaterThan,
            "<=" => InfixOperatorType::LessThanEquals,
            ">=" => InfixOperatorType::GreaterThanEquals,
            "<<" => InfixOperatorType::ShiftLeft,
            ">>" => InfixOperatorType::ShiftRight,
            "+" => InfixOperatorType::Plus,
            "-" => InfixOperatorType::Minus,
            "*" => InfixOperatorType::Multiply,
            "/" => InfixOperatorType::Divide,
            "%" => InfixOperatorType::Modulo,
            _ => unreachable!(),
        }
    }

    pub fn rhs(&self) -> Expression {
        support::children(&self.0).nth(1).unwrap()
    }

    pub fn text_range(&self) -> TextRange {
        self.0.text_range()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperatorType {
    Plus,
    Minus,
    Not,
}

ast_node!(UnaryOperator, SyntaxKind::UnaryOperator);

impl UnaryOperator {
    pub fn op(&self) -> SyntaxToken {
        support::token(&self.0, SyntaxKind::Punctuation).unwrap()
    }

    pub fn op_type(&self) -> UnaryOperatorType {
        match self.op().text() {
            "+" => UnaryOperatorType::Plus,
            "-" => UnaryOperatorType::Minus,
            "!" => UnaryOperatorType::Not,
            _ => unreachable!(),
        }
    }

    pub fn inner(&self) -> Literal {
        support::child(&self.0).unwrap()
    }
}

ast_node!(ErrorValue, SyntaxKind::ErrorValue);

impl ErrorValue {
    pub fn inner(&self) -> Expression {
        support::child(&self.0).unwrap()
    }
}

ast_node!(TupleExpression, SyntaxKind::TupleExpression);

impl TupleExpression {
    pub fn expressions(&self) -> AstChildren<Expression> {
        support::children(&self.0)
    }
}

ast_node!(ListExpression, SyntaxKind::ListExpression);

impl ListExpression {
    pub fn expressions(&self) -> AstChildren<Expression> {
        support::children(&self.0)
    }
}
