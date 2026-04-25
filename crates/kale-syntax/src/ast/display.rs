mod types {
    use std::fmt::{Display, Formatter, Result};
    use crate::ast::{BinOp, Literal, UnOp};

    impl Display for BinOp {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                BinOp::Mul => write!(f, "*"),
                BinOp::Div => write!(f, "/"),
                BinOp::Add => write!(f, "+"),
                BinOp::Sub => write!(f, "-"),
                BinOp::Lt => write!(f, "<"),
                BinOp::Le => write!(f, "<="),
                BinOp::Gt => write!(f, ">"),
                BinOp::Ge => write!(f, ">="),
                BinOp::Eq => write!(f, "=="),
                BinOp::Ne => write!(f, "!="),
                BinOp::And => write!(f, "and"),
                BinOp::Or => write!(f, "or"),
            }
        }
    }

    impl Display for UnOp {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                UnOp::Not => write!(f, "not"),
                UnOp::Neg => write!(f, "-"),
            }
        }
    }

    impl Display for Literal {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                Literal::Nil => write!(f, "nil"),
                Literal::Num(val) => write!(f, "{val}"),
                Literal::Bool(val) => write!(f, "{val}"),
                Literal::Str(val) => write!(f, "\"{val}\""),
            }
        }
    }
}

mod expr {
    use std::fmt::{Display, Formatter, Result};
    use crate::ast::{Binary, Call, Closure, Expr, Ident, Index, List, Member, Unary};

    impl Display for Expr {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                Expr::Literal(node) => write!(f, "{node}"),
                Expr::Ident(node) => write!(f, "{node}"),
                Expr::Call(node) => write!(f, "{node}"),
                Expr::Binary(node) => write!(f, "{node}"),
                Expr::Unary(node) => write!(f, "{node}"),
                Expr::List(node) => write!(f, "{node}"),
                Expr::Closure(node) => write!(f, "{node}"),
                Expr::Member(node) => write!(f, "{node}"),
                Expr::Index(node) => write!(f, "{node}"),
            }
        }
    }

    impl Display for Call {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            let args = self.args
                .iter()
                .map(|arg| format!("{arg}"))
                .collect::<Vec<_>>()
                .join(", ");
            write!(f, "{}({args})", self.callee)
        }
    }

    impl Display for Binary {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{} {} {}", self.lhs, self.op, self.rhs)
        }
    }

    impl Display for Unary {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{} {}", self.op, self.expr)
        }
    }

    impl Display for List {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            let elements = self.elements
                .iter()
                .map(|elem| format!("{elem}"))
                .collect::<Vec<_>>()
                .join(", ");
            write!(f, "[{elements}]")
        }
    }

    impl Display for Closure {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            let params = self.params
                .iter()
                .map(Ident::as_str)
                .collect::<Vec<_>>()
                .join(", ");
            write!(f, "fn({params}) {}", self.body)
        }
    }

    impl Display for Member {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{}.{}", self.object, self.property)
        }
    }

    impl Display for Index {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{}[{}]", self.object, self.index)
        }
    }
}

mod stmt {
    use std::fmt::{Display, Formatter, Result};
    use crate::ast::{Assign, Block, Expr, FnDef, Ident, If, Module, Return, Stmt, While};

    struct Printer {
        indent: usize,
    }

    impl Printer {
        fn new() -> Self {
            Self { indent: 0 }
        }

        fn write_indent(&mut self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{}", "    ".repeat(self.indent))
        }

        fn with_indent<F>(&mut self, f: F) -> Result
        where
            F: FnOnce(&mut Self) -> Result,
        {
            self.indent += 1;
            let result = f(self);
            self.indent -= 1;
            result
        }

        fn print_stmt(&mut self, stmt: &Stmt, f: &mut Formatter<'_>) -> Result {
            self.write_indent(f)?;
            match stmt {
                Stmt::Expr(node) => self.print_expr(node, f),
                Stmt::Module(node) => self.print_module(node, f),
                Stmt::FnDef(node) => self.print_fndef(node, f),
                Stmt::Assign(node) => self.print_assign(node, f),
                Stmt::If(node) => self.print_if(node, f),
                Stmt::While(node) => self.print_while(node, f),
                Stmt::Return(node) => self.print_return(node, f),
            }
        }

        fn print_block(&mut self, block: &Block, f: &mut Formatter<'_>) -> Result {
            writeln!(f, "{{")?;

            self.with_indent(|this| {
                for stmt in &block.0 {
                    this.print_stmt(stmt, f)?;
                    writeln!(f)?;
                }
                Ok(())
            })?;

            self.write_indent(f)?;
            write!(f, "}}")
        }

        fn print_expr(&mut self, node: &Expr, f: &mut Formatter<'_>) -> Result {
            write!(f, "{node};")
        }

        fn print_module(&mut self, node: &Module, f: &mut Formatter<'_>) -> Result {
            write!(f, "module {} ", node.ident)?;
            self.print_block(&node.body, f)
        }

        fn print_fndef(&mut self, node: &FnDef, f: &mut Formatter<'_>) -> Result {
            let params = node.params
                .iter()
                .map(Ident::as_str)
                .collect::<Vec<_>>()
                .join(", ");

            write!(f, "fn {}({params}) ", node.ident)?;
            self.print_block(&node.body, f)
        }

        fn print_assign(&mut self, node: &Assign, f: &mut Formatter<'_>) -> Result {
            write!(f, "{} = {};", node.target, node.value)
        }

        fn print_if(&mut self, node: &If, f: &mut Formatter<'_>) -> Result {
            write!(f, "if {} ", node.cond)?;
            self.print_block(&node.then_branch, f)?;

            if let Some(else_branch) = &node.else_branch {
                write!(f, " else ")?;
                self.print_block(else_branch, f)?;
            }

            Ok(())
        }

        fn print_while(&mut self, node: &While, f: &mut Formatter<'_>) -> Result {
            write!(f, "while {} ", node.cond)?;
            self.print_block(&node.body, f)
        }

        fn print_return(&mut self, node: &Return, f: &mut Formatter<'_>) -> Result {
            write!(f, "return {};", node.value)
        }
    }

    macro_rules! impl_display {
        ($target:ty => $method:ident) => {
            impl Display for $target {
                fn fmt(&self, f: &mut Formatter) -> Result {
                    Printer::new().$method(self, f)
                }
            }
        };
    }

    impl_display!(Stmt => print_stmt);
    impl_display!(Block => print_block);
    impl_display!(Module => print_module);
    impl_display!(FnDef => print_fndef);
    impl_display!(Assign => print_assign);
    impl_display!(If => print_if);
    impl_display!(While => print_while);
    impl_display!(Return => print_return);
}
