use super::ast::*;
use tcal_rs::totient;
use std::collections::HashMap;

pub struct Evaluator {
    vars: HashMap<String, f64>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    pub fn eval(&mut self, expr: &Expr) -> f64 {
        match expr {
            Expr::Number(n) => *n,

            Expr::Variable(name) => match name.as_str() {
                "pi" => std::f64::consts::PI,
                "e" => std::f64::consts::E,
                _ => *self.vars.get(name).unwrap_or(&0.0),
            },

            Expr::Assign { name, expr } => {
                let val = self.eval(expr);
                self.vars.insert(name.clone(), val);
                val
            }

            Expr::Unary { op, expr } => match op {
                UnaryOp::Neg => -self.eval(expr),
            },

            Expr::Binary { left, op, right } => {
                let l = self.eval(left);
                let r = self.eval(right);

                match op {
                    BinaryOp::Add => l + r,
                    BinaryOp::Sub => l - r,
                    BinaryOp::Mul => l * r,
                    BinaryOp::Div => l / r,
                    BinaryOp::Pow => l.powf(r),

                    BinaryOp::And => (l as i64 & r as i64) as f64,
                    BinaryOp::Or => (l as i64 | r as i64) as f64,
                    BinaryOp::Shl => ((l as i64) << r as i64) as f64,
                    BinaryOp::Shr => (l as i64 >> r as i64) as f64,
                }
            }

            Expr::Call { name, args } => {
                let vals: Vec<f64> = args.iter().map(|a| self.eval(a)).collect();

                match name.as_str() {
                    "sin" => vals[0].sin(),
                    "cos" => vals[0].cos(),
                    "tan" => vals[0].tan(),
                    "asin" => vals[0].asin(),
                    "acos" => vals[0].acos(),
                    "atan" => vals[0].atan(),
                    "sqrt" => vals[0].sqrt(),
                    "cbrt" => vals[0].cbrt(),
                    "abs" => vals[0].abs(),
                    "ln" | "log" => vals[0].ln(),
                    "log10" => vals[0].log10(),
                    "exp" => vals[0].exp(),
                    "totient" => totient(vals[0] as i64) as f64,
                    _ => panic!("Unknown function"),
                }
            }
        }
    }
}
