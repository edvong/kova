use crate::runtime::runtime_client;
use kova::Model;
use kova::dsl::func::{
    Count, FrameBound, GroupConcat, JsonEach, JsonTree, Sum, Wildcard, Window,
};
use kova::query::Select;
use models::runtime::order::Order;
use models::runtime::top_spender::TopSpender;
use models::runtime::user::User;

#[derive(Model)]
pub struct FtsDoc {
    pub title: String,
    pub body: String,
}

#[test]
fn cte_window_left_join_option_model_and_set_operations() {
    let client = runtime_client();

    client
        .insert(vec![
            User { id: 0, name: "Alice".into(), bio: None, balance: 100.0, is_active: true },
            User { id: 0, name: "Bob".into(), bio: None, balance: 200.0, is_active: true },
            User { id: 0, name: "NoOrderUser".into(), bio: None, balance: 50.0, is_active: true },
        ])
        .execute()
        .unwrap();

    client
        .insert(vec![
            Order { id: 0, user_id: 1, amount: 40.0, discount: None, is_active: true, raw_data: vec![1] },
            Order { id: 0, user_id: 1, amount: 80.0, discount: None, is_active: true, raw_data: vec![2] },
            Order { id: 0, user_id: 2, amount: 30.0, discount: None, is_active: true, raw_data: vec![3] },
        ])
        .execute()
        .unwrap();

    // 1. LEFT JOIN + `.load_as::<(User, Option<Order>)>()`
    // Unmatched right-hand rows produce `UnexpectedNull` on non-nullable `Order` columns,
    // which `FromRow for Option<Order>` converts into `Ok(None)`.
    let joined = client
        .select::<User>()
        .left_join::<Order>(Order::user_id.eq(User::id))
        .filter(User::id.in_list([2, 3]))
        .order_by(User::id.asc())
        .load_as::<(User, Option<Order>)>()
        .unwrap();
    assert_eq!(joined.len(), 2);
    assert_eq!(joined[0].0.name, "Bob");
    assert!(joined[0].1.is_some());
    assert_eq!(joined[0].1.as_ref().unwrap().amount, 30.0);
    assert_eq!(joined[1].0.name, "NoOrderUser");
    assert_eq!(joined[1].1, None);

    // 2. CTE + JOIN + HAVING
    let top = client
        .select::<User>()
        .with::<TopSpender>(
            Select::from::<Order>()
                .columns((
                    Order::user_id,
                    Sum(Order::amount).alias(TopSpender::total_amount),
                    Count(Wildcard).alias(TopSpender::order_count),
                ))
                .group_by(Order::user_id)
                .having(Sum(Order::amount).gt(50.0)),
        )
        .inner_join::<TopSpender>(User::id.eq(TopSpender::user_id))
        .columns((User::name, TopSpender::total_amount, TopSpender::order_count))
        .load_in::<(String, f64, i64)>()
        .unwrap();
    assert_eq!(top, vec![("Alice".to_string(), 120.0, 2)]);

    // 3. Window function rolling sum & GroupConcat
    let rolling = client
        .select::<Order>()
        .columns((
            Order::id,
            Sum(Order::amount).over(
                Window()
                    .partition_by(Order::user_id)
                    .order_by(Order::id.asc())
                    .rows(FrameBound::UnboundedPreceding, Some(FrameBound::CurrentRow)),
            ),
        ))
        .filter(Order::user_id.eq(1))
        .order_by(Order::id.asc())
        .load_in::<(i32, f64)>()
        .unwrap();
    assert_eq!(rolling, vec![(1, 40.0), (2, 120.0)]);

    let names = client
        .select::<User>()
        .columns(GroupConcat(User::name).separator(", ").order_by(User::name.desc()))
        .filter(User::id.le(2))
        .load_in::<String>()
        .unwrap();
    assert_eq!(names, vec!["Bob, Alice".to_string()]);
}

#[test]
fn sqlite_srf_json_each_json_tree_and_fts5_execution() {
    let client = runtime_client();

    client
        .insert(User {
            id: 0,
            name: r#"{"tags":["rust","sql"],"meta":{"ver":2}}"#.into(),
            bio: None,
            balance: 0.0,
            is_active: true,
        })
        .execute()
        .unwrap();

    // 1. JsonEach with path argument
    let tags = client
        .select::<User>()
        .srf(JsonEach((User::name, "$.tags")))
        .columns(JsonEach::value)
        .order_by(JsonEach::id.asc())
        .load_in::<String>()
        .unwrap();
    assert_eq!(tags, vec!["rust".to_string(), "sql".to_string()]);

    // 2. JsonTree recursive expansion
    let fullkeys = client
        .select::<User>()
        .srf(JsonTree((User::name, "$.meta")))
        .columns(JsonTree::fullkey)
        .order_by(JsonTree::id.asc())
        .load_in::<String>()
        .unwrap();
    assert_eq!(fullkeys, vec!["$.meta".to_string(), "$.meta.ver".to_string()]);

    // 3. FTS5 full-text search with BM25, Highlight, and Snippet
    client
        .insert(vec![
            FtsDoc {
                title: "Rust Database".into(),
                body: "Kova is a zero-cost type-safe SQL builder in Rust".into(),
            },
            FtsDoc {
                title: "Cooking Guide".into(),
                body: "How to bake sourdough bread".into(),
            },
        ])
        .execute()
        .unwrap();

    let hits = client
        .select::<FtsDoc>()
        .columns((
            FtsDoc::title,
            FtsDoc::body.highlight(1, "<b>", "</b>"),
            FtsDoc::body.snippet(1, "[", "]", "...", 10),
        ))
        .filter(FtsDoc::body.matches("Rust AND SQL"))
        .order_by(FtsDoc::body.bm25().weights([1.0, 5.0]).asc())
        .load_in::<(String, String, String)>()
        .unwrap();

    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].0, "Rust Database");
    assert!(hits[0].1.contains("<b>SQL</b>"));
    assert!(hits[0].2.contains("[Rust]"));
}