mod from {
    use crate::object::*;

    macro_rules! immutable {
        ($expr:expr) => {
            std::rc::Rc::new($expr)
        };
    }

    macro_rules! mutable {
        ($expr:expr) => {
            std::rc::Rc::new(std::cell::RefCell::new($expr))
        };
    }

    macro_rules! impl_from {
        ($target:ty, $ty:ty, $value:ident => $expr:expr) => {
            impl From<$target> for $ty {
                fn from($value: $target) -> Self {
                    $expr
                }
            }
        };

        ($target:ty, $variant:ident) => {
            impl From<$target> for Object {
                fn from(value: $target) -> Self {
                    Self::$variant(value)
                }
            }
        };

        ($target:ty, $value:ident => $expr:expr) => {
            impl From<$target> for Object {
                fn from($value: $target) -> Self {
                    $expr
                }
            }
        };
    }

    impl_from!(f64, Num);
    impl_from!(bool, Bool);
    impl_from!(char, Char);
    impl_from!(Immutable<String>, Str);
    impl_from!(Mutable<Vec<Object>>, List);
    impl_from!(Immutable<Closure>, Closure);
    impl_from!(Mutable<Module>, Module);
    impl_from!(Immutable<BoundMethod>, Bound);
    impl_from!(Immutable<StructDef>, StructDef);
    impl_from!(Mutable<Struct>, Struct);
    impl_from!(Mutable<dyn NativeObject>, Native);
    impl_from!(NativeFn, NativeFn);

    impl_from!(String, s => Self::Str(immutable!(s)));
    impl_from!(&String, s => s.clone().into());
    impl_from!(&str, s => s.to_string().into());

    impl_from!(Vec<Object>, list => Self::List(mutable!(list)));
    impl_from!(&[Object], list => list.to_vec().into());

    impl_from!(Closure, closure => Self::Closure(immutable!(closure)));
    impl_from!(Module, module => Self::Module(mutable!(module)));
    impl_from!(BoundMethod, bound => Self::Bound(immutable!(bound)));
    impl_from!(StructDef, def => Self::StructDef(immutable!(def)));
    impl_from!(Struct, instance => Self::Struct(mutable!(instance)));
}

mod try_from {
    use crate::object::*;

    macro_rules! impl_try_from {
        ($variant:ident($target:ty), $ty:expr) => {
            impl TryFrom<Object> for $target {
                type Error = $crate::Error;

                fn try_from(object: Object) -> Result<Self, Self::Error> {
                    match object {
                        Object::$variant(value) => Ok(value),
                        _ => Err($crate::Error::type_mismatch($ty, object.ty())),
                    }
                }
            }
        };
    }

    impl_try_from!(Num(f64), Type::Num);
    impl_try_from!(Bool(bool), Type::Bool);
    impl_try_from!(Char(char), Type::Char);
    impl_try_from!(Str(Immutable<String>), Type::Str);
    impl_try_from!(List(Mutable<Vec<Object>>), Type::List);
    impl_try_from!(Closure(Immutable<Closure>), Type::Closure);
    impl_try_from!(Module(Mutable<Module>), Type::Module);
    impl_try_from!(Bound(Immutable<BoundMethod>), Type::BoundMethod);
    impl_try_from!(StructDef(Immutable<StructDef>), Type::StructDef);
    impl_try_from!(Struct(Mutable<Struct>), Type::Struct);
    impl_try_from!(Native(Mutable<dyn NativeObject>), Type::Native);
    impl_try_from!(NativeFn(NativeFn), Type::NativeFn);
}
