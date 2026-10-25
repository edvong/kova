use crate::render::{capture_one, render_client};
use kova::dsl::Condition;
use kova::types::Engine;
use models::runtime::user::User;
use proptest::prelude::*;

#[derive(Debug, Clone)]
enum PgBoolAst {
    EqId(i32),
    ILikeName(String),
    InListIds(Vec<i32>),
    IsNullBio,
    Not(Box<PgBoolAst>),
    And(Box<PgBoolAst>, Box<PgBoolAst>),
    Or(Box<PgBoolAst>, Box<PgBoolAst>),
}

impl PgBoolAst {
    fn to_condition(&self) -> (Condition<'static, Engine, ()>, usize) {
        match self {
            PgBoolAst::EqId(v) => (User::id.eq(*v), 1),
            PgBoolAst::ILikeName(s) => (User::name.ilike(s.clone()), 1),
            PgBoolAst::InListIds(list) => {
                let len = list.len();
                (User::id.in_list(list.clone()), len)
            }
            PgBoolAst::IsNullBio => (User::bio.is_null(), 0),
            PgBoolAst::Not(inner) => {
                let (c, n) = inner.to_condition();
                (!c, n)
            }
            PgBoolAst::And(l, r) => {
                let (cl, nl) = l.to_condition();
                let (cr, nr) = r.to_condition();
                (cl & cr, nl + nr)
            }
            PgBoolAst::Or(l, r) => {
                let (cl, nl) = l.to_condition();
                let (cr, nr) = r.to_condition();
                (cl | cr, nl + nr)
            }
        }
    }
}

fn arb_pg_bool_ast() -> impl Strategy<Value = PgBoolAst> {
    let leaf = prop_oneof![
        any::<i32>().prop_map(PgBoolAst::EqId),
        "[a-zA-Z0-9_%]{1,8}".prop_map(PgBoolAst::ILikeName),
        prop::collection::vec(any::<i32>(), 0..5).prop_map(PgBoolAst::InListIds),
        Just(PgBoolAst::IsNullBio),
    ];
    leaf.prop_recursive(4, 32, 2, |inner| {
        prop_oneof![
            inner.clone().prop_map(|x| PgBoolAst::Not(Box::new(x))),
            (inner.clone(), inner.clone())
                .prop_map(|(a, b)| PgBoolAst::And(Box::new(a), Box::new(b))),
            (inner.clone(), inner).prop_map(|(a, b)| PgBoolAst::Or(Box::new(a), Box::new(b))),
        ]
    })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(32))]

    #[test]
    fn prop_pg_condition_placeholders_monotonic_and_valid_syntax(ast in arb_pg_bool_ast()) {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        rt.block_on(async {
            let client = render_client().await;
            let (cond, expected_params) = ast.to_condition();

            let cap = capture_one(client.select::<User>().filter(cond).load()).await;

            // 1. Balanced parentheses
            let open_parens = cap.sql.chars().filter(|&c| c == '(').count();
            let close_parens = cap.sql.chars().filter(|&c| c == ')').count();
            assert_eq!(open_parens, close_parens);

            // 2. Sequential $1 .. $N placeholders
            for i in 1..=expected_params {
                let needle = format!("${i}");
                assert!(
                    cap.sql.contains(&needle),
                    "missing placeholder {needle} in SQL: {}",
                    cap.sql
                );
            }
            let next_invalid = format!("${}", expected_params + 1);
            assert!(
                !cap.sql.contains(&next_invalid),
                "unexpected extra placeholder {next_invalid} in SQL: {}",
                cap.sql
            );
        });
    }
}