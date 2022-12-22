use super::input::{Environment, Expression, Label, Value};
use crate::day21::{Input, Output};

pub fn solve(input: &Input) -> Output {
    use Expression::*;
    use Value::*;

    // Eject the assignment to the "humn" variable from the environment.
    // We'll be calculating that.
    let mut env = input.clone();
    let humn_lbl = Label::try_from("humn").unwrap();
    env.remove(&humn_lbl);

    // Since the "root" monkey is supposed to check two numbers for equality,
    // let's get the two variables that the boss monkey will be checking and
    // determine what we get if we try to evaluate them.
    let root_lbl = Label::try_from("root").unwrap();
    let Add(left, right) = env.resolve(&root_lbl).unwrap() else { panic!("No root monkey!"); };

    // The PartialEval trait attempts to evaluate the calling expression and
    // updates each variable in the environment that can be evalutated along
    // the way. From this point forward, every variable in the environment
    // that doesn't depend on "humn" will be a raw value instead of an expression
    // containing references.
    let left_val = left.partial_eval(&mut env);
    let right_val = right.partial_eval(&mut env);

    // Turns out, from examining the input, there's only one path that depends
    // on the "humn" variable. This means that one of the two variables being
    // checked will be constant and the other will depend on the value of
    // "humn". So. let's check what we got from evaluating the left and right
    // variables and see which one returned an actual value. That will be the
    // value we start adjusting moving forward. The other variable value will
    // represent the formula we need to solve.
    let (mut carry, mut next_op) = match (left_val, right_val) {
        (lhs, Val(Raw(val))) => (val, Some(lhs)),
        (Val(Raw(val)), rhs) => (val, Some(rhs)),
        _ => unreachable!(),
    };

    // Since we know that both values checked by the "root" monkey must be equal,
    // then we know enough to start solving each expression from `next_op` all
    // the way to the value of "humn". For example, say we had the following
    // set of relationships:
    //
    // - root =  5 == abcd (5)
    // - abcd = 10 - efgh ->  5 = 10 - efgh -> efgh = 10 - 5 -> efgh =  5
    // - efgh = 60 / ijkl ->  5 = 60 / ijkl -> ijkl = 60 / 5 -> ijkl = 12
    // - ijkl =  3 * humn -> 12 =  3 * humn -> humn = 12 / 3 -> humn =  4
    //
    // Thus, we can calculate the value of "humn" by substituting values into
    // formulae and solving by rearrangement one formula at a time until we
    // find the value of "humn".
    while let Some(expr) = next_op {
        // Rearrange the expressions based on the value we're carrying forward and
        // the type of operation being performed. There's a lot of "unreachable"
        // points along the way that are possible because we know that every
        // variable that doesn't depend on "humn" has already been fully evaluated.
        (carry, next_op) = match expr {
            Add(lhs, rhs) => match (lhs, rhs) {
                (Ref(lhs), Raw(val)) => (carry - val, env.resolve(&lhs)),
                (Raw(val), Ref(rhs)) => (carry - val, env.resolve(&rhs)),
                (_, _) => unreachable!(),
            },
            Sub(lhs, rhs) => match (lhs, rhs) {
                (Ref(lhs), Raw(val)) => (val + carry, env.resolve(&lhs)),
                (Raw(val), Ref(rhs)) => (val - carry, env.resolve(&rhs)),
                (_, _) => unreachable!(),
            },
            Mul(lhs, rhs) => match (lhs, rhs) {
                (Ref(lhs), Raw(val)) => (carry / val, env.resolve(&lhs)),
                (Raw(val), Ref(rhs)) => (carry / val, env.resolve(&rhs)),
                (_, _) => unreachable!(),
            },
            Div(lhs, rhs) => match (lhs, rhs) {
                (Ref(lhs), Raw(val)) => (val * carry, env.resolve(&lhs)),
                (Raw(val), Ref(rhs)) => (val / carry, env.resolve(&rhs)),
                (_, _) => unreachable!(),
            },
            Val(_) => unreachable!(),
        };
    }

    carry.into()
}

/// Remove an expression from the Environment based on Label
impl Environment {
    fn remove(&mut self, label: &Label) -> Option<Expression> {
        self.0.remove(label)
    }
}

/// This trait is used to evaluate Expressions in an Environment, updating
/// the Environment whenever a variable is evaluated. Going forward, the
/// Environment will have variables as fully evaluated as possible upon
/// which the expression that calls PartialEval depends.
trait PartialEval {
    fn partial_eval(&self, env: &mut Environment) -> Expression;
}

impl PartialEval for Value {
    fn partial_eval(&self, env: &mut Environment) -> Expression {
        use Expression::*;
        use Value::*;

        match self {
            // This is where the magic happens. Every time a reference is
            // successfully evaluated, update the Environment with its new
            // value.
            Ref(var) => {
                let Some(expr) = env.resolve(var) else { return Val(*self); };
                let eval = expr.partial_eval(env);
                env.0.insert(*var, eval);
                eval
            }
            _ => Val(*self),
        }
    }
}

impl PartialEval for Expression {
    fn partial_eval(&self, env: &mut Environment) -> Expression {
        use Expression::*;
        use Value::*;

        // For each type of expression, if it can be fully evaluated, do so. If
        // only one side can be evaluated, evaluate that one and retain the other
        // as-is.
        match self {
            Add(v1, v2) => match (v1.partial_eval(env), v2.partial_eval(env)) {
                (Val(Raw(lhs)), Val(Raw(rhs))) => Val(Raw(lhs + rhs)),
                (_, Val(Raw(rhs))) => Add(*v1, Raw(rhs)),
                (Val(Raw(lhs)), _) => Add(Raw(lhs), *v2),
                (_, _) => *self,
            },
            Sub(v1, v2) => match (v1.partial_eval(env), v2.partial_eval(env)) {
                (Val(Raw(lhs)), Val(Raw(rhs))) => Val(Raw(lhs - rhs)),
                (_, Val(Raw(rhs))) => Sub(*v1, Raw(rhs)),
                (Val(Raw(lhs)), _) => Sub(Raw(lhs), *v2),
                (_, _) => *self,
            },
            Mul(v1, v2) => match (v1.partial_eval(env), v2.partial_eval(env)) {
                (Val(Raw(lhs)), Val(Raw(rhs))) => Val(Raw(lhs * rhs)),
                (_, Val(Raw(rhs))) => Mul(*v1, Raw(rhs)),
                (Val(Raw(lhs)), _) => Mul(Raw(lhs), *v2),
                (_, _) => *self,
            },
            Div(v1, v2) => match (v1.partial_eval(env), v2.partial_eval(env)) {
                (Val(Raw(lhs)), Val(Raw(rhs))) => Val(Raw(lhs / rhs)),
                (_, Val(Raw(rhs))) => Div(*v1, Raw(rhs)),
                (Val(Raw(lhs)), _) => Div(Raw(lhs), *v2),
                (_, _) => *self,
            },
            // Obviously, a value has fewer options
            Expression::Val(value) => value.partial_eval(env),
        }
    }
}
