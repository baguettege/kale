use crate::decode::{Decode, Decoder};
use crate::Result;

impl<T: Decode> Decode for Vec<T> {
    fn decode(decoder: &mut Decoder) -> Result<Self> {
        let len = decoder.decode::<usize>()?;
        (0..len).map(|_| decoder.decode::<T>()).collect()
    }
}

impl<T: Decode> Decode for Option<T> {
    fn decode(decoder: &mut Decoder) -> Result<Self> {
        Ok(if decoder.decode::<bool>()? {
            Some(decoder.decode::<T>()?)
        } else {
            None
        })
    }
}
