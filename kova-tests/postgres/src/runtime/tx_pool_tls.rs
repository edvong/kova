#![allow(unreachable_code, dead_code, unused)]

use crate::render::pg_conn_str;
use crate::runtime::runtime_client;
use kova::options::RecyclingMethod;
use kova::{Error, Options};
use models::runtime::user::User;
use std::path::PathBuf;
use std::time::Duration;

fn ca_cert_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("certs/ca.crt")
}

#[tokio::test]
async fn pg_async_transaction_commit_and_rollback_atomicity() {
    let client = runtime_client().await;

    // 1. Transaction commit persists all changes
    let committed_id = client
        .transaction(async |tx| {
            let ids = tx
                .insert(User {
                    id: 0,
                    name: "tx_ok".into(),
                    bio: None,
                    balance: 100.0,
                    is_active: true,
                })
                .returning(User::id)
                .execute_in::<i32>()
                .await?;

            tx.update::<User>()
                .set(User::balance.increment(50.0))
                .filter(User::id.eq(ids[0]))
                .execute()
                .await?;

            Ok(ids[0])
        })
        .await
        .unwrap();

    let bal = client
        .select::<User>()
        .columns(User::balance)
        .filter(User::id.eq(committed_id))
        .load_in::<f64>()
        .await
        .unwrap();
    assert_eq!(bal, vec![150.0]);

    // 2. Returning `Err` inside the async closure rolls back all intermediate mutations
    let rollback_res: kova::Result<()> = client
        .transaction(async |tx| {
            tx.update::<User>()
                .set(User::balance.set(0.0))
                .filter(User::id.eq(committed_id))
                .execute()
                .await?;

            tx.insert(User {
                id: 0,
                name: "tx_rolled_back".into(),
                bio: None,
                balance: 999.0,
                is_active: true,
            })
                .execute()
                .await?;

            Err(Error::ValidationError("force rollback".into()))
        })
        .await;
    assert!(matches!(rollback_res, Err(Error::ValidationError(_))));

    let users = client.select::<User>().load().await.unwrap();
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].name, "tx_ok");
    assert_eq!(users[0].balance, 150.0);
}

#[tokio::test]
async fn pg_pool_options_and_recycling_configuration() {
    let client = Options::from(pg_conn_str())
        .max_size(4)
        .wait_timeout(Duration::from_secs(5))
        .create_timeout(Duration::from_secs(5))
        .recycle_timeout(Duration::from_secs(5))
        .recycling_method(RecyclingMethod::Verified)
        .build()
        .unwrap();

    let one = client.statement("SELECT 1").load_in::<i32>().await.unwrap();
    assert_eq!(one, vec![1]);
}

#[cfg(all(feature = "rustls", not(feature = "native-tls")))]
#[tokio::test]
async fn pg_tls_connection_with_rustls() {
    let ca_pem = std::fs::read(ca_cert_path())
        .expect("missing certs/ca.crt; start `docker compose up -d` in kova-tests/Postgres first");

    let mut root_store = rustls::RootCertStore::empty();
    let mut reader = std::io::BufReader::new(ca_pem.as_slice());
    for cert in rustls_pemfile::certs(&mut reader) {
        root_store.add(cert.unwrap()).unwrap();
    }

    let tls_config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let client = Options::from(format!("{} sslmode=require", pg_conn_str()))
        .tls(|| Ok(tls_config))
        .build()
        .unwrap();

    let ssl_active = client
        .statement("SELECT ssl FROM pg_stat_ssl WHERE pid = pg_backend_pid()")
        .load_in::<bool>()
        .await
        .unwrap();
    assert_eq!(ssl_active, vec![true], "connection must be encrypted via rustls");

    let err_client = Options::from(pg_conn_str())
        .tls(|| Err(Error::ConnectionError("mock rustls error".into())))
        .build();
    assert!(matches!(err_client, Err(Error::ConnectionError(_))));
}

#[cfg(all(feature = "native-tls", not(feature = "rustls")))]
#[tokio::test]
async fn pg_tls_connection_with_native_tls() {
    let ca_pem = std::fs::read(ca_cert_path())
        .expect("missing certs/ca.crt; start `docker compose up -d` in kova-tests/Postgres first");

    let cert = native_tls::Certificate::from_pem(&ca_pem).unwrap();
    let connector = native_tls::TlsConnector::builder()
        .add_root_certificate(cert)
        .build()
        .unwrap();

    let client = Options::from(format!("{} sslmode=require", pg_conn_str()))
        .tls(|| Ok(connector))
        .build()
        .unwrap();

    let ssl_active = client
        .statement("SELECT ssl FROM pg_stat_ssl WHERE pid = pg_backend_pid()")
        .load_in::<bool>()
        .await
        .unwrap();
    assert_eq!(ssl_active, vec![true], "connection must be encrypted via native-tls");

    let err_client = Options::from(pg_conn_str())
        .tls(|| Err(Error::ConnectionError("mock native-tls error".into())))
        .build();
    assert!(matches!(err_client, Err(Error::ConnectionError(_))));
}