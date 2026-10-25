use crate::types::{Inspector, PgPool, Postgres};
use deadpool_postgres::{
    Manager, ManagerConfig, Pool, PoolConfig, RecyclingMethod as NativeRecyclingMethod, Runtime,
};
use kova_core::database::Client;
use kova_core::types::Async;
use kova_core::{Error, Result};
use std::str::FromStr;
use std::time::Duration;
use tokio_postgres::Config;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RecyclingMethod {
    #[default]
    Fast,
    Verified,
    Clean,
}

pub struct Options {
    config: Result<Config>,
    pool_config: PoolConfig,
    manager_config: ManagerConfig,
    inspector: Option<Inspector>,

    #[cfg(feature = "rustls")]
    rustls_state: Option<Result<rustls::ClientConfig>>,

    #[cfg(feature = "native-tls")]
    native_tls_state: Option<Result<native_tls::TlsConnector>>,
}

impl Options {
    // Accepts standard Postgres connection strings,
    // like "host=localhost user=postgres dbname=kova"
    pub fn from<T: AsRef<str>>(conn: T) -> Self {
        let config = Config::from_str(conn.as_ref()).map_err(Error::database);

        Self {
            config,
            pool_config: PoolConfig::new(10),
            manager_config: ManagerConfig::default(),
            inspector: None,

            #[cfg(feature = "rustls")]
            rustls_state: None,

            #[cfg(feature = "native-tls")]
            native_tls_state: None,
        }
    }

    pub fn max_size(mut self, v: usize) -> Self {
        self.pool_config.max_size = v;
        self
    }

    pub fn wait_timeout(mut self, v: Duration) -> Self {
        self.pool_config.timeouts.wait = Some(v);
        self
    }

    pub fn create_timeout(mut self, v: Duration) -> Self {
        self.pool_config.timeouts.create = Some(v);
        self
    }

    pub fn recycle_timeout(mut self, v: Duration) -> Self {
        self.pool_config.timeouts.recycle = Some(v);
        self
    }

    pub fn recycling_method(mut self, v: RecyclingMethod) -> Self {
        self.manager_config.recycling_method = match v {
            RecyclingMethod::Fast => NativeRecyclingMethod::Fast,
            RecyclingMethod::Verified => NativeRecyclingMethod::Verified,
            RecyclingMethod::Clean => NativeRecyclingMethod::Clean,
        };
        self
    }

    #[cfg(feature = "rustls")]
    pub fn tls<F>(mut self, setup: F) -> Self
        where
            F: FnOnce() -> Result<rustls::ClientConfig> {
        self.rustls_state = Some(setup());
        self
    }

    #[cfg(feature = "native-tls")]
    pub fn tls<F>(mut self, setup: F) -> Self
        where
            F: FnOnce() -> Result<native_tls::TlsConnector> {
        self.native_tls_state = Some(setup());
        self
    }

    pub fn inspect(mut self, f: Inspector) -> Self {
        self.inspector = Some(f);
        self
    }

    pub fn build(self) -> Result<Client<Postgres, Async, ()>> {
        #[cfg(feature = "rustls")]
        if let Some(tls_result) = self.rustls_state {
            let config = tls_result?;
            let tls = tokio_postgres_rustls::MakeRustlsConnect::new(config);
            let manager = Manager::from_config(self.config?, tls, self.manager_config);
            let pool = Pool::builder(manager)
                .config(self.pool_config)
                .runtime(Runtime::Tokio1)
                .build()
                .map_err(Error::database)?;

            return Ok(Client::new(PgPool{ pool, inspector: self.inspector }));
        }

        #[cfg(feature = "native-tls")]
        if let Some(tls_result) = self.native_tls_state {
            let connector = tls_result?;
            let tls = postgres_native_tls::MakeTlsConnector::new(connector);
            let manager = Manager::from_config(self.config?, tls, self.manager_config);
            let pool = Pool::builder(manager)
                .config(self.pool_config)
                .runtime(Runtime::Tokio1)
                .build()
                .map_err(Error::database)?;

            return Ok(Client::new(PgPool{ pool, inspector: self.inspector }));
        }

        let manager =
            Manager::from_config(self.config?, tokio_postgres::NoTls, self.manager_config);
        let pool = Pool::builder(manager)
            .config(self.pool_config)
            .runtime(Runtime::Tokio1)
            .build()
            .map_err(Error::database)?;

        Ok(Client::new(PgPool{ pool, inspector: self.inspector }))
    }
}
