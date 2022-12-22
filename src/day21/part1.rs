use super::input::{Environment, Expression, Label, Value};
use crate::day21::{Input, Output};

/// Solve Day 21, Part 1
pub fn solve(input: &Input) -> Output {
    let root = Value::try_from("root").unwrap();
    root.eval(input).unwrap().into()
}

/// Fetch an Expression from the Environment by Label
impl Environment {
    pub fn resolve(&self, var: &Label) -> Option<Expression> {
        self.0.get(var).copied()
    }
}

/// Trait for evaluating an Expression in an Environment
trait Eval {
    fn eval(&self, env: &Environment) -> Option<i128>;
}

impl Eval for Value {
    /// Recursively evaluate the Value until a Value::Raw can be returned
    fn eval(&self, env: &Environment) -> Option<i128> {
        match self {
            Value::Ref(var) => env.resolve(var).and_then(|v| v.eval(env)),
            Value::Raw(val) => Some(*val),
        }
    }
}

impl Eval for Expression {
    /// Recursively evaluate the Value until a Value::Raw can be returned
    fn eval(&self, env: &Environment) -> Option<i128> {
        // Perform operations based on the type of Expression
        match self {
            Expression::Add(v1, v2) => Some(v1.eval(env)? + v2.eval(env)?),
            Expression::Sub(v1, v2) => Some(v1.eval(env)? - v2.eval(env)?),
            Expression::Mul(v1, v2) => Some(v1.eval(env)? * v2.eval(env)?),
            Expression::Div(v1, v2) => Some(v1.eval(env)? / v2.eval(env)?),
            Expression::Val(value) => value.eval(env),
        }
    }
}
