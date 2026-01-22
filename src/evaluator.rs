/// Evaluator for FORM expressions
use crate::ast::*;
use std::collections::HashMap;

pub struct Evaluator {
    symbols: HashMap<String, Expr>,
    expressions: HashMap<String, Expr>,
    rules: Vec<(Expr, Expr)>,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            symbols: HashMap::new(),
            expressions: HashMap::new(),
            rules: Vec::new(),
        }
    }

    pub fn eval_statement(&mut self, stmt: Statement) -> Result<String, String> {
        match stmt {
            Statement::SymbolsDecl(syms) => {
                for sym in syms {
                    self.symbols.insert(sym.clone(), Expr::Symbol(sym));
                }
                Ok("Symbols declared".to_string())
            }
            Statement::ExpressionDecl { name, expr } => {
                let simplified = self.simplify(expr)?;
                self.expressions.insert(name.clone(), simplified.clone());
                Ok(format!("{} = {}", name, simplified))
            }
            Statement::LocalDecl { name, expr } => {
                let simplified = self.simplify(expr)?;
                self.expressions.insert(name.clone(), simplified.clone());
                Ok(format!("{} = {}", name, simplified))
            }
            Statement::IdRule { pattern, replacement } => {
                self.rules.push((pattern.clone(), replacement.clone()));
                Ok(format!("Rule added: {} -> {}", pattern, replacement))
            }
            Statement::Print(name) => {
                if let Some(expr) = self.expressions.get(&name) {
                    Ok(format!("{} = {}", name, expr))
                } else {
                    Err(format!("Expression '{}' not found", name))
                }
            }
            Statement::Sort => {
                // Apply all rules to all expressions
                let mut updated = HashMap::new();
                for (name, expr) in &self.expressions {
                    let simplified = self.apply_rules(expr.clone())?;
                    updated.insert(name.clone(), simplified);
                }
                self.expressions = updated;
                Ok("Sorted and rules applied".to_string())
            }
            Statement::EvalExpr(expr) => {
                let result = self.simplify(expr)?;
                Ok(format!("{}", result))
            }
        }
    }

    fn simplify(&self, expr: Expr) -> Result<Expr, String> {
        match expr {
            Expr::Number(_) => Ok(expr),
            Expr::Symbol(ref name) => {
                // Check if symbol has a value
                if let Some(val) = self.expressions.get(name) {
                    self.simplify(val.clone())
                } else {
                    Ok(expr)
                }
            }
            Expr::BinOp { op, left, right } => {
                let left = self.simplify(*left)?;
                let right = self.simplify(*right)?;

                // Try to evaluate if both are numbers
                if let (Expr::Number(l), Expr::Number(r)) = (&left, &right) {
                    let result = match op {
                        BinOpKind::Add => l + r,
                        BinOpKind::Sub => l - r,
                        BinOpKind::Mul => l * r,
                        BinOpKind::Div => {
                            if *r == 0.0 {
                                return Err("Division by zero".to_string());
                            }
                            l / r
                        }
                        BinOpKind::Pow => l.powf(*r),
                    };
                    return Ok(Expr::Number(result));
                }

                // Algebraic simplifications
                match op {
                    BinOpKind::Add => {
                        // x + 0 = x
                        if let Expr::Number(0.0) = right {
                            return Ok(left);
                        }
                        if let Expr::Number(0.0) = left {
                            return Ok(right);
                        }
                    }
                    BinOpKind::Mul => {
                        // x * 0 = 0
                        if let Expr::Number(0.0) = right {
                            return Ok(Expr::Number(0.0));
                        }
                        if let Expr::Number(0.0) = left {
                            return Ok(Expr::Number(0.0));
                        }
                        // x * 1 = x
                        if let Expr::Number(1.0) = right {
                            return Ok(left);
                        }
                        if let Expr::Number(1.0) = left {
                            return Ok(right);
                        }
                    }
                    BinOpKind::Pow => {
                        // x ^ 0 = 1
                        if let Expr::Number(0.0) = right {
                            return Ok(Expr::Number(1.0));
                        }
                        // x ^ 1 = x
                        if let Expr::Number(1.0) = right {
                            return Ok(left);
                        }
                    }
                    _ => {}
                }

                Ok(Expr::BinOp {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                })
            }
            Expr::UnOp { op, operand } => {
                let operand = self.simplify(*operand)?;

                if let Expr::Number(n) = operand {
                    let result = match op {
                        UnOpKind::Neg => -n,
                    };
                    Ok(Expr::Number(result))
                } else {
                    Ok(Expr::UnOp {
                        op,
                        operand: Box::new(operand),
                    })
                }
            }
            Expr::FunctionCall { name, args } => {
                let args: Result<Vec<_>, _> = args.into_iter().map(|a| self.simplify(a)).collect();
                let args = args?;

                // Built-in functions
                match name.as_str() {
                    "sin" | "cos" | "exp" | "log" => {
                        if args.len() == 1 {
                            if let Expr::Number(n) = args[0] {
                                let result = match name.as_str() {
                                    "sin" => n.sin(),
                                    "cos" => n.cos(),
                                    "exp" => n.exp(),
                                    "log" => n.ln(),
                                    _ => unreachable!(),
                                };
                                return Ok(Expr::Number(result));
                            }
                        }
                    }
                    _ => {}
                }

                Ok(Expr::FunctionCall { name, args })
            }
        }
    }

    fn apply_rules(&self, expr: Expr) -> Result<Expr, String> {
        let mut result = expr;
        let mut changed = true;
        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 100;

        // Keep applying rules until no more changes
        while changed && iterations < MAX_ITERATIONS {
            changed = false;
            iterations += 1;
            
            // Try to apply rules to the current expression
            for (pattern, replacement) in &self.rules {
                if let Some(bindings) = self.match_pattern(&result, pattern) {
                    result = self.substitute(replacement.clone(), &bindings);
                    changed = true;
                    break; // Start over with the new expression
                }
            }
            
            // Also apply rules recursively to subexpressions
            if !changed {
                result = match result {
                    Expr::BinOp { op, left, right } => {
                        let new_left = self.apply_rules_recursive(*left)?;
                        let new_right = self.apply_rules_recursive(*right)?;
                        Expr::BinOp {
                            op,
                            left: Box::new(new_left),
                            right: Box::new(new_right),
                        }
                    }
                    Expr::UnOp { op, operand } => {
                        let new_operand = self.apply_rules_recursive(*operand)?;
                        Expr::UnOp {
                            op,
                            operand: Box::new(new_operand),
                        }
                    }
                    Expr::FunctionCall { name, args } => {
                        let new_args: Result<Vec<_>, _> = args.into_iter()
                            .map(|a| self.apply_rules_recursive(a))
                            .collect();
                        Expr::FunctionCall {
                            name,
                            args: new_args?,
                        }
                    }
                    _ => result,
                };
                break; // Exit the while loop since we've recursively processed
            }
        }

        self.simplify(result)
    }

    fn apply_rules_recursive(&self, expr: Expr) -> Result<Expr, String> {
        // First apply rules to subexpressions
        let result = match expr {
            Expr::BinOp { op, left, right } => {
                let new_left = self.apply_rules_recursive(*left)?;
                let new_right = self.apply_rules_recursive(*right)?;
                Expr::BinOp {
                    op,
                    left: Box::new(new_left),
                    right: Box::new(new_right),
                }
            }
            Expr::UnOp { op, operand } => {
                let new_operand = self.apply_rules_recursive(*operand)?;
                Expr::UnOp {
                    op,
                    operand: Box::new(new_operand),
                }
            }
            Expr::FunctionCall { name, args } => {
                let new_args: Result<Vec<_>, _> = args.into_iter()
                    .map(|a| self.apply_rules_recursive(a))
                    .collect();
                Expr::FunctionCall {
                    name,
                    args: new_args?,
                }
            }
            _ => expr,
        };

        // Then try to match patterns at this level
        for (pattern, replacement) in &self.rules {
            if let Some(bindings) = self.match_pattern(&result, pattern) {
                return Ok(self.substitute(replacement.clone(), &bindings));
            }
        }

        Ok(result)
    }

    fn match_pattern(&self, expr: &Expr, pattern: &Expr) -> Option<HashMap<String, Expr>> {
        match (expr, pattern) {
            (Expr::Symbol(name1), Expr::Symbol(name2)) => {
                // Symbol in pattern matches symbol in expression if names match
                if name1 == name2 {
                    Some(HashMap::new())
                } else {
                    None
                }
            }
            (Expr::Number(n1), Expr::Number(n2)) => {
                if (n1 - n2).abs() < 1e-10 {
                    Some(HashMap::new())
                } else {
                    None
                }
            }
            (
                Expr::BinOp {
                    op: op1,
                    left: l1,
                    right: r1,
                },
                Expr::BinOp {
                    op: op2,
                    left: l2,
                    right: r2,
                },
            ) => {
                if op1 == op2 {
                    let left_match = self.match_pattern(l1, l2)?;
                    let right_match = self.match_pattern(r1, r2)?;

                    // Merge bindings
                    let mut bindings = left_match;
                    for (k, v) in right_match {
                        if let Some(existing) = bindings.get(&k) {
                            if existing != &v {
                                return None;
                            }
                        }
                        bindings.insert(k, v);
                    }
                    Some(bindings)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn substitute(&self, expr: Expr, bindings: &HashMap<String, Expr>) -> Expr {
        match expr {
            Expr::Symbol(ref name) => {
                if let Some(val) = bindings.get(name) {
                    val.clone()
                } else {
                    expr
                }
            }
            Expr::BinOp { op, left, right } => Expr::BinOp {
                op,
                left: Box::new(self.substitute(*left, bindings)),
                right: Box::new(self.substitute(*right, bindings)),
            },
            Expr::UnOp { op, operand } => Expr::UnOp {
                op,
                operand: Box::new(self.substitute(*operand, bindings)),
            },
            Expr::FunctionCall { name, args } => Expr::FunctionCall {
                name,
                args: args.into_iter().map(|a| self.substitute(a, bindings)).collect(),
            },
            _ => expr,
        }
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_number() {
        let mut eval = Evaluator::new();
        let stmt = Statement::EvalExpr(Expr::Number(42.0));
        let result = eval.eval_statement(stmt).unwrap();
        assert_eq!(result, "42");
    }

    #[test]
    fn test_eval_addition() {
        let mut eval = Evaluator::new();
        let expr = Expr::BinOp {
            op: BinOpKind::Add,
            left: Box::new(Expr::Number(1.0)),
            right: Box::new(Expr::Number(2.0)),
        };
        let stmt = Statement::EvalExpr(expr);
        let result = eval.eval_statement(stmt).unwrap();
        assert_eq!(result, "3");
    }

    #[test]
    fn test_eval_multiplication() {
        let mut eval = Evaluator::new();
        let expr = Expr::BinOp {
            op: BinOpKind::Mul,
            left: Box::new(Expr::Number(3.0)),
            right: Box::new(Expr::Number(4.0)),
        };
        let stmt = Statement::EvalExpr(expr);
        let result = eval.eval_statement(stmt).unwrap();
        assert_eq!(result, "12");
    }

    #[test]
    fn test_symbol_declaration() {
        let mut eval = Evaluator::new();
        let stmt = Statement::SymbolsDecl(vec!["x".to_string(), "y".to_string()]);
        let result = eval.eval_statement(stmt).unwrap();
        assert_eq!(result, "Symbols declared");
    }
}
