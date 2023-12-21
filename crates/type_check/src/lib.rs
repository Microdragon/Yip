mod infix;
mod literal;
mod unary;

use ast::expression::{Expression, IfExpression};
use text_size::TextRange;

pub enum TypeError {
    NoCommonType {
        lhs: Type,
        rhs: Type,
        range: TextRange,
    },
    UnexpectedType {
        expected: Type,
        actual: Type,
        range: TextRange,
    },
    UnexpectedTypes {
        expected: &'static [Type],
        actual: Type,
        range: TextRange,
    },
}

type Result<T> = std::result::Result<T, TypeError>;

#[derive(PartialEq, Eq)]
pub enum Type {
    Any,
    Boolean,
    String,
    Integer,
    SignedInteger,
    Float,
    Option(Box<Type>),
    List(Box<Type>),
    Tuple(Vec<Type>),
    Result(Box<Type>, Box<Type>),
}

impl Type {
    fn requires_type(self, expected: Type, range: TextRange) -> Result<Type> {
        match common_type(expected, self, range) {
            Ok(ty) => Ok(ty),
            Err(TypeError::NoCommonType { lhs, rhs, range }) => Err(TypeError::UnexpectedType {
                expected: lhs,
                actual: rhs,
                range,
            }),
            Err(err) => Err(err),
        }
    }

    fn requires_types(self, expected: &'static [Type], range: TextRange) -> Result<Type> {
        match self {
            Type::Any => Ok(Type::Any),
            Type::Option(x) => Ok(Type::Option(Box::new(x.requires_types(expected, range)?))),
            Type::List(x) => Ok(Type::List(Box::new(x.requires_types(expected, range)?))),
            Type::Result(x, error) => Ok(Type::Result(
                Box::new(x.requires_types(expected, range)?),
                error,
            )),
            actual => {
                if expected.contains(&actual) {
                    Ok(actual)
                } else {
                    Err(TypeError::UnexpectedTypes {
                        expected,
                        actual,
                        range,
                    })
                }
            }
        }
    }
}

pub fn infer_type(expr: Expression) -> Result<Type> {
    match expr {
        Expression::If(expr) => infer_if(expr),
        Expression::Infix(infix) => infix::infer_infix(infix),
        Expression::Unary(unary) => unary::infer_unary(unary),
        Expression::Error(error) => unary::infer_error(error),
        Expression::Tuple(tuple) => unary::infer_tuple(tuple),
        Expression::List(list) => unary::infer_list(list),
        Expression::Literal(lit) => Ok(literal::infer_literal(lit)),
    }
}

fn infer_if(expr: IfExpression) -> Result<Type> {
    let condition = expr.condition();
    let condition_range = condition.text_range();
    infer_type(condition)?.requires_type(Type::Boolean, condition_range)?;

    let then = infer_type(expr.then())?;
    let else_ = infer_type(expr.else_())?;

    common_type(then, else_, expr.text_range())
}

fn common_type(lhs: Type, rhs: Type, range: TextRange) -> Result<Type> {
    if lhs == rhs {
        Ok(lhs)
    } else {
        match (lhs, rhs) {
            // Any can be coerced to any other type
            (Type::Any, rhs) => Ok(rhs),
            (lhs, Type::Any) => Ok(lhs),

            // Integer can be coerced to SignedInteger type
            (Type::SignedInteger, Type::Integer) => Ok(Type::SignedInteger),
            (Type::Integer, Type::SignedInteger) => Ok(Type::SignedInteger),

            // Option type should be handled transparently
            (Type::Option(lhs), Type::Option(rhs)) => {
                Ok(Type::Option(Box::new(common_type(*lhs, *rhs, range)?)))
            }
            (Type::Option(inner), rhs) => {
                Ok(Type::Option(Box::new(common_type(*inner, rhs, range)?)))
            }
            (lhs, Type::Option(inner)) => {
                Ok(Type::Option(Box::new(common_type(lhs, *inner, range)?)))
            }

            // Result type should be handled transparently
            (Type::Result(lhs_inner, lhs_error), Type::Result(rhs_inner, rhs_error)) => {
                Ok(Type::Result(
                    Box::new(common_type(*lhs_inner, *rhs_inner, range)?),
                    Box::new(common_type(*lhs_error, *rhs_error, range)?),
                ))
            }
            (Type::Result(inner, error), rhs) => Ok(Type::Result(
                Box::new(common_type(*inner, rhs, range)?),
                error,
            )),
            (lhs, Type::Result(inner, error)) => Ok(Type::Result(
                Box::new(common_type(lhs, *inner, range)?),
                error,
            )),

            (lhs, rhs) => Err(TypeError::NoCommonType { lhs, rhs, range }),
        }
    }
}
