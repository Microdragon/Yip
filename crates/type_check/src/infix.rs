use crate::{common_type, infer_type, Result, Type, TypeError};
use ast::expression::{InfixOperator, InfixOperatorType};
use text_size::TextRange;

pub fn infer_infix(infix: InfixOperator) -> Result<Type> {
    let lhs = infix.lhs();
    let lhs_range = lhs.text_range();
    let lhs = infer_type(lhs)?;

    let rhs = infix.rhs();
    let rhs_range = rhs.text_range();
    let rhs = infer_type(rhs)?;

    match infix.op_type() {
        InfixOperatorType::OptionCoalescing => {
            infer_option_coalescing(lhs, rhs, lhs_range, rhs_range)
        }
        InfixOperatorType::BitwiseAnd
        | InfixOperatorType::BitwiseXor
        | InfixOperatorType::BitwiseOr => {
            let lhs = lhs.requires_type(Type::Integer, lhs_range)?;
            let rhs = rhs.requires_type(Type::Integer, rhs_range)?;
            common_type(lhs, rhs, infix.text_range())
        }
        InfixOperatorType::LogicalAnd | InfixOperatorType::LogicalOr => {
            let lhs = lhs.requires_type(Type::Boolean, lhs_range)?;
            let rhs = rhs.requires_type(Type::Boolean, rhs_range)?;
            common_type(lhs, rhs, infix.text_range())
        }
        InfixOperatorType::Equals | InfixOperatorType::NotEquals => {
            common_type(lhs, rhs, infix.text_range())?;
            Ok(Type::Boolean)
        }
        InfixOperatorType::LessThan
        | InfixOperatorType::GreaterThan
        | InfixOperatorType::LessThanEquals
        | InfixOperatorType::GreaterThanEquals
        | InfixOperatorType::Minus
        | InfixOperatorType::Divide
        | InfixOperatorType::Modulo => {
            const TYPES: &[Type] = &[Type::Integer, Type::SignedInteger, Type::Float];
            let lhs = lhs.requires_types(TYPES, lhs_range)?;
            let rhs = rhs.requires_types(TYPES, rhs_range)?;
            common_type(lhs, rhs, infix.text_range())
        }
        InfixOperatorType::ShiftLeft | InfixOperatorType::ShiftRight => {
            let lhs = lhs.requires_type(Type::Integer, lhs_range)?;
            rhs.requires_type(Type::Integer, rhs_range)?;
            Ok(lhs)
        }
        InfixOperatorType::Plus => {
            const TYPES: &[Type] = &[
                Type::String,
                Type::Integer,
                Type::SignedInteger,
                Type::Float,
            ];
            let lhs = lhs.requires_types(TYPES, lhs_range)?;
            let rhs = rhs.requires_types(TYPES, rhs_range)?;
            common_type(lhs, rhs, infix.text_range())
        }
        InfixOperatorType::Multiply => todo!(),
    }
}

fn infer_option_coalescing(
    lhs: Type,
    rhs: Type,
    lhs_range: TextRange,
    rhs_range: TextRange,
) -> Result<Type> {
    let Type::Option(inner) = lhs else {
        return Err(TypeError::UnexpectedType { expected: Type::Option(Box::new(Type::Any)), actual: lhs, range: lhs_range });
    };

    common_type(*inner, rhs, rhs_range)
}
