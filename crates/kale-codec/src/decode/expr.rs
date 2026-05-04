use crate::decode::{Decode, Decoder};
use crate::tag::AstTag;
use crate::{Error, Result};
use kale_syntax::ast::{Binary, Call, Closure, Expr, ExprKind, Ident, List, Literal, Member, Unary};

impl Decode for ExprKind {
    fn decode(decoder: &mut Decoder) -> Result<Self> {
        Ok(match decoder.decode::<AstTag>()? {
            AstTag::Ident => decoder.decode::<Ident>()?.into(),
            AstTag::Literal => decoder.decode::<Literal>()?.into(),
            AstTag::Call => decoder.decode::<Call>()?.into(),
            AstTag::Binary => decoder.decode::<Binary>()?.into(),
            AstTag::Unary => decoder.decode::<Unary>()?.into(),
            AstTag::List => decoder.decode::<List>()?.into(),
            AstTag::Closure => decoder.decode::<Closure>()?.into(),
            AstTag::Member => decoder.decode::<Member>()?.into(),
            tag => return Err(Error::UnknownTag(tag as u8)),
        })
    }
}

impl Decode for Box<Expr> {
    fn decode(decoder: &mut Decoder) -> Result<Self> {
        Ok(Self::new(decoder.decode()?))
    }
}

impl Decode for Call {
    fn decode(decoder: &mut Decoder) -> Result<Self> {
        let callee = decoder.decode()?;
        let args = decoder.decode()?;
        Ok(Self::new(callee, args))
    }
}

impl Decode for Binary {
    fn decode(decoder: &mut Decoder) -> Result<Self> {
        let lhs = decoder.decode()?;
        let op = decoder.decode()?;
        let rhs = decoder.decode()?;
        Ok(Self::new(lhs, op, rhs))
    }
}

impl Decode for Unary {
    fn decode(decoder: &mut Decoder) -> Result<Self> {
        let op = decoder.decode()?;
        let expr = decoder.decode()?;
        Ok(Self::new(op, expr))
    }
}

impl Decode for List {
    fn decode(decoder: &mut Decoder) -> Result<Self> {
        let elements = decoder.decode()?;
        Ok(Self::new(elements))
    }
}

impl Decode for Closure {
    fn decode(decoder: &mut Decoder) -> Result<Self> {
        let params = decoder.decode()?;
        let body = decoder.decode()?;
        Ok(Self::new(params, body))
    }
}

impl Decode for Member {
    fn decode(decoder: &mut Decoder) -> Result<Self> {
        let object = decoder.decode()?;
        let property = decoder.decode()?;
        Ok(Self::new(object, property))
    }
}
