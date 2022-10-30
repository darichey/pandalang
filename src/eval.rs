use recursion::{Collapse, expand_and_collapse};

use crate::{
    ast::{BinOpKind, Expr, ExprF},
    value::Value,
};

fn global_env_TEMP(name: &str) -> i64 {
    match name {
        "x" => 0,
        "y" => 1,
        "x'" => 2,
        "foo" => 3,
        "a" => 4,
        "b" => 5,
        "c" => 6,
        "d" => 7,
        "e" => 8,
        _ => panic!("unknown env var"),
    }
}

pub fn eval(expr: Expr) -> Value {
    // Collapse::collapse_layers(expr, |expr| match expr {
    //     ExprF::Int { n } => Value::Int { n },
    //     ExprF::Var { name } => Value::Int {
    //         n: global_env_TEMP(name.as_str()),
    //     },
    //     ExprF::BinOp { left, right, kind } => {
    //         let f = match kind {
    //             BinOpKind::Add => std::ops::Add::add,
    //             BinOpKind::Sub => std::ops::Sub::sub,
    //             BinOpKind::Mul => std::ops::Mul::mul,
    //             BinOpKind::Div => std::ops::Div::div,
    //         };

    //         let (x, y) = match (left, right) {
    //             (Value::Int { n: x }, Value::Int { n: y }) => (x, y),
    //             _ => panic!("oh god oh fuck"),
    //         };

    //         Value::Int { n: f(x, y) }
    //     }
    //     ExprF::Fun { arg, body } => Value::Fun {
    //         arg,
    //         body: Box::new(body),
    //     },
    //     ExprF::App { fun, arg } => {
    //         let (arg_name, body) = match fun {
    //             Value::Fun { arg, body } => (arg, *body),
    //             _ => panic!("oh god oh fuck"),
    //         };

    //         cas(body, arg, arg_name)
    //     }
    // })

    todo!()
}

fn cas(e1: Expr, e2: Expr, var: String) -> Expr {
    // return match *e1.0 {
    //     ExprF::Var { name } => {
    //         if var == name {
    //             e2
    //         } else {
    //             ExprBoxed(Box::new(ExprF::Var { name }))
    //         }
    //     }
    //     ExprF::Fun { arg, body } => {
    //         if var == arg {
    //             ExprBoxed(Box::new(ExprF::Fun { arg, body }))
    //         } else {
    //             ExprBoxed(Box::new(ExprF::Fun {
    //                 arg,
    //                 body: cas(body, e2, var),
    //             }))
    //         }
    //     }
    //     ExprF::App { fun, arg } => ExprBoxed(Box::new(ExprF::App {
    //         fun: cas(fun, e2.clone(), var.clone()),
    //         arg: cas(arg, e2, var),
    //     })),
    //     ExprF::Int { n } => ExprBoxed(Box::new(ExprF::Int { n })),
    //     ExprF::BinOp { left, right, kind } => ExprBoxed(Box::new(ExprF::BinOp {
    //         left: cas(left, e2.clone(), var.clone()),
    //         right: cas(right, e2, var),
    //         kind,
    //     })),
    // };
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{eval, parser, value::Value};

    fn eval_test(s: String) -> Value {
        eval::eval(parser::parse(s.as_str()).unwrap().into())
    }

    #[test]
    fn evals() {
        insta::glob!("snapshot_inputs/**/*.panda", |path| {
            let source = eval_test(std::fs::read_to_string(&path).unwrap());
            insta::assert_debug_snapshot!(source);
        });
    }
}
