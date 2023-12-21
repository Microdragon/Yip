use crate::literal::infer_literal;
use crate::{infer_type, Result, Type, common_type};
use ast::expression::{
    ErrorValue, ListExpression, TupleExpression, UnaryOperator, UnaryOperatorType,
};

const UNARY_PLUS_MINUS_TYPES: &[Type] = &[Type::Integer, Type::SignedInteger, Type::Float];
const UNARY_NOT_TYPES: &[Type] = &[
    Type::Boolean,
    Type::Integer,
    Type::SignedInteger,
    Type::Float,
];

pub fn infer_unary(unary: UnaryOperator) -> Result<Type> {
    let inner = unary.inner();
    let range = inner.text_range();
    let inner = infer_literal(inner);

    match unary.op_type() {
        UnaryOperatorType::Plus => inner.requires_types(UNARY_PLUS_MINUS_TYPES, range),
        UnaryOperatorType::Not => inner.requires_types(UNARY_NOT_TYPES, range),
        UnaryOperatorType::Minus => {
            let inner = inner.requires_types(UNARY_PLUS_MINUS_TYPES, range)?;
            if inner == Type::Integer {
                Ok(Type::SignedInteger)
            } else {
                Ok(inner)
            }
        }
    }
}

pub fn infer_error(error: ErrorValue) -> Result<Type> {
    let ty = infer_type(error.inner())?;
    Ok(Type::Result(Box::new(Type::Any), Box::new(ty)))
}

pub fn infer_tuple(tuple: TupleExpression) -> Result<Type> {
    let mut types = Vec::new();
    for expr in tuple.expressions() {
        types.push(infer_type(expr)?);
    }

    Ok(Type::Tuple(types))
}

pub fn infer_list(list: ListExpression) -> Result<Type> {
    let mut iter = list.expressions();
    let Some(common) = iter.next() else {
        return Ok(Type::List(Box::new(Type::Any)));
    };
    let mut common = infer_type(common)?;

    for item in iter {
        let range = item.text_range();
        let ty = infer_type(item)?;
        common = common_type(common, ty, range)?;
    }

    Ok(common)
}
