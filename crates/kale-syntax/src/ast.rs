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
                    $enum::$node(node)
                }
            }
        )+
    };
}

mod stmt;
mod expr;
mod types;
mod display;

pub use types::*;
pub use stmt::*;
pub use expr::*;
