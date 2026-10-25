use crate::options::InitConnection;
use crate::types::Value;
use kova_core::types::{Connection, ConnectionSync, Executable, ExecutableSync, Sync};
use kova_core::{Error, Result};
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

pub type Inspector = fn(&str, &[Value<'_>]);
pub(crate) type InitFn = Box<dyn Fn(&mut InitConnection<'_>) -> Result<()> + Send + std::marker::Sync>;

pub(crate) enum Target {
    File(PathBuf),
    Memory,
}

struct PoolState {
    idle: Vec<rusqlite::Connection>,
    total: u32,
}

pub(crate) struct PoolInner {
    target: Target,
    init: Option<InitFn>,
    max_size: u32,
    timeout: Option<Duration>,
    state: Mutex<PoolState>,
    condvar: Condvar,
}

impl PoolInner {
    fn open_connection(&self) -> Result<rusqlite::Connection> {
        let mut conn = match &self.target {
            Target::File(path) => rusqlite::Connection::open(path).map_err(Error::database)?,
            Target::Memory => rusqlite::Connection::open_in_memory().map_err(Error::database)?,
        };
        if let Some(dur) = self.timeout {
            conn.busy_timeout(dur).map_err(Error::database)?;
        }
        if let Some(init) = &self.init {
            init(&mut InitConnection(&mut conn))?;
        }
        Ok(conn)
    }

    pub(crate) fn get(&self) -> Result<PooledConn<'_>> {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        let mut deadline: Option<Instant> = None;

        loop {
            // 1. Fast path: pop a warm connection from the idle stack
            if let Some(conn) = state.idle.pop() {
                return Ok(PooledConn { pool: self, conn: Some(conn) });
            }

            // 2. Grow path: allocate a slot and open the connection outside the Mutex lock
            if state.total < self.max_size {
                state.total += 1;
                drop(state);

                match self.open_connection() {
                    Ok(conn) => return Ok(PooledConn { pool: self, conn: Some(conn) }),
                    Err(err) => {
                        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
                        state.total -= 1;
                        self.condvar.notify_one();
                        return Err(err);
                    }
                }
            }

            // 3. Wait path: pool is at capacity, wait until a connection is returned or deadline expires
            match self.timeout {
                None => {
                    state = self.condvar.wait(state).unwrap_or_else(|e| e.into_inner());
                }
                Some(dur) => {
                    let end = *deadline.get_or_insert_with(|| Instant::now() + dur);
                    let remaining = end.saturating_duration_since(Instant::now());
                    if remaining.is_zero() {
                        return Err(Error::ConnectionError(
                            "timed out waiting for an available SQLite connection".into(),
                        ));
                    }
                    let (new_state, timeout_res) = self
                        .condvar
                        .wait_timeout(state, remaining)
                        .unwrap_or_else(|e| e.into_inner());
                    state = new_state;
                    if timeout_res.timed_out() && state.idle.is_empty() {
                        return Err(Error::ConnectionError(
                            "timed out waiting for an available SQLite connection".into(),
                        ));
                    }
                }
            }
        }
    }
}

pub(crate) struct PooledConn<'a> {
    pool: &'a PoolInner,
    conn: Option<rusqlite::Connection>,
}

impl Deref for PooledConn<'_> {
    type Target = rusqlite::Connection;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.conn.as_ref().unwrap()
    }
}

impl DerefMut for PooledConn<'_> {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.conn.as_mut().unwrap()
    }
}

impl Drop for PooledConn<'_> {
    #[inline]
    fn drop(&mut self) {
        if let Some(conn) = self.conn.take() {
            let mut state = self.pool.state.lock().unwrap_or_else(|e| e.into_inner());
            state.idle.push(conn);
            self.pool.condvar.notify_one();
        }
    }
}

#[derive(Clone)]
pub struct SqlitePool {
    pub(crate) inner: Arc<PoolInner>,
    pub(crate) inspector: Option<Inspector>,
}

impl SqlitePool {
    pub(crate) fn new(
        target: Target, init: Option<InitFn>, max_size: u32, timeout: Option<Duration>,
        inspector: Option<Inspector>,
    ) -> Result<Self> {
        // In-memory SQLite databases are strictly single-connection (`:memory:`).
        let max_size = match target {
            Target::Memory => 1,
            Target::File(_) => max_size.max(1),
        };

        let inner = Arc::new(PoolInner {
            target,
            init,
            max_size,
            timeout,
            state: Mutex::new(PoolState { idle: Vec::with_capacity(max_size as usize), total: 1 }),
            condvar: Condvar::new(),
        });

        // Eagerly open the first connection to validate path/init SQL immediately.
        let first_conn = inner.open_connection()?;
        inner.state.lock().unwrap_or_else(|e| e.into_inner()).idle.push(first_conn);

        Ok(Self { inner, inspector })
    }
}

pub struct SqliteTx<'t> {
    pub(crate) tx: rusqlite::Transaction<'t>,
    pub(crate) inspector: Option<Inspector>,
}

pub trait SqliteExecutor: ExecutableSync<Native = rusqlite::Connection> {
    fn inspect(&self, sql: &str, params: &[Value<'_>]);
}

impl SqliteExecutor for SqlitePool {
    #[inline(always)]
    fn inspect(&self, sql: &str, params: &[Value<'_>]) {
        if let Some(f) = self.inspector {
            f(sql, params);
        }
    }
}

impl SqliteExecutor for &mut SqliteTx<'_> {
    #[inline(always)]
    fn inspect(&self, sql: &str, params: &[Value<'_>]) {
        if let Some(f) = self.inspector {
            f(sql, params);
        }
    }
}

impl Executable for SqlitePool {
    type Mode = Sync;
    type Native = rusqlite::Connection;
}

impl ExecutableSync for SqlitePool {
    fn with_native_sync<F, R>(&self, f: F) -> Result<R>
        where
            F: FnOnce(&Self::Native) -> Result<R> {
        let guard = self.inner.get()?;
        f(&guard)
    }
}

impl<'a> Executable for SqliteTx<'a> {
    type Mode = Sync;
    type Native = rusqlite::Connection;
}

impl<'a> ExecutableSync for SqliteTx<'a> {
    fn with_native_sync<F, R>(&self, f: F) -> Result<R>
        where
            F: FnOnce(&Self::Native) -> Result<R> {
        f(&self.tx)
    }
}

impl Connection for SqlitePool {
    type Transaction<'t> = SqliteTx<'t>;
}

impl<'a> ConnectionSync for SqlitePool {
    fn transaction_sync<F, R>(&self, f: F) -> Result<R>
        where
            F: FnOnce(&mut Self::Transaction<'_>) -> Result<R> {
        let mut guard = self.inner.get()?;
        let native_tx = guard.transaction().map_err(Error::database)?;
        let mut tx = SqliteTx { tx: native_tx, inspector: self.inspector };
        let result = f(&mut tx);
        match result {
            Ok(data) => {
                tx.tx.commit().map_err(Error::database)?;
                Ok(data)
            }
            Err(e) => {
                tx.tx.rollback().map_err(Error::database)?;
                Err(e)
            }
        }
    }
}
