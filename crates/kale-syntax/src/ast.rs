macro_rules! node {
    (
        $ident:ident {
            $( $field:ident: $ty:ty ),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone)]
        pub struct $ident {
            $( pub $field: $ty ),*
        }

        impl $ident {
            pub fn new($( $field: $ty ),*) -> Self {
                Self { $($field),* }
            }
        }
    };
}

macro_rules! impl_from {
    ($enum:ident => $($node:ident),+ $(,)?) => {
        $(
            impl From<$node> for $enum {
                fn from(node: $node) -> Self {
                    Self::$node(node)
                }
            }
        )+
    };
}

mod types;
mod expr;
mod stmt;
mod pretty;

pub use types::*;
pub use expr::*;
pub use stmt::*;

#[derive(Debug, Clone)]
pub struct Program(pub Block);
