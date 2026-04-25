use crate::encode::{Encode, Encoder};

impl<T: Encode> Encode for Vec<T> {
    fn encode(&self, encoder: &mut Encoder) {
        encoder.encode(&self.len());
        for element in self {
            encoder.encode(element);
        }
    }
}

impl<T: Encode> Encode for Option<T> {
    fn encode(&self, encoder: &mut Encoder) {
        if let Some(value) = self {
            encoder
                .encode(&true)
                .encode(value);
        } else {
            encoder.encode(&false);
        }
    }
}
