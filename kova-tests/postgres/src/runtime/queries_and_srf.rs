use crate::runtime::codec::AppCodec;
use crate::runtime::runtime_client;
use kova::dsl::func::{
    Count, DenseRank, FrameBound, GenerateSeries, JsonbEachText, Lag, Lead, Ntile, RowNumber, Sum,
    Unnest, Wildcard, Window,
};
use kova::query::Select;
use models::runtime::event_log::{AuditPayload, Decimal, EventLog, TagList};
use models::runtime::order::Order;
use models::runtime::top_spender::TopSpender;
use models::runtime::user::User;
use std::collections::HashMap;
use url::Url;
use uuid::Uuid;

struct GsAlias;
struct GsAlias2;

#[tokio::test]
async fn pg_cte_window_and_left_join_option_model_execution() {
    let client = runtime_client().await;

    client
        .insert(vec![
            User { id: 0, name: "Alice".into(), bio: None, balance: 100.0, is_active: true },
            User { id: 0, name: "Bob".into(), bio: None, balance: 200.0, is_active: true },
        ])
        .execute()
        .await
        .unwrap();

    client
        .insert(vec![
            Order { id: 0, user_id: 1, amount: 10.0, discount: None, is_active: true, raw_data: vec![1] },
            Order { id: 0, user_id: 1, amount: 30.0, discount: None, is_active: true, raw_data: vec![2] },
            Order { id: 0, user_id: 1, amount: 60.0, discount: None, is_active: true, raw_data: vec![3] },
        ])
        .execute()
        .await
        .unwrap();

    // 1. LEFT JOIN + `.load_as::<(User, Option<Order>)>()`
    // Bob (id = 2) has no orders; `Option<Order>` catches `UnexpectedNull` and yields `None`.
    let joined = client
        .select::<User>()
        .left_join::<Order>(Order::user_id.eq(User::id))
        .filter(User::id.eq(2))
        .load_as::<(User, Option<Order>)>()
        .await
        .unwrap();
    assert_eq!(joined.len(), 1);
    assert_eq!(joined[0].0.name, "Bob");
    assert_eq!(joined[0].1, None);

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
                .having(Sum(Order::amount).ge(100.0)),
        )
        .inner_join::<TopSpender>(User::id.eq(TopSpender::user_id))
        .columns((User::name, TopSpender::total_amount, TopSpender::order_count))
        .load_in::<(String, f64, i64)>()
        .await
        .unwrap();
    assert_eq!(top, vec![("Alice".to_string(), 100.0, 3)]);

    // 3. RowNumber, DenseRank, Ntile, Lag, Lead & Rolling Sum
    let win_rows = client
        .select::<Order>()
        .columns((
            Order::id,
            RowNumber().over(Window().partition_by(Order::user_id).order_by(Order::id.asc())),
            DenseRank().over(Window().order_by(Order::amount.desc())),
            Ntile(2).over(Window().order_by(Order::id.asc())),
            Lag(Order::amount)
                .offset(1)
                .default(0.0)
                .over(Window().order_by(Order::id.asc())),
            Lead(Order::amount)
                .offset(1)
                .default(999.0)
                .over(Window().order_by(Order::id.asc())),
            Sum(Order::amount).over(
                Window()
                    .partition_by(Order::user_id)
                    .order_by(Order::id.asc())
                    .rows(FrameBound::UnboundedPreceding, Some(FrameBound::CurrentRow)),
            ),
        ))
        .order_by(Order::id.asc())
        .load_in::<(i32, i64, i64, i32, f64, f64, f64)>()
        .await
        .unwrap();

    assert_eq!(
        win_rows,
        vec![
            (1, 1, 3, 1, 0.0, 30.0, 10.0),
            (2, 2, 2, 1, 10.0, 60.0, 40.0),
            (3, 3, 1, 2, 30.0, 999.0, 100.0),
        ]
    );
}

#[tokio::test]
async fn pg_srf_unnest_generate_series_and_jsonb_each_execution() {
    let client = runtime_client().await.codec::<AppCodec>();

    client
        .insert(vec![
            User { id: 0, name: "U1".into(), bio: None, balance: 10.0, is_active: true },
            User { id: 0, name: "U2".into(), bio: None, balance: 20.0, is_active: true },
            User { id: 0, name: "U3".into(), bio: None, balance: 30.0, is_active: true },
        ])
        .execute()
        .await
        .unwrap();

    // 1. Unnest(&[i32])
    let target_ids: &[i32] = &[1, 3];
    let unnested = client
        .select::<User>()
        .srf(Unnest(target_ids))
        .columns((User::id, User::name, Unnest::unnest))
        .filter(User::id.eq(Unnest::unnest))
        .order_by(User::id.asc())
        .load_in::<(i32, String, i32)>()
        .await
        .unwrap();
    assert_eq!(
        unnested,
        vec![(1, "U1".to_string(), 1), (3, "U3".to_string(), 3)]
    );

    // 2. GenerateSeries(start, stop).step(step) + Tag
    let series = client
        .select::<User>()
        .srf((
            GenerateSeries(1_i32, 3_i32).step(2_i32).tag::<GsAlias>(),
            GenerateSeries(10_i32, 20_i32).step(10_i32).tag::<GsAlias2>(),
        ))
        .columns((
            User::name,
            GenerateSeries::generate_series.tag::<GsAlias>(),
            GenerateSeries::generate_series.tag::<GsAlias2>(),
        ))
        .filter(User::id.eq(GenerateSeries::generate_series.tag::<GsAlias>()))
        .order_by((
            User::id.asc(),
            GenerateSeries::generate_series.tag::<GsAlias2>().asc(),
        ))
        .load_in::<(String, i32, i32)>()
        .await
        .unwrap();
    assert_eq!(
        series,
        vec![
            ("U1".to_string(), 1, 10),
            ("U1".to_string(), 1, 20),
            ("U3".to_string(), 3, 10),
            ("U3".to_string(), 3, 20),
        ]
    );

    // 3. JsonbEachText expanding JSONB key-value pairs
    client
        .insert(EventLog {
            id: Uuid::new_v4(),
            description: "srf_json".into(),
            created_at: chrono::Utc::now(),
            updated_at: jiff::Timestamp::now(),
            website: Url::parse("https://rust-lang.org").unwrap(),
            metadata: HashMap::from([
                ("k1".to_string(), "v1".to_string()),
                ("k2".to_string(), "v2".to_string()),
            ]),
            cost: Decimal(0),
            payload: AuditPayload { actor: "sys".into(), level: 1 },
            labels: TagList(Vec::new()),
        })
        .execute()
        .await
        .unwrap();

    let kv_pairs = client
        .select::<EventLog>()
        .srf(JsonbEachText(EventLog::metadata))
        .columns((JsonbEachText::key, JsonbEachText::value))
        .order_by(JsonbEachText::key.asc())
        .load_in::<(String, String)>()
        .await
        .unwrap();
    assert_eq!(
        kv_pairs,
        vec![
            ("k1".to_string(), "v1".to_string()),
            ("k2".to_string(), "v2".to_string()),
        ]
    );
}