use crate::render::{capture_one, render_client};
use kova::dsl::Condition;
use kova::dsl::func::Math;
use kova::types::Engine;
use models::runtime::user::User;
use proptest::prelude::*;

#[derive(Debug, Clone)]
enum BoolAst {
    EqId(i32),
    LikeName(String),
    IsNullBio,
    Not(Box<BoolAst>),
    And(Box<BoolAst>, Box<BoolAst>),
    Or(Box<BoolAst>, Box<BoolAst>),
}

impl BoolAst {
    fn to_condition(&self) -> (Condition<'static, Engine, ()>, usize) {
        match self {
            BoolAst::EqId(v) => (User::id.eq(*v), 1),
            BoolAst::LikeName(s) => (User::name.like(s.clone()), 1),
            BoolAst::IsNullBio => (User::bio.is_null(), 0),
            BoolAst::Not(inner) => {
                let (c, n) = inner.to_condition();
                (!c, n)
            }
            BoolAst::And(l, r) => {
                let (cl, nl) = l.to_condition();
                let (cr, nr) = r.to_condition();
                (cl & cr, nl + nr)
            }
            BoolAst::Or(l, r) => {
                let (cl, nl) = l.to_condition();
                let (cr, nr) = r.to_condition();
                (cl | cr, nl + nr)
            }
        }
    }
}

fn arb_bool_ast() -> impl Strategy<Value = BoolAst> {
    let leaf = prop_oneof![
        any::<i32>().prop_map(BoolAst::EqId),
        "[a-zA-Z0-9_%]{1,8}".prop_map(BoolAst::LikeName),
        Just(BoolAst::IsNullBio),
    ];
    leaf.prop_recursive(4, 32, 2, |inner| {
        prop_oneof![
            inner.clone().prop_map(|x| BoolAst::Not(Box::new(x))),
            (inner.clone(), inner.clone())
                .prop_map(|(a, b)| BoolAst::And(Box::new(a), Box::new(b))),
            (inner.clone(), inner).prop_map(|(a, b)| BoolAst::Or(Box::new(a), Box::new(b))),
        ]
    })
}

#[derive(Debug, Clone)]
enum MathOpKind {
    Add(f64),
    Sub(f64),
    Mul(f64),
    Div(f64),
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    #[test]
    fn prop_condition_tree_balanced_parens_and_valid_sqlite_syntax(ast in arb_bool_ast()) {
        let client = render_client();
        let (cond, expected_params) = ast.to_condition();

        let cap = capture_one(|| client.select::<User>().filter(cond).load());

        let open_parens = cap.sql.chars().filter(|&c| c == '(').count();
        let close_parens = cap.sql.chars().filter(|&c| c == ')').count();
        prop_assert_eq!(open_parens, close_parens);

        let placeholders = cap.sql.chars().filter(|&c| c == '?').count();
        prop_assert_eq!(placeholders, expected_params);
    }

    #[test]
    fn prop_math_chain_invariants(ops in prop::collection::vec(prop_oneof![
        (-1000.0..1000.0_f64).prop_map(MathOpKind::Add),
        (-1000.0..1000.0_f64).prop_map(MathOpKind::Sub),
        (-100.0..100.0_f64).prop_map(MathOpKind::Mul),
        (1.0..100.0_f64).prop_map(MathOpKind::Div),
    ], 1..12)) {
        let client = render_client();
        let expected_len = ops.len();

        let mut iter = ops.into_iter();
        let first = iter.next().unwrap();
        let mut expr: Math<_, ()> = match first {
            MathOpKind::Add(v) => User::balance.add(v),
            MathOpKind::Sub(v) => User::balance.sub(v),
            MathOpKind::Mul(v) => User::balance.mul(v),
            MathOpKind::Div(v) => User::balance.div(v),
        };
        for op in iter {
            expr = match op {
                MathOpKind::Add(v) => expr.add(v),
                MathOpKind::Sub(v) => expr.sub(v),
                MathOpKind::Mul(v) => expr.mul(v),
                MathOpKind::Div(v) => expr.div(v),
            };
        }

        let cap = capture_one(|| client.select::<User>().columns(expr).load_in::<f64>());

        let placeholders = cap.sql.chars().filter(|&c| c == '?').count();
        prop_assert_eq!(placeholders, expected_len);
    }
}