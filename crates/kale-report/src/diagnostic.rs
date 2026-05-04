use kale_syntax::span::Span;

pub trait Diagnostic {
    fn message(&self) -> String;
    fn span(&self) -> Span;
}

macro_rules! impl_diagnostic {
    ($error:path) => {
        impl Diagnostic for $error {
            fn message(&self) -> String {
                self.to_string()
            }
            
            fn span(&self) -> Span {
                self.span()
            }
        }
    };
}

impl_diagnostic!(kale_lexer::Error);
impl_diagnostic!(kale_parser::Error);
impl_diagnostic!(kale_interpreter::Error);
