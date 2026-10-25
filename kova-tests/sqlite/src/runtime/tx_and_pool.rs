use crate::runtime::runtime_client;
use kova::{Error, Options};
use models::runtime::user::User;
use std::time::Duration;

#[test]
fn transaction_commit_and_rollback_atomicity() {
    let client = runtime_client();

    // 1. Successful commit persists all operations inside closure
    let uid = client
        .transaction(|tx| {
            let ids = tx
                .insert(User {
                    id: 0,
                    name: "tx_committed".into(),
                    bio: None,
                    balance: 300.0,
                    is_active: true,
                })
                .returning(User::id)
                .execute_in::<i32>()?;
            tx.update::<User>()
                .set(User::balance.increment(200.0))
                .filter(User::id.eq(ids[0]))
                .execute()?;
            Ok(ids[0])
        })
        .unwrap();

    let bal = client
        .select::<User>()
        .columns(User::balance)
        .filter(User::id.eq(uid))
        .load_in::<f64>()
        .unwrap();
    assert_eq!(bal, vec![500.0]);

    // 2. Returning `Err` inside closure triggers automatic rollback
    let tx_err: kova::Result<()> = client.transaction(|tx| {
        tx.update::<User>()
            .set(User::balance.set(0.0))
            .filter(User::id.eq(uid))
            .execute()?;
        tx.insert(User {
            id: 0,
            name: "tx_ghost".into(),
            bio: None,
            balance: 1.0,
            is_active: false,
        })
            .execute()?;
        Err(Error::ValidationError("abort transaction".into()))
    });
    assert!(matches!(tx_err, Err(Error::ValidationError(_))));

    let users = client.select::<User>().load().unwrap();
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].name, "tx_committed");
    assert_eq!(users[0].balance, 500.0);
}

#[test]
fn sqlite_file_options_and_pool_settings() {
    let tmp_path = std::env::temp_dir().join(format!("kova_test_{}.db", std::process::id()));
    let _ = std::fs::remove_file(&tmp_path);

    let client = Options::from(&tmp_path)
        .connection_timeout(Duration::from_secs(5))
        .init(|conn| {
            conn.execute_batch("PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL;")
        })
        .build()
        .unwrap();

    let mode = client
        .statement("PRAGMA journal_mode")
        .load_in::<String>()
        .unwrap();
    assert_eq!(mode[0].to_lowercase(), "wal");

    drop(client);
    let _ = std::fs::remove_file(&tmp_path);
}