/// Abstract Syntax Tree for FORM expressions
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    Symbol(String),
    BinOp {
        op: BinOpKind,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    UnOp {
        op: UnOpKind,
        operand: Box<Expr>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnOpKind {
    Neg,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Number(n) => {
                if n.fract() == 0.0 {
                    write!(f, "{}", *n as i64)
                } else {
                    write!(f, "{}", n)
                }
            }
            Expr::Symbol(s) => write!(f, "{}", s),
            Expr::BinOp { op, left, right } => {
                let op_str = match op {
                    BinOpKind::Add => "+",
                    BinOpKind::Sub => "-",
                    BinOpKind::Mul => "*",
                    BinOpKind::Div => "/",
                    BinOpKind::Pow => "^",
                };
                write!(f, "({} {} {})", left, op_str, right)
            }
            Expr::UnOp { op, operand } => {
                let op_str = match op {
                    UnOpKind::Neg => "-",
                };
                write!(f, "({}{})", op_str, operand)
            }
            Expr::FunctionCall { name, args } => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    SymbolsDecl(Vec<String>),
    ExpressionDecl { name: String, expr: Expr },
    LocalDecl { name: String, expr: Expr },
    IdRule { pattern: Expr, replacement: Expr },
    Print(String),
    Sort,
    EvalExpr(Expr),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::SymbolsDecl(syms) => write!(f, "Symbols {}", syms.join(", ")),
            Statement::ExpressionDecl { name, expr } => write!(f, "Expression {} = {}", name, expr),
            Statement::LocalDecl { name, expr } => write!(f, "Local {} = {}", name, expr),
            Statement::IdRule { pattern, replacement } => write!(f, "id {} = {}", pattern, replacement),
            Statement::Print(s) => write!(f, "Print {}", s),
            Statement::Sort => write!(f, ".sort"),
            Statement::EvalExpr(e) => write!(f, "{}", e),
        }
    }
}
