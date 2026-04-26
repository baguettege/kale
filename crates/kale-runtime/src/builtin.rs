use crate::object::Builtin;

pub struct Lib {
    pub ident: &'static str,
    pub builtins: &'static [Builtin],
}

impl Lib {
    pub const fn new(ident: &'static str, builtins: &'static [Builtin]) -> Self {
        Self { ident, builtins }
    }
}

#[macro_export]
macro_rules! lib {
    ($ident:literal, $( $builtin:expr ),+ $(,)?) => {
        $crate::builtin::Lib::new(
            $ident,
            &[$($builtin),+],
        )
    };
}
