use crate::Type;
use ast::expression::Literal;

pub fn infer_literal(lit: Literal) -> Type {
    match lit {
        Literal::Boolean(_) => Type::Boolean,
        Literal::None(_) => Type::Option(Box::new(Type::Any)),
        Literal::String(_) => Type::String,
        Literal::Integer(_) => Type::Integer,
        Literal::Float(_) => Type::Float,
        Literal::Path(_) => todo!(),
    }
}
