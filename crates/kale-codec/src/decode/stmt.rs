use kale_syntax::ast::{Assign, Block, Expr, FnDef, If, Module, Return, Stmt, While};
use crate::decode::{Decode, Decoder};
use crate::{Error, Result};
use crate::tag::AstTag;

impl Decode for Stmt {
    fn decode(decoder: &mut Decoder) -> Result<Self> {
        Ok(match decoder.decode::<AstTag>()? {
            AstTag::Module => decoder.decode::<Module>()?.into(),
            AstTag::FnDef => decoder.decode::<FnDef>()?.into(),
            AstTag::Assign => decoder.decode::<Assign>()?.into(),
            AstTag::If => decoder.decode::<If>()?.into(),
            AstTag::While => decoder.decode::<While>()?.into(),
            AstTag::Return => decoder.decode::<Return>()?.into(),
            AstTag::Expr => decoder.decode::<Expr>()?.into(),
            tag => return Err(Error::UnknownTag(tag as u8)),
        })
    }
}

impl Decode for Block {
    fn decode(decoder: &mut Decoder) -> Result<Self> {
        Ok(Block(decoder.decode()?))
    }
}

impl Decode for Module {
    fn decode(decoder: &mut Decoder) -> Result<Self> {
        let ident = decoder.decode()?;
        let body = decoder.decode()?;
        Ok(Self::new(ident, body))
    }
}

impl Decode for FnDef {
    fn decode(decoder: &mut Decoder) -> Result<Self> {
        let ident = decoder.decode()?;
        let params = decoder.decode()?;
        let body = decoder.decode()?;
        Ok(Self::new(ident, params, body))
    }
}

impl Decode for Assign {
    fn decode(decoder: &mut Decoder) -> Result<Self> {
        let target = decoder.decode()?;
        let value = decoder.decode()?;
        Ok(Self::new(target, value))
    }
}

impl Decode for If {
    fn decode(decoder: &mut Decoder) -> Result<Self> {
        let cond = decoder.decode()?;
        let then_branch = decoder.decode()?;
        let else_branch = decoder.decode()?;
        Ok(Self::new(cond, then_branch, else_branch))
    }
}

impl Decode for While {
    fn decode(decoder: &mut Decoder) -> Result<Self> {
        let cond = decoder.decode()?;
        let body = decoder.decode()?;
        Ok(Self::new(cond, body))
    }
}

impl Decode for Return {
    fn decode(decoder: &mut Decoder) -> Result<Self> {
        let value = decoder.decode()?;
        Ok(Self::new(value))
    }
}
