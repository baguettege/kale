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
    use crate::ast::{Assign, Block, FnDef, Ident, If, Module, Return, Stmt, While};

    struct Printer {
        indent: usize,
    }

    impl Printer {
        fn new() -> Self {
            Self { indent: 0 }
        }

        fn indent(&self, f: &mut Formatter) -> Result {
            write!(f, "{}", "    ".repeat(self.indent))
        }

        fn print_stmt(&mut self, stmt: &Stmt, f: &mut Formatter) -> Result {
            self.indent(f)?;
            write!(f, "{}", stmt)
        }

        fn print_block(&mut self, block: &Block, f: &mut Formatter<'_>) -> Result {
            writeln!(f, "{{")?;

            self.indent += 1;
            for stmt in &block.0 {
                self.print_stmt(stmt, f)?;
                writeln!(f)?;
            }
            self.indent -= 1;

            self.indent(f)?;
            writeln!(f, "}}")
        }
    }

    impl Display for Stmt {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                Stmt::Expr(node) => write!(f, "{node}"),
                Stmt::Module(node) => write!(f, "{node}"),
                Stmt::FnDef(node) => write!(f, "{node}"),
                Stmt::Assign(node) => write!(f, "{node}"),
                Stmt::If(node) => write!(f, "{node}"),
                Stmt::While(node) => write!(f, "{node}"),
                Stmt::Return(node) => write!(f, "{node}"),
            }
        }
    }

    impl Display for Block {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            Printer::new().print_block(self, f)
        }
    }

    impl Display for Module {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "module {} {}", self.ident, self.body)
        }
    }

    impl Display for FnDef {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            let params = self.params
                .iter()
                .map(Ident::as_str)
                .collect::<Vec<_>>()
                .join(", ");
            write!(f, "fn {}({params}) {}", self.ident, self.body)
        }
    }

    impl Display for Assign {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{} = {};", self.target, self.value)
        }
    }

    impl Display for If {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "if {} {}", self.cond, self.then_branch)?;
            if let Some(else_branch) = &self.else_branch {
                write!(f, " else {}", else_branch)
            } else {
                Ok(())
            }
        }
    }

    impl Display for While {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "while {} {}", self.cond, self.body)
        }
    }

    impl Display for Return {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "return {};", self.value)
        }
    }
}
