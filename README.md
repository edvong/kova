# Kova

In production, the traditional ORM is an architectural lie. It turns domain logic into an unreadable mess far worse than raw SQL, while choking the database with implicit queries and lock contention.

Industry veterans blame the object-relational impedance mismatch; modern developers claim Rust’s strictness makes ORMs impossible. Both excuses are an intellectual cop-out. Mapping flat rows to native structs has zero inherent friction. Rust’s compiler didn't create these problems—it simply exposed the rot. The true culprit is a data layer built on three bankrupt design dogmas:

**1. The "Object Graph" Delusion**
Mainstream ORMs peddle a fantasy: that you can pretend the relational database doesn't exist, treat physical tables as a network of in-memory pointers, and hot-swap database engines at runtime behind a single watered-down dialect. Sustaining that illusion requires bloating the framework with lazy loading, session caches, units of work, and dirty checking. Half a century of battle-tested relational algebra gets thrown out the window so application code can clumsily chase pointers in memory, while "database agnosticism" drags your system down to the lowest common denominator of every engine on earth.  
Even in Rust—a language forged on explicit memory layouts and zero-cost abstractions—frameworks still wave the banner of ergonomics, promising a "cozy and easy" developer experience while smuggling this thoroughly discredited object-graph model from enterprise OOP. Some wrap plain structs in bloated `ActiveModel` state trackers; others attempt to build a universal query engine in application space, papering over the physical chasm between SQL and NoSQL. Beyond breeding runaway `N+1` query storms, connection pool starvation, deadlocks, and borrow-checker nightmares in production, this design solves nothing: it merely relocates database complexity into your application runtime and turns your data layer into an unmaintainable liability.

**2. Unhinged Type Gymnastics**
In the name of compile-time SQL verification, another camp swings to the opposite extreme: encoding every selected column, every `JOIN`, and every `WHERE` clause directly into the generic return type. A routine query with five joins and a handful of filters balloons into dozens of layers of recursive tuple trees and sprawling trait bounds. Compile times explode. A single mismatched field spews hundreds of lines of impenetrable compiler vomit, and you cannot split a query into helper functions or conditionally append a filter without boxing half the universe. That isn't safety. It is a failure to reduce an open grammar into a closed algebra—dumping the compiler's raw solver state straight into your lap.

**3. Macro Abuse and the Retreat to String Stitching**
Forcing users to wrap business logic in procedural macros is a black-box cover-up for an inadequate type system. The dominant faction retreats to raw SQL strings under the banner of "keeping it simple"—going so far as demanding a live database instance or brittle offline schema dumps just to run `cargo check`. This is not pragmatism; it is architectural defeatism: failing to build a composable abstraction, and then declaring abstractions useless. The moment you need to compose filters dynamically, disambiguate multiple self-joins, or reuse query fragments across modules, the illusion of simplicity vanishes. You are thrown straight back into glorified string concatenation—counting `$1, $2` positional arguments by hand, balancing parentheses in your head, and turning every database refactor into a runtime minefield.

We do not need another leaky illusion. We need an intuitive, mathematically rigorous way to express SQL directly in Rust.

Kova exists to do one thing: strip away the synthetic bloat and return the data access layer to its physical reality and algebraic boundaries. No magical runtimes. No compiler abuse. No string hacks.

**If you know SQL, you already know Kova.**

1. **Map relational algebra directly; never hide it.** SQL is already the most precise language for manipulating relational data. Kova maps SQL ASTs directly onto idiomatic Rust expressions. When you need deeply nested hierarchical views, Kova invokes native database JSON/JSONB aggregation—assembling the entire tree inside the database engine and deserializing it into structs in a single SQL query and a single network round-trip. In-memory object-graph traversal is dead.
2. **Respect physical boundaries; reject the myth of runtime unification.** PostgreSQL is a networked client/server protocol, so Kova's PG client is natively `async`. SQLite is an in-process C engine linked directly into your address space; wrapping it in `async` only adds pointless cross-thread scheduling overhead, so Kova's SQLite client is natively synchronous. Core relational algebra shares an identical syntax, while native engine capabilities—PG's JSONB operator chains, 1D arrays, row-level locks, and set-returning functions; SQLite's binary JSONB, FTS5 full-text search, and BM25 scoring—are exposed directly per engine, backed by prepared statement caches and zero-copy slice borrowing.
3. **Zero query macros, zero trait imports, flat constant-width types.** Beyond a single `#[derive(Model)]` (or `FromRow`, `Cte`) on your structs, writing queries, mutations, and transactions requires **zero macros and zero trait imports (no `use xxx::prelude::*`)**. Every query capability lives on inherent methods of structs and column nodes, powered by an internal `Open`/`Close` dual-track operand dispatch that lets columns and literals mix freely. Semantic validation happens at the method call boundary, and phantom markers are immediately erased via `EraseX`. Builder type signatures never grow, monomorphization bloat is eliminated, and every type boundary is guarded by custom `#[diagnostic::on_unimplemented]` compiler diagnostics.
4. **Parameterized codec contexts (`Codec <X>`) for large Cargo workspaces.** By parameterizing both encoding/decoding and domain capability traits over a context type `X`, Kova sidesteps Rust's Orphan Rules algebraically. Extract pure domain models into a standalone `models` crate with zero database driver dependencies and zero Newtype wrappers, then wire up zero-copy codecs, cross-type compatibility rules, and native SQL operators for any third-party type in your downstream service crates.

---

## Table of Contents

1. Get Started
2. Client & Connection Pool
3. Data Modeling & the Three Relational Roles
4. CRUD & the Typestate Machine
5. Expressions, Boolean Algebra & Type Compatibility
6. Joins, Lexical Tag Scopes & Subqueries
7. Standard Functions, Aggregate Modifiers & Window Analytics
8. PostgreSQL Native Features & Higher-Order Functions
9. SQLite Native Features & Full-Text Search
10. Transactions & the Raw SQL Escape Hatch
11. Workspace Architecture & Design Trade-offs

---

## 1. Get Started

Kova natively supports PostgreSQL and SQLite. Here is what that looks like in practice—and notice what is missing from start to finish: no trait imports, and not a single query macro.

### 1.1 Dependencies

Kova enables the asynchronous `postgres` engine by default. To use the synchronous `sqlite` engine, disable default features and enable `sqlite`:

```toml
# PostgreSQL engine (enable rustls or native-tls as needed)
[dependencies]
kova = { version = "0.1", features = ["rustls"] }

# SQLite engine
[dependencies]
kova = { version = "0.1", default-features = false, features = ["sqlite"] }
```

### 1.2 Models & Basic Queries

Take two tables: `User` and `Order`. Annotate the structs with `#[derive(Model)]`, and mark database-generated columns like auto-incrementing primary keys with `#[kova(auto)]`:

```rust
use kova::{Client, Model, Options, Result};
use kova::dsl::func::{Count, Sum, Wildcard};

#[derive(Model)]
pub struct User {
    #[kova(auto)]
    pub id: i32,
    pub name: String,
    pub bio: Option<String>,
    pub balance: f64,
    pub is_active: bool,
}

#[derive(Model)]
pub struct Order {
    #[kova(auto)]
    pub id: i32,
    pub user_id: i32,
    pub amount: f64,
    pub is_active: bool,
}

async fn basic_operations() -> Result<()> {
    let client = Options::from("host=127.0.0.1 user=postgres dbname=app")
        .max_size(16)
        .build()?;

    // 1. Batch insert and retrieve generated IDs.
    // Empty slices short-circuit immediately; batches exceeding the wire protocol parameter limit
    // (PG: 65,535 / SQLite: 32,766) are automatically split and executed in chunks.
    let new_users = [
        User { id: 0, name: "Alice".into(), bio: Some("Architect".into()), balance: 1000.0, is_active: true },
        User { id: 0, name: "Bob".into(), bio: None, balance: 200.0, is_active: true },
    ];
    let ids = client
        .insert(&new_users[..])
        .returning(User::id)
        .execute_in::<i32>()
        .await?;

    // 2. Filtered query.
    // `User::bio` is `Option<String>`, yet accepts `&str` directly without `Some(String::from(...))`.
    // `User::balance` is `f64`, yet accepts an `i16` literal via lossless widening.
    let active_users = client
        .select::<User>()
        .filter(User::is_active.eq(true) & User::bio.eq("Architect") & User::balance.ge(100_i16))
        .load() // Infers Vec<User> automatically
        .await?;

    // 3. Joins, aggregate filtering, and tuple extraction.
    // Rust's bidirectional type inference lets you either use turbofish `.load_in::<(i32, String, f64, i64)>()`
    // or annotate `let stats: Vec<(i32, String, f64, i64)> = ...` and call `.load_in()`. Both are identical.
    let stats = client
        .select::<User>()
        .inner_join::<Order>(Order::user_id.eq(User::id))
        .columns((User::id, User::name, Sum(Order::amount), Count(Wildcard)))
        .filter(User::is_active.eq(true) & !User::name.like("%bot%"))
        .group_by((User::id, User::name))
        .having(Sum(Order::amount).gt(50.0))
        .order_by(Sum(Order::amount).desc())
        .limit(10)
        .load_in::<(i32, String, f64, i64)>()
        .await?;

    Ok(())
}
```

As you write queries, Kova's typestate machine and capability guards reject invalid SQL at compile time:

```rust
// ❌ Compile error: UPDATE is in the `Unset` state without `.set()`; `.execute()` does not exist.
client.update::<User>().filter(User::id.eq(1)).execute().await;

// ❌ Compile error: Conflict target specified without declaring `.do_nothing()` or `.update()`.
client.insert(&new_users[0]).on_conflict(User::id).execute().await;

// ❌ Compile error: Aggregate functions lack `CanGroup` and cannot appear in `GROUP BY`.
client.select::<User>().group_by(Sum(Order::amount));

// ❌ Compile error: Non-nullable `User::name` rejects `Option`, preventing reverse null pollution.
client.select::<User>().filter(User::name.eq(Some("Alice")));

// ❌ Compile error: Implicit narrowing from `i64` to `i32` is forbidden.
client.select::<User>().filter(User::id.eq(100_i64));
```

### 1.3 Assembling Nested Views in a Single Query

Suppose you need to fetch blog posts (`Post`), each embedding its `Author` and a full list of comments (`Vec<CommentDto>`), where each comment also embeds its commenter (`Author`).

Forget firing multiple round-trip queries or folding cartesian products in application memory. Define your physical tables alongside read-only view structs—deriving `FromRow` generates read-only column identifiers like `PostView::author` for `.alias()` binding:

```rust
use kova::{FromRow, Model};
use serde::Deserialize;

#[derive(Model)]
pub struct Post {
    #[kova(auto)]
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author_id: i32,
}

#[derive(Model)]
pub struct Comment {
    #[kova(auto)]
    pub id: i32,
    pub post_id: i32,
    pub user_id: i32,
    pub content: String,
}

#[derive(FromRow)]
pub struct PostView {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub author: Author,
    pub comments: Vec<CommentDto>,
}

#[derive(Deserialize, FromRow)]
pub struct CommentDto {
    pub id: i32,
    pub content: String,
    pub commenter: Author,
}

#[derive(Deserialize)]
pub struct Author {
    pub id: i32,
    pub name: String,
}
```

Once JSON decoding is wired up (see Chapter 11), invoke the database's native JSON/JSONB aggregation functions to pull the entire tree in **one SQL query and one network round-trip**:

```rust
// --- PostgreSQL: JsonbBuildObject & JsonbAgg ---
// Kova automatically wraps `JsonbAgg` in `coalesce(..., '[]'::jsonb)` during rendering,
// so posts with zero comments decode cleanly into `vec![]` instead of failing on SQL NULL.
use kova::dsl::func::{JsonbAgg, JsonbBuildObject};
use kova::query::Select;

let views = pg_client
    .select::<Post>()
    .columns((
        Post::id,
        Post::title,
        Post::content,
        JsonbBuildObject((User::id, User::name)).alias(PostView::author),
        Select::from::<Comment>()
            .columns(JsonbAgg(JsonbBuildObject((
                Comment::id,
                Comment::content,
                JsonbBuildObject((User::id, User::name)).alias(CommentDto::commenter),
            ))))
            .inner_join::<User>(Comment::user_id.eq(User::id))
            .filter(Comment::post_id.eq(Post::id))
            .alias(PostView::comments),
    ))
    .inner_join::<User>(Post::author_id.eq(User::id))
    .load_as::<PostView>()
    .await?;
```

```rust
// --- SQLite: Binary JsonbObject, JsonbGroupArray & Json ---
use kova::dsl::func::{Json, JsonObject, JsonbGroupArray, JsonbObject};
use kova::query::Select;

let views = sqlite_client
    .select::<Post>()
    .columns((
        Post::id,
        Post::title,
        Post::content,
        Json(JsonbObject((User::id, User::name))).alias(PostView::author),
        Select::from::<Comment>()
            .columns(Json(JsonbGroupArray(JsonbObject((
                Comment::id,
                Comment::content,
                JsonObject((User::id, User::name)).alias(CommentDto::commenter),
            )))))
            .inner_join::<User>(Comment::user_id.eq(User::id))
            .filter(Comment::post_id.eq(Post::id))
            .alias(PostView::comments),
    ))
    .inner_join::<User>(Post::author_id.eq(User::id))
    .load_as::<PostView>()?;
```

### 1.4 Transaction Pipelines

For multi-step workflows like balance deductions and order creation, run a closure-driven transaction and pipe primary keys or balances returned via `RETURNING` straight into the next query. Returning `Ok` commits automatically; hitting `?` on an error rolls back:

```rust
async fn place_order(buyer: &User, order_amount: f64) -> Result<(i32, f64)> {
    let (new_order_id, remaining_balance) = client
        .transaction(async |tx| {
            // 1. Upsert buyer and retrieve (user_id, current_balance)
            let mut user_rows = tx
                .insert(buyer)
                .on_conflict(User::id)
                .update((User::name.excluded(), User::balance.increment(500.0)))
                .returning((User::id, User::balance))
                .execute_in::<(i32, f64)>()
                .await?;
            let (uid, current_balance) = user_rows.pop().unwrap();

            // 2. Create order and retrieve generated order_id
            let new_order = Order {
                id: 0,
                user_id: uid,
                amount: order_amount,
                is_active: true,
            };
            let mut order_ids = tx
                .insert(&new_order)
                .returning(Order::id)
                .execute_in::<i32>()
                .await?;
            let order_id = order_ids.pop().unwrap();

            // 3. Deduct balance and retrieve updated balance
            let mut balances = tx
                .update::<User>()
                .set((
                    User::balance.decrement(order_amount),
                    User::bio.set("Active Buyer"),
                ))
                .filter(User::id.eq(uid) & User::balance.ge(order_amount))
                .returning(User::balance)
                .execute_in::<f64>()
                .await?;
            let final_balance = balances.pop().unwrap_or(current_balance);

            Ok((order_id, final_balance))
        })
        .await?;

    Ok((new_order_id, remaining_balance))
}
```

### 1.5 Cross-Crate Domain Models & Custom Types

In a multi-crate workspace, a standalone `models` crate typically holds types like `Uuid`, `Url`, `DateTime<Utc>`, or domain-specific types like `Decimal` and `ProductSpec`.

With Kova, your `models` crate needs zero database driver dependencies and zero Newtype wrappers. Define a local marker `pub struct AppCodec;` in your downstream service crate, and a few lines of trait declarations grant your custom types full SQL operator support and cross-type parameter compatibility (see Chapter 11 for the full setup):

```rust
// --- In standalone `models` crate: zero database driver dependencies ---
#[derive(Model)]
pub struct Product {
    pub id: Uuid,
    pub price: Decimal,
    pub vendor_url: Url,
    pub specs: ProductSpec,
    pub created_at: DateTime<Utc>,
}

// --- In downstream service crate: grant SQL capabilities & compatibility rules ---
impl CanMath<AppCodec> for Decimal {}
impl Compatible<Static<Decimal>, AppCodec> for Static<i64> {}

impl CanString<AppCodec> for Url {}
impl<'a> Compatible<Static<Url>, AppCodec> for Static<&'a str> {}

impl CanPgJsonb<AppCodec> for ProductSpec {}

// Switch client codec context (zero runtime cost); custom types now act like built-ins:
let client = base_client.codec::<AppCodec>();

client
    .update::<Product>()
    .set((
        Product::price.increment(500_i64),            // Increment Decimal column directly by i64
        Product::vendor_url.set("https://apple.com"), // Assign &str directly to Url column
        Product::specs.set(&new_specs),               // Write ProductSpec as JSONB
    ))
    .filter(
        Product::id.eq(&target_uuid)                  // Zero-copy 16-byte slice bind for &Uuid
            & Product::vendor_url.ilike("https://%")  // Call ILIKE directly on Url column
            & Product::specs.json_path_text("warranty").eq("2_years")
    )
    .execute()
    .await?;
```

---

## 2. Client & Connection Pool

### 2.1 Why is PostgreSQL Async while SQLite is Sync?

Plenty of database libraries pitch "one unified async interface for all databases" as a selling point. Papering over physical differences in underlying I/O models almost always results in severe performance regressions.

PostgreSQL is a networked client/server architecture. Once your process sends a packet over TCP, it spends most of its time waiting on network I/O. Non-blocking `async`/`await` allows worker threads to yield the CPU while waiting on the wire and service thousands of concurrent requests.

SQLite is the exact opposite. There is no remote server; it is an in-process C library compiled directly into your binary's address space. Every query reads and writes memory pages or local disk files directly on the calling thread, often returning in a few microseconds. Wrapping SQLite in an `async` facade for superficial "API uniformity" forces every microsecond memory read to be boxed into a closure, shipped across threads to a background blocking pool (`spawn_blocking`), and sent back over a channel to wake the async executor. The cross-thread synchronization and context-switching overhead is routinely several times slower than the SQLite query itself.

**Kova's PostgreSQL client is natively asynchronous (requiring `.await` on terminal methods), whereas Kova's SQLite client is natively synchronous (executing directly on the calling thread).** Aside from `.await`, their APIs are identical.

### 2.2 Configuring the PostgreSQL Client

The PostgreSQL client is built on `tokio-postgres` and `deadpool-postgres`. Every query execution automatically invokes `prepare_typed_cached` to cache prepared statement handles per connection—no manual statement management required:

```rust
use kova::{Client, Options};
use kova::options::RecyclingMethod;
use std::time::Duration;

let client = Options::from("host=127.0.0.1 port=5432 user=postgres password=secret dbname=app")
    .max_size(16)                                // Max pool capacity (default: 10)
    .wait_timeout(Duration::from_secs(5))        // Timeout waiting for an available connection
    .create_timeout(Duration::from_secs(5))      // Timeout establishing a physical connection
    .recycle_timeout(Duration::from_secs(5))     // Health-check timeout when recycling a connection
    .recycling_method(RecyclingMethod::Verified) // Recycling strategy (default: Fast; options: Verified / Clean)
    .inspect(|sql, params| {
        // SQL observation hook: fires right before execution; wire into logging or tracing
        println!("SQL: {sql} | Params: {params:?}");
    })
    .build()?;
```

With the `rustls` or `native-tls` feature enabled, inject TLS configuration via `.tls(...)`:

```rust
let client = Options::from("host=db.example.com user=postgres dbname=app sslmode=require")
    .tls(|| {
        let config = rustls::ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();
        Ok(config)
    })
    .build()?;
```

### 2.3 Configuring the SQLite Client & Connection Initialization

The SQLite client is built on `rusqlite` and ships with a lightweight native connection pool (`SqlitePool`) that caches prepared statements via `prepare_cached`.

Rather than pulling in third-party pooling crates, `SqlitePool` uses standard library `Mutex` and `Condvar` primitives to maintain a LIFO stack of warm connections. Checkouts pop the most recently returned connection whose page cache is still hot. When the pool is below capacity and needs to open a new physical connection, the thread drops the `Mutex` lock before performing disk I/O and running initialization SQL, never blocking other threads returning connections to the pool. Calling `.build()` eagerly opens the first physical connection to validate the file path and initialization SQL at startup.

Connect to a file on disk via `Options::from(path)`, or create an in-memory database via `Options::in_memory()`—which pins the pool size to `1` so multiple connections don't spawn isolated in-memory databases.

One crucial SQLite detail: `WAL` journal mode, `synchronous` fsync levels, `foreign_keys` enforcement, and custom scalar functions are per-connection states bound to **each individual physical connection**. `Options` provides an `.init(...)` hook that runs on every new physical connection opened by the pool:

```rust
use kova::{Client, Options};
use std::time::Duration;

let client = Options::from("data/app.db")
    .max_size(8)                                 // Default 8 for files; fixed at 1 for in-memory
    .connection_timeout(Duration::from_secs(10)) // Acts as both pool checkout timeout and SQLite busy_timeout
    .inspect(|sql, params| {
        println!("SQL: {sql} | Params: {params:?}");
    })
    .init(|conn| {
        // 1. Configure per-connection PRAGMAs
        conn.execute_batch(
            "PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL; PRAGMA foreign_keys = ON;"
        )?;

        // 2. Register SQLite's `X REGEXP Y` callback.
        // SQLite's parser recognizes the REGEXP keyword, but binds no implementation by default.
        // Registering this callback unlocks `Column.regexp(pattern)` in queries.
        conn.regexp(|text, pattern| {
            text.contains(pattern)
        })
    })
    .build()?;
```

### 2.4 Organizing `Client` in Your Application

`Client` wraps the underlying connection pool in an `Arc`. Cloning a `Client` is a cheap reference-count bump, and every query method takes `&self`. Two patterns work best in practice:

**Pattern 1: Global Static (`LazyLock`)**
In standalone services, place `Client` inside a `std::sync::LazyLock`. Because both `LazyLock` and `Client` implement `Deref`, you can even name the static variable lowercase `client` and call it directly from anywhere:

```rust
use kova::{Client, Options};
use std::sync::LazyLock;

#[allow(non_upper_case_globals)]
pub static client: LazyLock<Client> = LazyLock::new(|| {
    Options::from("host=127.0.0.1 user=postgres dbname=app")
        .max_size(16)
        .build()
        .expect("Failed to initialize database pool")
});

pub async fn disable_user(uid: i32) -> kova::Result<u64> {
    client
        .update::<User>()
        .set(User::is_active.set(false))
        .filter(User::id.eq(uid))
        .execute()
        .await
}
```

**Pattern 2: Repository Structs**
In layered architectures, pass a `Client` into domain repositories to keep data access encapsulated within module boundaries:

```rust
pub struct UserRepo {
    client: Client,
}

impl UserRepo {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn find_active_by_id(&self, uid: i32) -> kova::Result<Vec<User>> {
        self.client
            .select::<User>()
            .filter(User::id.eq(uid) & User::is_active.eq(true))
            .load()
            .await
    }
}
```

---

## 3. Data Modeling & the Three Relational Roles

In a database-backed application, Rust structs play three distinct roles:

* **Physical tables** in the database that support both reads and writes;
* **Read-only views** that receive results from multi-table joins or aggregations;
* **Temporary virtual tables** representing `WITH cte AS (...)`, referenced and joined only within a single query.

Conflating these roles makes it impossible for the type system to stop someone from calling `delete` on a read-only aggregation view or `insert` on a CTE virtual table. Kova provides three mutually exclusive derive macros that lock down role boundaries at definition time.

### 3.1 `#[derive(Model)]`: Physical Tables

Derive `Model` for structs that map to physical database tables (or queryable FTS5 virtual tables):

```rust
use kova::Model;

#[derive(Model)]
pub struct User {
    #[kova(auto)]
    pub id: i32,
    pub name: String,
    pub bio: Option<String>,
    pub balance: f64,
    pub is_active: bool,
}
```

`#[derive(Model)]` generates three things at compile time:

1. **Table and column name mapping**: The struct identifier is converted to `snake_case` for the table name (`User` → `"user"`), and field names map directly to column names. If a field name is a Rust keyword like `pub r#type: String`, the macro strips the `r#` prefix and maps it to `"type"`.
2. **Zero-cost static column and table constants**: Generates associated column constants of the same name (`User::id` of type `Column<i32, ColumnMeta>`) along with a static table constant `pub const User: Table<User>` for whole-table references like `RowToJson(User)`. These constants hold only compile-time string slices and phantom markers, occupying zero runtime memory.
3. **`#[kova(auto)]` column handling**: Auto-incrementing primary keys (`SERIAL` / `AUTOINCREMENT`) or columns with server-side defaults must be read back during `SELECT` and `RETURNING`, but omitted during `INSERT` so the database can populate them. Fields marked `#[kova(auto)]` are automatically skipped in generated `INSERT` statements and aren't even required to implement `Encode`. When constructing an instance for insertion, supply a placeholder like `id: 0`. Marking every field on a struct with `#[kova(auto)]` is rejected at compile time to prevent invalid zero-column `INSERT` statements.

### 3.2 `#[derive(FromRow)]`: Read-Only Query Views

When a query involves multi-table joins, aggregations, or nested JSON assembly, define a target struct and derive `FromRow`:

```rust
use kova::FromRow;

#[derive(FromRow)]
pub struct OrderSummary {
    pub user_id: i32,
    pub user_name: String,
    pub total_spent: f64,
}
```

`FromRow` also generates column constants like `OrderSummary::total_spent` so you can write `Sum(Order::amount).alias(OrderSummary::total_spent)` in projections. However, its internal metadata is read-only `IdentifierMeta` rather than physical `ColumnMeta`:

* It implements `FromRow`, but neither `Model` nor `Relation`. You can only pass it to `.load_as::<OrderSummary>()` at the end of a query; the compiler forbids initiating `select`, `insert`, `update`, `delete`, or `join` on it.
* Its column constants lack `CanAssignment` and `CanConflict`—methods like `.set()`, `.increment()`, and `.excluded()` do not exist on `OrderSummary::total_spent`.

**Adaptive Fast/Slow Path Row Decoding**
When decoding a database row into a struct, looking up columns by name is safe against reordering but incurs string-matching overhead; reading by positional index is fastest but breaks if columns are reordered in `.columns(...)`.
Kova's `Row::get_column` combines both: when reading the `idx`-th struct field, it first checks whether the `idx`-th column name in the result set matches the target field name. On a match—which covers virtually all standard queries—it extracts the value in `O(1)` by physical index. Only if the projection order was scrambled does it fall back to name lookup.

### 3.3 `#[derive(Cte)]`: Common Table Expressions (Virtual Tables)

When writing `WITH cte_name AS (SELECT ...)`, you need a symbol representing the temporary virtual table and its columns. Derive `Cte`:

```rust
use kova::Cte;

#[derive(Cte)]
pub struct VipStats {
    pub user_id: i32,
    pub total_amount: f64,
    pub order_count: i64,
}
```

`VipStats` implements `Relation` and `Cte`, exposing read-only column constants. It can only be bound via `.with::<VipStats>(subquery)` and joined within that same query; it cannot initiate a standalone query or be decoded via `.load_as::<VipStats>()`.

`Model`, `FromRow`, and `Cte` are strictly mutually exclusive—deriving any two on the same struct is a compile error.

---

## 4. CRUD & the Typestate Machine

SQL clauses have strict ordering and contextual dependencies:

* An `UPDATE` without `SET` is a syntax error;
* An `INSERT ... ON CONFLICT (id)` without `DO NOTHING` or `DO UPDATE` is an incomplete statement;
* Only `DO UPDATE` can be followed by `WHERE`; attaching `WHERE` to `DO NOTHING` is invalid;
* Once a mutation adds `RETURNING`, it must read the returned rows rather than calling an interface that only returns an affected row count.

To catch these errors at compile time without bloating type signatures, Kova uses a **fixed-width typestate machine**. Each builder tracks its phase using one or two zero-sized type markers (ZSTs). State transitions swap out a single marker parameter: methods invalid in the current phase simply do not exist on that type, while the builder's type signature width remains constant.

### 4.1 Inserting Data (`INSERT`) & Automatic Batch Splitting

`client.insert(...)` is polymorphic over its input: it accepts an owned entity `User`, a borrow `&User`, a `Vec<User>` (or `&Vec<User>`), or a slice `&[User]`:

```rust
let alice = User {
    id: 0,
    name: "Alice".into(),
    bio: None,
    balance: 500.0,
    is_active: true,
};
let bob = User {
    id: 0,
    name: "Bob".into(),
    bio: Some("Systems Engineer".into()),
    balance: 800.0,
    is_active: true,
};

// Single borrowed insert (zero cloning)
client.insert(&alice).execute().await?;

// Slice batch insert
client.insert(&[alice, bob][..]).execute().await?;
```

During bulk inserts, Kova automatically handles two edge cases that routinely cause production incidents:

1. **Empty slice short-circuiting**: A dynamically filtered `Vec<User>` might be empty at runtime. When Kova detects zero rows, it short-circuits and returns `Ok(0)` (or an empty `Vec`) immediately without emitting invalid empty `VALUES` SQL.
2. **Automatic batch splitting**: Database wire protocols enforce hard limits on bind parameters per statement—`65,535` for PostgreSQL and `32,766` for SQLite. For `User` (which has 4 insertable columns), a single PostgreSQL `INSERT` can hold at most `65535 / 4 = 16,383` rows. Pass 20,000 rows at once and Kova automatically splits them into two `INSERT` statements (16,383 rows in batch one, 3,617 rows in batch two, resetting placeholders to `$1` for the second batch), executing them sequentially and summing the affected rows. To guarantee all-or-nothing atomicity across multiple split batches, wrap the `INSERT` call inside `client.transaction(...)`.

`client.insert::<T>(...)` also accepts a `Select::from::<M>()` subquery directly to emit `INSERT INTO ... SELECT ...`:

```rust
use kova::query::Select;

client
    .insert::<Post>(
        Select::from::<User>()
            .columns((User::name, User::name, User::id))
            .filter(User::is_active.eq(true)),
    )
    .execute()
    .await?;
```

### 4.2 Conflict Handling State Machine (`ON CONFLICT` / Upsert)

The conflict state parameter on the `Insert` builder transitions through three steps:

1. **Initial state `NoConflict`**: `.execute()` is available immediately. Neither `.do_nothing()`, `.update(...)`, nor `.filter(...)` exists on the builder.
2. **Intermediate state `ConflictTarget`**: Calling `.on_conflict(...)` with a target—a single column `User::id`, a tuple including scalar functions `(User::id, Lower(User::name))`, or a PostgreSQL constraint name `Constraint("uk_user_name")`—transitions the builder to `ConflictTarget`.
   Because `ConflictTarget` does not implement the internal `CanExecute` trait, `.execute()` vanishes from the type. You must specify a conflict resolution action before executing.
3. **Terminal states `ConflictNothing` and `ConflictUpdate`**:
    * Calling `.do_nothing()` transitions to `ConflictNothing`, restoring `.execute()`, still without `.filter(...)`.
    * Calling `.update(...)` transitions to `ConflictUpdate`, restoring `.execute()` and **exclusively unlocking `.filter(...)`** to express `DO UPDATE SET ... WHERE ...`.

```rust
use kova::dsl::func::Lower;
use kova::dsl::Constraint; // PostgreSQL-exclusive

// 1. Do nothing on conflict
client
    .insert(&alice)
    .on_conflict(Constraint("uk_user_name"))
    .do_nothing()
    .execute()
    .await?;

// 2. Update on conflict (`.excluded()` renders as `excluded."bio"`)
client
    .insert(&alice)
    .on_conflict((User::id, Lower(User::name)))
    .update((
        User::bio.excluded(),
        User::balance.increment(100.0),
    ))
    .filter(User::is_active.eq(true))
    .execute()
    .await?;
```

> **Rendering detail**: SQL syntax requires column names inside `ON CONFLICT ("id", lower("name"))` to be unqualified, whereas columns in the subsequent `DO UPDATE SET` and `WHERE` clauses must be table-qualified (`"user"."balance"`) to prevent ambiguity against `excluded`. Kova's renderer automatically sets `unqualify_columns = true` upon entering the `ON CONFLICT` target list and restores qualification immediately after the closing parenthesis.

### 4.3 Updating Data (`UPDATE`) & the `Unset` Guard

A fresh builder created via `client.update::<User>()` starts in the `Unset` state, where neither `.execute()` nor `.returning(...)` exists, making it impossible to forget `SET`.

Calling `.set(...)` transitions the builder to `Assigned`, unlocking execution methods and `.returning(...)`. Once `.returning(...)` transitions the builder to `Returning`, `.set(...)` is closed off to enforce canonical clause ordering:

```rust
// 1. Basic assignment and atomic numeric increment/decrement
client
    .update::<User>()
    .set((
        User::name.set("Alice Smith"),
        User::balance.increment(50.0), // Renders: "balance" = "user"."balance" + $2
    ))
    .filter(User::id.eq(1))
    .execute()
    .await?;

// 2. Cross-table update (UPDATE ... SET ... FROM ...)
// `.from::<R>()` accepts a single table or a tuple of tables, e.g., `.from::<(Order, Comment)>()`
client
    .update::<User>()
    .set(User::balance.decrement(Order::amount))
    .from::<Order>()
    .filter(User::id.eq(Order::user_id) & Order::id.eq(10))
    .execute()
    .await?;
```

### 4.4 Deleting Data (`DELETE`) & `RETURNING` Dispatch

`Insert`, `Update`, and `Delete` share the same return-mode typestate (`NoReturn` vs. `Returning`):

* By default, builders are in `NoReturn` and expose only `.execute()`, returning the affected row count `Result<u64>`.
* Calling `.returning(...)` transitions the builder to `Returning`. `.execute()` disappears and is replaced by two row-extraction methods:
    1. `.execute_in::<U>()`: Extracts columns positionally into a scalar (`i32`) or a tuple (`(i32, String)`).
    2. `.execute_as::<M>()`: Maps columns by field name into a struct implementing `FromRow` (including `Model`).

```rust
// 1. Delete and positionally extract deleted IDs
let deleted_ids = client
    .delete::<User>()
    .filter(User::is_active.eq(false))
    .returning(User::id)
    .execute_in::<i32>()
    .await?;

// 2. Update and map the modified rows back into `User` structs
let updated_users = client
    .update::<User>()
    .set(User::balance.set(0.0))
    .filter(User::balance.lt(0.0))
    .returning((User::id, User::name, User::bio, User::balance, User::is_active))
    .execute_as::<User>()
    .await?;
```

### 4.5 Querying Data (`SELECT`) & Clause Mutation Semantics

`client.select::<T>()` provides three terminal extraction methods:

* `.load()`: Maps rows directly into the primary table model `Vec<T>` without type annotations.
* `.load_as::<U>()`: Maps rows into any `FromRow` view struct, or into a tuple of joined models such as `(User, Option<Order>)`.
* `.load_in::<U>()`: When projecting specific columns or aggregates, extracts values positionally into a scalar or a tuple of up to 32 elements (e.g., `(i32, String, Option<f64>)`).

For `.load_in`, `.load_as`, `.execute_in`, and `.execute_as`, Rust's bidirectional type inference lets you specify the target type either via turbofish `::<U>` on the method call or via variable annotation `let rows: Vec<U> = ...`:

```rust
// 1. load(): Returns Vec<User> directly
let users = client
    .select::<User>()
    .filter(User::is_active.eq(true))
    .order_by((
        User::balance.desc().nulls_last(),
        User::name.asc().nulls_first(),
        User::id,                // Default sort direction
        User::is_active.eq(true) // ORDER BY also accepts boolean Conditions directly (defaults to ASC)
    ))
    .limit(20)
    .offset(40)
    .load()
    .await?;

// 2. load_in(): Extract a single scalar column (annotating `Vec<String>` is identical to `.load_in::<String>()`)
let distinct_names: Vec<String> = client
    .select::<User>()
    .distinct()
    .columns(User::name)
    .load_in()
    .await?;
```

Because builders are plain structs with flat types, you can pass them across functions or mutate them across `if`/`else` branches. When mutating a builder in place, single-slot clauses—such as `.columns(...)`, `.filter(...)`, `.group_by(...)`, `.having(...)`, `.order_by(...)`, `.limit(...)`, `.offset(...)`, and `Update`'s `.set(...)` / `.from(...)`—**overwrite** their previous state cleanly, dropping any old bind parameters so nothing leaks into the final SQL (under SQLite, calling `.offset(n)` without `.limit(...)` automatically emits `limit -1 offset n`). Multi-item clauses—`.with::<C>(...)`, joins, `.lock(...)`, and set operations like `.union(...)`—**append** nodes in the order they are called.

---

## 5. Expressions, Boolean Algebra & Type Compatibility

Most statically typed ORMs turn "type safety" into pedantic ceremony: if `bio` is `Option<String>`, you aren't allowed to write `User::bio.eq("architect")`—you are forced to write `User::bio.eq(Some(String::from("architect")))`; if a column is `f64`, you can't pass an `f32` or `i16` variable; and if `discount` is `Option<f64>`, you aren't even allowed to call `Order::discount.add(10.0)`.

Kova's rule is simple: **the type system should eliminate tedious boilerplate conversions for you, stepping in only where precision loss, sign overflow, or null pollution would actually occur.**

### 5.1 Comparison Methods & the Boolean Algebra Tree (`&`, `|`, `!`)

Every column and expression node (wrapped in `Node<T>`) exposes SQL comparison methods directly: standard comparisons (`.eq`, `.ne`, `.gt`, `.ge`, `.lt`, `.le`), null checks (`.is_null`, `.is_not_null`), range and set predicates (`.between`, `.not_between`, `.in_list`, `.not_in_list`, `.in_select`, `.not_in_select`), and pattern matching (`.like`, `.not_like`).

The right-hand operand is dispatched through Kova's internal `Open` (any encodable Rust value) and `Close` (AST node) dual-track mechanism: pass a literal or variable (`Order::amount.gt(100.0)`) or another column or function expression (`Order::amount.between(0.0, User::balance)`).

Combine multiple conditions using Rust's bitwise operators: **`&` for `AND`, `|` for `OR`, and `!` for `NOT`**.

```rust
let cond = User::is_active.eq(true)
    & !User::name.like("%test%")
    & (User::balance.between(100.0, 5000.0) | User::id.in_list([1, 2, 3]))
    & User::bio.is_not_null();
```

1. **Precedence alignment and N-ary tree flattening**: Rust's operator precedence (`!` > `&` > `|`) matches standard SQL precedence (`NOT` > `AND` > `OR`). Chaining `a & b & c` does not allocate a deeply nested binary tree; `Condition` flattens consecutive conjunctions in-place into a single contiguous `ConditionNode::And(Vec<ConditionNode>)` vector. Parentheses are emitted only when an `OR` is nested inside an `AND` (or vice versa), producing clean SQL with zero redundant parens.
2. **Safe fallback for empty `in_list`**: If the iterator passed to `User::id.in_list(ids)` is empty at runtime, emitting `IN ()` would trigger a SQL syntax error. Kova renders an empty `InList` as `false` and an empty `NotInList` as `true`.

### 5.2 Chained Arithmetic (`.add` / `.sub` / `.mul` / `.div`)

Any column or expression whose output satisfies `CanMath<X>` can call `.add(v)`, `.sub(v)`, `.mul(v)`, and `.div(v)`. The resulting `Math` node preserves the left-hand operand's output type `T::Output`, allowing arbitrary chaining followed by comparisons, ordering, grouping, or `.alias(...)`:

```rust
let rows = client
    .select::<User>()
    .inner_join::<Order>(Order::user_id.eq(User::id))
    .columns((
        User::id,
        // Renders: ((("user"."balance" - "order"."amount") + $1) * $2) as "balance"
        User::balance.sub(Order::amount).add(10.0).mul(0.95).alias(User::balance),
    ))
    .filter(User::balance.sub(Order::amount).ge(0.0))
    .load_in::<(i32, f64)>()
    .await?;
```

### 5.3 Null Penetration, Reference Penetration & Lossless Widening

Kova's operand compatibility matrix (`Compatible<Target, X>`) enforces three compile-time rules:

**1. Capability penetration for nullable columns (`Column<Option<T>>`)**
In SQL, nullable columns naturally participate in arithmetic, aggregation, and string matching. Kova implements transparent `TypedNode` and capability penetration for `Option<T>` (`impl<T: CanMath<X>, X> CanMath<X> for Option<T>`). Even when `Order::discount` is `Column<Option<f64>>` and `User::bio` is `Column<Option<String>>`, you can write `Order::discount.add(0.1)`, `Sum(Order::discount)`, or `Lower(User::bio)` directly.

**2. One-way null boundary and zero-copy reference penetration**
When targeting a nullable `Column<Option<T>>`, you can pass `Option<T>`, `&Option<T>`, `Option<&T>`, or unwrapped values `T`, `&T`, `&&T`, and `Box<T>` directly—writing `User::bio.eq("developer")` works without ceremony.

For a non-nullable `Column<T>`, references (`&T`, `&&T`), `Box<T>`, and borrowed slices (`&str` and `char` for `String`; `&[u8]` and `&[u8; N]` for `Vec<u8>`; `&[&str]` for `Vec<String>`) all pass straight through via `Cow::Borrowed` with zero heap allocation. **What will not pass is an `Option<T>` or a nullable column**—writing `User::name.eq(Some("alice"))` or `Order::amount.add(Order::discount)` is rejected at compile time to block reverse null pollution.

**3. Permitting lossless widening, blocking lossy conversions**
Narrow integers widen into wider integers or higher-precision floats without losing a single bit: pass `i8`, `i16`, `u8`, or `u16` directly into `Column<i32>`, `Column<i64>`, or `Column<f64>`; pass `i32` or `u32` into `Column<i64>` or `Column<f64>`; and pass `f32` into `Column<f64>`.

Conversely, any conversion that could truncate or overflow is rejected at compile time:

* **No narrowing**: `i64` → `Column<i32>` and `f64` → `Column<f32>` are forbidden;
* **No same-width unsigned-to-signed**: `u32` → `Column<i32>` and `u64` → `Column<i64>` are forbidden because the high bit would wrap into a negative number;
* **No lossy integer-to-float**: `i64` into `Column<f64>` and `i32` into `Column<f32>` are forbidden. Because IEEE 754 `f64` has only 53 bits of significand precision (`f32` has 24), converting an `i64` larger than 2<sup>53</sup> into `f64` silently drops low-order bits. Kova shuts that door at compile time.

> **Opt-in narrow decoding (`narrow-cast` feature)**
> PostgreSQL and SQLite wire protocols natively transmit signed `i16`, `i32`, `i64`, and `f64`. By default, Kova implements lossless upward `Encode` for `i8`, `u8`, `i16`, `u16`, `u32`, and `f32`, without exposing direct `Decode` into those narrow types or `u64`/`usize`/`isize` (`i16` and `f32` are decode-enabled by default on PostgreSQL, and require `narrow-cast` on SQLite).
> If your application needs to decode database columns into these types, enable `features = ["narrow-cast"]` in `Cargo.toml`. Kova performs checked runtime conversions via `try_from`, returning `Error::ParseError` or `Error::ConversionError` on out-of-range values—never silently truncating.

---

## 6. Joins, Lexical Tag Scopes & Subqueries

### 6.1 Polymorphic Joins & `LEFT JOIN` Nullable Row Unpacking

`Select` provides five join methods: `.inner_join::<R>(...)`, `.left_join::<R>(...)`, `.right_join::<R>(...)`, `.full_join::<R>(...)`, and parameterless `.cross_join::<R>()`.

Join constraints implement the `IntoJoinConstraint` protocol, supporting two forms:

1. **Boolean `Condition` (emits `ON`)**: e.g., `.inner_join::<User>(Order::user_id.eq(User::id) & User::is_active.eq(true))`. Multiple conditions must be combined with `&` or `|`; the compiler statically rejects condition tuples `(cond1, cond2)` or mixed tuples `(cond, col)`.
2. **Shared column or column tuple (emits `USING`)**: When joining on identically named columns, pass a single column `.left_join::<Post>(Post::id)` or a column tuple `.right_join::<Comment>((Comment::id, Comment::user_id))`, which renders as `USING ("id", "user_id")`.

**Unpacking Null Right-Hand Rows in `LEFT JOIN`**
In a `LEFT JOIN`, an unmatched right-hand table returns `NULL` across all of its columns. In Kova, for any type `T` implementing `FromRow` (including `Model`), `Option<T>` automatically implements `FromRow`. When decoding `Option<Order>` hits `UnexpectedNull` on a non-nullable field, the decoder advances the column cursor forward by `Order::COLUMN_COUNT` columns and returns `Ok(None)`:

```rust
// If a User has no Orders, the second tuple element decodes cleanly as `None` without misaligning the cursor
let pairs = client
    .select::<User>()
    .left_join::<Order>(Order::user_id.eq(User::id))
    .load_as::<(User, Option<Order>)>()
    .await?;
```

### 6.2 Source Disambiguation: `.tag::<M>()` vs. `.alias(...)`

In rendered SQL, both `.tag::<M>()` and `.alias(...)` emit `AS "..."`, but in the type system they govern opposite directions:

**`.alias(ident)` renames output projections (`SELECT expr AS name`)**
Computed expressions like `Sum(Order::amount)` have no inherent column name. To let downstream `FromRow` structs or JSON object constructors bind to them by name, attach an alias via `.alias(OrderSummary::total_spent)`. If the target alias doesn't belong to an existing struct, declare a standalone identifier with `kova::dsl::Identifier`:

```rust
use kova::dsl::Identifier;
use kova::dsl::func::{JsonbBuildObject, Lower};

const LOWER_NAME: Identifier<String> = Identifier::new("lower_name");
// Renders: jsonb_build_object('id', "user"."id", 'lower_name', lower("user"."name"))
let expr = JsonbBuildObject((User::id, Lower(User::name).alias(LOWER_NAME)));
```

**`.tag::<M>()` disambiguates input table sources (`FROM "user" AS "user_0"`)**
When the same `User` table is self-joined as both buyer and seller, or the same set-returning function is called twice, the static constant `User::id` becomes ambiguous.

Traditional query builders retreat to stringly-typed aliases like `table.alias("u1")` and `col("u1.id")`, which invite typos and collide inside nested subqueries. Kova replaces string aliases with compile-time type identity (`TypeId`)—just declare zero-sized marker structs (`struct Buyer; struct Seller;`) as tags:

```rust
struct Buyer;
struct Seller;

let rows = client
    .select::<User>()
    .tag::<Buyer>() // Tag the primary table as Buyer
    .inner_join::<User>(User::id.tag::<Buyer>().eq(User::id.tag::<Seller>()))
    .tag::<Seller>() // Tag the joined table as Seller
    .columns((
        User::id.tag::<Buyer>(),
        User::name.tag::<Seller>(),
    ))
    .filter(User::is_active.tag::<Buyer>().eq(true))
    .load_in::<(i32, String)>()
    .await?;
```

The underlying `TagRegistry` assigns sequential indices and deterministically emits:

```sql
select "user_0"."id", "user_1"."name"
from "user" as "user_0"
inner join "user" as "user_1" on "user_0"."id" = "user_1"."id"
where "user_0"."is_active" = $1
```

This tagging system is backed by four guarantees:

1. **Typestate-enforced placement**: On `Select`, `.tag::<M>()` only exists immediately after declaring the primary table (`JustAddFrom`) or adding a join (`JustAddJoin`), while set-returning functions are tagged directly on their node (`Unnest(Post::tags).tag::<M>()`). Each table or SRF source can be tagged at most once; the moment you move on to `.filter()` or `.columns()`, `.tag()` disappears.
2. **Deterministic numbering independent of reference order**: Even if you reference `Seller` before `Buyer` in `.columns(...)`, the renderer pre-registers tags strictly in `FROM -> JOIN` declaration order (`Buyer -> 0, Seller -> 1`).
3. **Lexical subquery scope isolation and outer correlation capture**: Entering a subquery pushes a new lexical scope (`enter_scope`) onto `TagRegistry`. If the subquery also tags a table with `.tag::<Buyer>()`, it receives a fresh local index `"user_2"` without shadowing the outer `"user_0"`. If the subquery references `.tag::<Seller>()`—which was only declared in the outer query—the registry walks up the enclosing scopes like a closure capturing an outer variable and resolves it to `"user_1"`, making correlated subqueries effortless and safe.
4. **Stack-first allocation**: `TagRegistry` inlines 8 slots directly on the stack (`[TypeId; 8]`). Standard queries resolve tags with zero heap allocations, spilling over to a `Vec` only when a query exceeds 8 distinct tags.

### 6.3 Subquery Isolation (`OuterQuery` vs. `InnerQuery`) & Set Operations

The `Select` builder uses a dispatch type parameter `D` to strictly separate top-level executable queries from embedded subqueries:

* **Top-level query (`OuterQuery<'c, E>`)**: Created via `client.select::<User>()`, holds a database connection reference, and exposes execution methods like `.load()`, but **does not** implement `IntoSelectStatement` or `IntoProjection`—it cannot be embedded inside another query.
* **Inner subquery (`InnerQuery`)**: Created via the static method `Select::from::<User>()`, holds no database connection, exposes **zero** execution methods, and exists purely as an AST node to be embedded into an outer query.

```rust
use kova::query::Select;
use kova::dsl::func::{Count, Wildcard};

// 1. WHERE IN / NOT IN subquery (uses `render_bare` internally to avoid redundant double parentheses)
let active_buyers = client
    .select::<User>()
    .filter(
        User::id.in_select(
            Select::from::<Order>()
                .columns(Order::user_id)
                .filter(Order::amount.gt(1000.0)),
        )
    )
    .load()
    .await?;

// 2. Correlated scalar subquery inside SELECT projections
let order_counts = client
    .select::<User>()
    .columns((
        User::id,
        Select::from::<Order>()
            .columns(Count(Wildcard))
            .filter(Order::user_id.eq(User::id))
            .alias(VipStats::order_count),
    ))
    .load_in::<(i32, i64)>()
    .await?;

// 3. Set operations (union / union_all / intersect / except)
// Standard SQL requires a trailing ORDER BY after set operations to use unqualified column names (`order by "id" desc`).
// Kova's renderer automatically strips table prefixes from trailing ORDER BY clauses when set operations are present.
let combined = client
    .select::<User>()
    .columns((User::id, User::name))
    .filter(User::is_active.eq(true))
    .union_all(
        Select::from::<User>()
            .columns((User::id, User::name))
            .filter(User::balance.gt(5000.0)),
    )
    .order_by(User::id.desc())
    .limit(10)
    .load_in::<(i32, String)>()
    .await?;
```

---

## 7. Standard Functions, Aggregate Modifiers & Window Analytics

Every SQL function constructor in `kova::dsl::func` is a typed algebraic node that enforces input capabilities and computes its own output type.

### 7.1 Standard Scalar Functions & Capability Transitions

Scalar functions transform both values and capabilities as you nest them. `Lower(expr)` and `Upper(expr)` demand `CanString<X>` and yield `Static<String>`, keeping string operators active. `Abs(expr)` demands `CanMath<X>` and preserves the exact numeric type `expr::Output`.

`Length(expr)` goes a step further: it takes a `CanString<X>` expression and **transitions the output type to `Static<i64>`**. Wrap a string column in `Length(...)` and `.like()` vanishes from the node, while integer arithmetic like `.add(10_i64)` unlocks automatically:

```rust
use kova::dsl::func::{Abs, Length, Lower, Upper};

let rows = client
    .select::<User>()
    .columns((
        Lower(Upper(User::name)),       // Upper outputs String, feeding directly into Lower
        Length(User::name).add(10_i64), // Length outputs i64, unlocking .add()
    ))
    .filter(Length(Lower(User::name)).gt(5_i64))
    .load_in::<(String, i64)>()
    .await?;
```

Structural capabilities like `CanGroup` and `CanConflict` also penetrate scalar wrappers recursively (`impl<M: CanConflict, E: CanConflict> CanConflict for Scalar<M, E>`). `Lower(User::name)` is accepted as an `ON CONFLICT` expression index target because `User::name` is a physical column; `Lower(VipStats::user_id)` (wrapping a read-only view column) or `Abs(Sum(Order::amount))` (wrapping an aggregate) is rejected recursively by the compiler.

### 7.2 Aggregate Functions & Chained Modifiers (`distinct` / `order_by` / `filter`)

Kova provides five cross-engine standard aggregates: `Count(expr)` (which also accepts `Wildcard` for `count(*)` and always outputs `Static<i64>`), `Sum(expr)` and `Avg(expr)` (requiring `CanMath<X>` and preserving the numeric type), plus polymorphic `Min(expr)` and `Max(expr)`.

Because aggregate nodes implement `CanDistinct`, `CanOrderBy`, and `CanFilter`, you can chain `.distinct()`, in-parentheses `.order_by(...)`, and `.filter(...)` (`FILTER (WHERE ...)`) directly onto them—methods that do not exist on plain columns, scalar functions, or dedicated window functions:

```rust
use kova::dsl::func::{Avg, Count, Lower, Max, Min, Sum, Wildcard};

let stats = client
    .select::<Order>()
    .inner_join::<User>(Order::user_id.eq(User::id))
    .columns((
        // Chaining distinct, order_by, and filter on a single aggregate:
        // Renders: count(distinct lower("user"."name") order by "order"."id" desc nulls first) filter (where "order"."is_active" = $1)
        Count(Lower(User::name))
            .distinct()
            .order_by(Order::id.desc().nulls_first())
            .filter(Order::is_active.eq(true)),
        Sum(Order::amount)
            .filter(Order::amount.gt(100.0)),
        Avg(Order::amount),
        Min(Order::amount),
        Max(Order::amount),
    ))
    .load_in::<(i64, Option<f64>, Option<f64>, Option<f64>, Option<f64>)>()
    .await?;
```

> **Nested modifier isolation**: When an aggregate wraps another function (e.g., `Count(Lower(User::name)).distinct().order_by(...)`), `RenderContext` consumes `distinct` and `order_by` via `ctx.agg_modifiers.take()` the instant it enters the outer `count(`, before recursively rendering the inner `lower(...)`. Outer modifiers never leak into inner sub-expressions.

### 7.3 Window Functions (`OVER`) & Frame Bounds (`FrameBound`)

Every aggregate function and dedicated window function implements `CanWindow`, unlocking `.over(Window()...)`. `Window()` lets you specify `.partition_by(...)` (guarded by `CanGroup` to prevent aggregates inside partition clauses), window-local `.order_by(...)`, and three window frame modes (`.rows(start, end)`, `.range(start, end)`, `.groups(start, end)`):

```rust
use kova::dsl::func::{Count, FrameBound, Sum, Window};

let running_totals = client
    .select::<Order>()
    .columns((
        Order::id,
        Order::user_id,
        // 1. Total orders partitioned by user
        Count(Order::id).over(Window().partition_by(Order::user_id)),
        // 2. Rolling cumulative sum: ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
        Sum(Order::amount).over(
            Window()
                .partition_by(Order::user_id)
                .order_by(Order::id.asc())
                .rows(FrameBound::UnboundedPreceding, Some(FrameBound::CurrentRow)),
        ),
        // 3. Sliding window count: RANGE BETWEEN 1 PRECEDING AND 2 FOLLOWING
        Count(Order::id).over(
            Window()
                .order_by(Order::id.asc())
                .range(FrameBound::Preceding(1), Some(FrameBound::Following(2))),
        ),
    ))
    .load_in::<(i32, i32, i64, f64, i64)>()
    .await?;
```

### 7.4 Common Table Expressions (`WITH` / CTE)

Combine `#[derive(Cte)]` from Section 3.3 with `.with::<C>(subquery)` to flatten multi-stage aggregations into readable pipelines:

```rust
use kova::Cte;
use kova::dsl::func::{Count, Sum, Wildcard};
use kova::query::Select;

#[derive(Cte)]
pub struct VipStats {
    pub user_id: i32,
    pub total_amount: f64,
    pub order_count: i64,
}

let vips = client
    .select::<User>()
    .with::<VipStats>(
        Select::from::<Order>()
            .columns((
                Order::user_id,
                Sum(Order::amount).alias(VipStats::total_amount),
                Count(Wildcard).alias(VipStats::order_count),
            ))
            .group_by(Order::user_id)
            .having(Sum(Order::amount).gt(1000.0)),
    )
    .inner_join::<VipStats>(User::id.eq(VipStats::user_id))
    .columns((User::name, VipStats::total_amount, VipStats::order_count))
    .order_by(VipStats::total_amount.desc())
    .load_in::<(String, f64, i64)>()
    .await?;
```

---

## 8. PostgreSQL Native Features & Higher-Order Functions

Under the `postgres` feature, Kova wires PostgreSQL's analytical window functions, JSONB operator chains, 1D arrays, and Set-Returning Functions (SRFs) directly into the core AST via `PostgresExtension`. For the examples below, `Post` has a `tags: Vec<String>` array column and `Product` has a `specs: ProductSpec` JSONB column.

### 8.1 Dedicated Window Functions (Ranking, Bucketing & Cross-Row Offsets)

Beyond attaching `.over(...)` to standard aggregates, the PostgreSQL engine provides dedicated window constructors. These nodes implement `CanWindow` but omit `CanDistinct` and `CanFilter`, making invalid syntax like `row_number() filter (...)` impossible to express.

Ranking functions (`RowNumber()`, `Rank()`, `DenseRank()`) output `Static<i64>`, distribution functions (`PercentRank()`, `CumeDist()`) output `Static<f64>`, and `Ntile(buckets)` outputs `Static<i32>`. For cross-row comparisons, `Lag(expr)` and `Lead(expr)` inherit the wrapped expression's exact output type and domain capabilities—so `Lag(Order::amount)` retains `CanMath` and can call `.add(10.0)` directly. Chain `.offset(n)` with an `i32` literal or integer column, and `.default(val)` with a fallback value or column of type `expr::Output`:

```rust
use kova::dsl::func::{CumeDist, DenseRank, Lag, Lead, Ntile, PercentRank, Rank, RowNumber, Window};

let metrics = client
    .select::<Order>()
    .columns((
        RowNumber().over(Window().partition_by(Order::user_id).order_by(Order::id.asc())),
        Rank().over(Window().order_by(Order::amount.desc())),
        DenseRank().over(Window().order_by(Order::amount.desc())),
        PercentRank().over(Window().order_by(Order::amount.asc())),
        CumeDist().over(Window().order_by(Order::amount.asc())),
        Ntile(4).over(Window().partition_by(Order::user_id)),
        // Amount from 2 rows back, defaulting to 0.0 when out of bounds
        Lag(Order::amount)
            .offset(2)
            .default(0.0)
            .over(Window().order_by(Order::id.asc())),
        // Dynamic column offset and column fallback: step taken from Order::user_id, fallback to current Order::amount
        Lead(Order::amount)
            .offset(Order::user_id)
            .default(Order::amount)
            .over(Window().order_by(Order::id.asc())),
    ))
    .load_in::<(i64, i64, i64, f64, f64, i32, f64, f64)>()
    .await?;
```

### 8.2 JSONB Construction, Aggregation & Path Navigation Chains

PostgreSQL JSONB operations are guarded by the `CanPgJsonb<X>` trait; plain `Column<String>` does not implement it, preventing accidental JSONB operator calls on text columns.

**1. Constructors and Aggregates**

* `JsonbBuildObject(projections)`: Assembles columns or `.alias()`-bound expressions into `jsonb_build_object('key', val, ...)`. Single quotes in keys are automatically escaped (`'k''ey'`); passing an unaliased computed expression like a bare `Lower(User::name)` returns `Error::QueryError` at build time rather than emitting broken SQL.
* `JsonbAgg(expr)`: Aggregates rows into a JSONB array. In raw PostgreSQL, `jsonb_agg` over an empty set returns `NULL` instead of `'[]'::jsonb`, which routinely crashes application-layer `Vec<T>` deserialization. When rendering `JsonbAgg`, **Kova automatically wraps the outermost expression—even after `DISTINCT`, `ORDER BY`, `FILTER`, or `OVER`—in `coalesce(jsonb_agg(...), '[]'::jsonb)`**, guaranteeing a valid empty array on zero rows.
* `JsonbObject(keys_arr, vals_arr)` and `JsonbObjectSingle(kv_arr)`: Require inputs satisfying `CanPgArray<X>`, mapping to two-array zip construction and single-array key-value pair construction respectively.
* `RowToJson(table)`: Accepts a table constant directly (`RowToJson(Post)` or tagged `RowToJson(User.tag::<T1>())`), rendering `row_to_json("user_0")`.

**2. Path Navigation & Predicates (State Transitions and Operator Precedence)**
On any node satisfying `CanPgJsonb<X>` (such as `Product::specs`), navigation methods are typed according to what they return:

* **JSONB-preserving chains (`JsonbChain` → `Dynamic` with `CanPgJsonb`)**: `.json_path(key)` (`->`), `.json_path_array(keys)` (`#>`), `.json_concat(val)` (`||`), `.json_delete(key)` (`-`), and `.json_delete_path(keys)` (`#-`) all return a JSONB node that can be chained further.
  *Precedence protection*: In PostgreSQL's grammar, deletion `-` binds tighter than path extraction `->`. Left unparenthesized, `specs -> 'hardware' - 'serial_no'` is parsed as `specs -> ('hardware' - 'serial_no')`. Kova's renderer inspects the left operand: a plain column renders as `"specs" - $1`, whereas a composite `->` base is automatically parenthesized as `("specs" -> $1) - $2`.
* **Text-extracting terminators (`JsonbText` → `Static<String>`)**: `.json_path_text(key)` (`->>`) and `.json_path_text_array(keys)` (`#>>`) transition the node's output to `Static<String>`. String methods like `.like()`, `.ilike()`, and `.matches()` unlock immediately, while all JSONB methods are stripped—statically preventing invalid SQL like `specs ->> 'a' -> 'b'`.
* **Boolean filter predicates**: Structural containment via `.json_contains(val)` (`@>`) and `.json_contained_by(val)` (`<@`); key existence via `.json_has_key(k)` (`?`), `.json_has_any_keys(ks)` (`?|`, rendering the RHS as `array[$1, $2]`), and `.json_has_all_keys(ks)` (`?&`); plus SQL/JSON Path predicates `.json_path_exists(path)` (`@?`) and `.json_path_predicate(path)` (`@@`).

```rust
let products = client
    .select::<Product>()
    .filter(
        Product::specs.json_delete("legacy_flag").json_has_key("warranty")
            & Product::specs.json_path("hardware").json_delete("serial_no").json_has_any_keys(["cpu", "gpu"])
            & Product::specs.json_path_array(["dimensions", "box"]).json_delete_path(["temp", "draft"]).json_has_all_keys(["width", "height"])
            & Product::specs.json_path_exists("$.stock ? (@ > 0)")
            & Product::specs.json_path_predicate("$.certified == true")
            & Product::specs.json_path_text("sku").matches("^PRO-[0-9]+")
    )
    .load()
    .await?;
```

### 8.3 1D Arrays, `Any`/`All` Quantifiers & POSIX Regular Expressions

Array columns (`Column<Vec<T>>`) and `ArrayAgg(expr)` nodes (whose output type is automatically lifted to `Static<Vec<T>>` via `IntoArrayOutput`) satisfy `CanPgArray<X>`, unlocking `.array_contains(rhs)` (`@>`), `.array_contained_by(rhs)` (`<@`), and `.array_overlap(rhs)` (`&&`). Pass borrowed slices like `&[&str]` or `[&str; N]` directly to `Column<Vec<String>>` without allocating a `Vec<String>` on the heap.

For quantified comparisons, `Any(...)` and `All(...)` accept both slices/arrays—rendering `Post::id.eq(Any([1_i32, 2, 3]))` as `"post"."id" = any(array[$1, $2, $3])`—and `Select::from` subqueries, rendering `Post::id.ne(All(Select::from::<Order>()...))` as `"post"."id" != all(select ...)`. Textual nodes also gain case-insensitive `.ilike(pat)` / `.not_ilike(pat)` and the four POSIX regex operators: `.matches(pat)` (`~`), `.not_matches(pat)` (`!~`), `.imatches(pat)` (`~*`), and `.not_imatches(pat)` (`!~*`).

```rust
use kova::dsl::func::{All, Any, ArrayAgg};
use kova::query::Select;

let posts = client
    .select::<Post>()
    .columns((Post::id, ArrayAgg(Post::title)))
    .filter(
        Post::id.eq(Any([1_i32, 2, 3]))
            & Post::id.ne(All(
                Select::from::<Order>()
                    .columns(Order::user_id)
                    .filter(Order::is_active.eq(true)),
            ))
            & Post::tags.array_contains(&["rust", "sql"][..])
            & Post::title.ilike("kova%")
            & Post::title.imatches("^orm")
    )
    .group_by(Post::id)
    .having(ArrayAgg(Post::title).array_overlap(&["featured"][..]))
    .load_in::<(i32, Vec<String>)>()
    .await?;
```

### 8.4 Set-Returning Functions (SRF: `Unnest`, `GenerateSeries`, `JsonbEach`)

PostgreSQL allows calling Set-Returning Functions inside the `FROM` clause. Kova provides five strongly-typed SRF nodes:

* `Unnest(array)` expands any `CanPgArray<X>` column (`Post::tags`) or Rust slice (`&[1, 2, 3]`) into the virtual column `Unnest::unnest`.
* `GenerateSeries(start, stop)` (with optional `.step(step)`) exposes `GenerateSeries::generate_series`.
* `JsonbArrayElements(jsonb)`, `JsonbEach(jsonb)`, and `JsonbEachText(jsonb)` expand `CanPgJsonb<X>` nodes or subqueries into `value` or `(key, value)` virtual columns.

SRF nodes implement `Deref` to their static schema structs, so you can reference virtual columns either via associated constants (`JsonbEachText::key`) or via a local variable (`each_fn.key`). When constructed over `Copy` types like `Column`, the SRF node itself is `Copy` and can be passed by reference `&srf` into `.srf(...)`.

> **Under-the-hood SRF alias rendering**
> In PostgreSQL, aliasing a single-column scalar SRF like `unnest` or `generate_series` requires an explicit column definition list—`AS "generate_series_0"("generate_series")`—otherwise outer references to `"generate_series_0"."generate_series"` fail to resolve. Conversely, SRFs like `jsonb_each_text` that natively return composite `OUT` records (`OUT key text, OUT value text`) must be aliased solely as `AS "jsonb_each_text_0"`. Kova's `RenderContext::push_srf_alias` distinguishes both protocols automatically whenever you call `.tag::<TagSrf>()` on an SRF node.

```rust
use kova::dsl::func::{GenerateSeries, JsonbArrayElements, JsonbEach, JsonbEachText, Unnest};

struct TagSrf;

// 1. Attach Unnest and a tagged stepped GenerateSeries
// Renders: select "unnest"."unnest", "generate_series_0"."generate_series" from "post", unnest("post"."tags"), generate_series($1, $2, $3) as "generate_series_0"("generate_series")
let srf_rows = client
    .select::<Post>()
    .srf((
        Unnest(Post::tags),
        GenerateSeries(1_i32, 10_i32).step(2_i32).tag::<TagSrf>(),
    ))
    .columns((
        Unnest::unnest,
        GenerateSeries::generate_series.tag::<TagSrf>(),
    ))
    .load_in::<(String, i32)>()
    .await?;

// 2. Expand JSONB arrays and key-value pairs
let spec_kv = client
    .select::<Product>()
    .srf((
        JsonbArrayElements(Product::specs.json_path("variants")),
        JsonbEach(Product::specs),
        JsonbEachText(Product::specs).tag::<TagSrf>(),
    ))
    .columns((
        JsonbArrayElements::value,
        JsonbEach::key,
        JsonbEachText::value.tag::<TagSrf>(),
    ))
    .load_in::<(String, String, String)>()
    .await?;
```

---

## 9. SQLite Native Features & Full-Text Search

Under the `sqlite` feature, Kova provides static type support for SQLite 3.45+ binary `JSONB` payloads, string aggregation, recursive JSON tree expansion, and the FTS5 full-text search engine.

### 9.1 `GroupConcat` & JSON / Binary JSONB

`GroupConcat(expr)` provides SQLite's string aggregation, supporting `.separator(" | ")` (with automatic single-quote escaping) alongside `.distinct()`, `.order_by(...)`, `.filter(...)`, and `.over(...)`.

For JSON processing, Kova separates standard text JSON from SQLite 3.45+ binary JSONB at the type level:

* **Text JSON (`JsonObject`, `JsonGroupArray`)** outputs `Static<String>`, retaining `CanString<X>` so text methods like `.like()` work directly.
* **Binary JSONB (`JsonbObject`, `JsonbGroupArray`)** outputs `Static<Vec<u8>>`. These nodes satisfy `CanSqliteJson<X>` for fast binary manipulation while statically blocking text operators like `.like()`. Wrap them in `Json(expr)` at the projection edge to convert binary JSONB back to `Static<String>`.
* **Dynamic path extraction (`.json_extract` / `.jsonb_extract`)** is available on any `CanSqliteJson<X>` node (`String`, `Vec<u8>`, `HashMap`, or custom JSON structs; non-JSON columns like `i32` or `Uuid` are rejected). Because an extracted JSON value can be a string, number, or boolean, its output type is `Dynamic`—allowing further `.json_extract(...)` chains, arithmetic via `.add(10)`, regex matching via `.regexp(...)`, or comparisons against primitive types:

```rust
use kova::dsl::func::{GroupConcat, Json, JsonGroupArray, JsonObject, JsonbGroupArray, JsonbObject};

let rows = client
    .select::<User>()
    .columns((
        GroupConcat(User::name).separator(" | ").order_by(User::name.asc()),
        JsonObject((User::id, User::name)),
        Json(JsonbObject((User::id, User::name))),
        JsonGroupArray(User::id).distinct().filter(User::is_active.eq(true)),
    ))
    .filter(
        User::bio.json_extract("$.profile").json_extract("$.level").add(10).ge(100)
            & User::bio.jsonb_extract("$.role").eq("admin")
    )
    .load_in::<(Option<String>, String, String, String)>()?;
```

### 9.2 Set-Returning Functions (SRF: `JsonEach` & `JsonTree`)

SQLite's `JsonEach` (shallow expansion) and `JsonTree` (recursive tree walk) accept three argument forms via `IntoSrfFunction`:

1. **Single node**: `JsonEach(Product::specs)` expands the entire column;
2. **`(Node, Path)` tuple**: `JsonTree((Product::specs, "$.hardware"))` expands recursively starting from a specific JSON path, where the path argument is constrained by `Compatible<Static<String>, X>` to reject numbers or booleans at compile time;
3. **Inner subquery**: `JsonEach(Select::from::<Product>().columns(Product::specs)...)` expands the scalar subquery result into rows.

Both expose SQLite's standard 8 virtual columns: `key` (`String`), `value` (`Dynamic`), `type_` (mapping to `"type"`, `String`), `atom` (`Dynamic`), `id` (`i64`), `parent` (`i64`), `fullkey` (`String`), and `path` (`String`):

```rust
use kova::dsl::func::{JsonEach, JsonTree};

struct TreeAlias;

let tree_nodes = client
    .select::<Product>()
    .srf((
        JsonEach(Product::specs),
        JsonTree((Product::specs, "$.hardware")).tag::<TreeAlias>(),
    ))
    .columns((
        JsonEach::key,
        JsonEach::type_,
        JsonTree::fullkey.tag::<TreeAlias>(),
        JsonTree::value.tag::<TreeAlias>(),
    ))
    .filter(
        JsonEach::atom.is_not_null()
            & JsonTree::parent.tag::<TreeAlias>().gt(0_i64)
    )
    .load_in::<(String, String, String, String)>()?;
```

### 9.3 FTS5 Full-Text Search (`matches`, `bm25`, `highlight`, `snippet`)

After creating an FTS5 virtual table in SQLite via `CREATE VIRTUAL TABLE "post" USING fts5("title", "content")`, derive `#[derive(Model)]` on your struct as usual.

The `CanSqliteFts<X>` trait is implemented automatically for physical `Column`s satisfying `CanString<X>`, extracting the virtual table name `fts_table()` from column metadata. Numeric columns, UUID columns, and composite expressions like `Lower(Post::title)` do not implement `CanSqliteFts` and cannot call FTS5 methods.

On any textual FTS5 column, `.matches(pattern)` renders `"post"."content" match ?`, while `.bm25()` emits `bm25("post")` with output `Static<f64>`—implementing `CanMath<X>` so arithmetic like `.mul(-1.0)` works out of the box, alongside `.weights([10.0, 2.5])` for custom column scoring. For result formatting, `.highlight(col_idx, left, right)` wraps matched terms in tags and `.snippet(col_idx, left, right, ellipsis, max_tokens)` extracts contextual text fragments (both accepting static `&str` or dynamic `String` variables and outputting `Static<String>`):

```rust
let hits = client
    .select::<Post>()
    .columns((
        Post::title,
        Post::content.highlight(1, "<b>", "</b>"),
        Post::content.snippet(1, "[", "]", "...", 10),
    ))
    .filter(Post::content.matches("Rust AND SQL"))
    .order_by(Post::content.bm25().weights([10.0, 2.5]).asc())
    .load_in::<(String, String, String)>()?;
```

---

## 10. Transactions & the Raw SQL Escape Hatch

### 10.1 Closure-Driven Transactions & Compile-Time Escape Prevention

Calling `begin()` and `commit()` manually is a recipe for leaked connections and dangling row locks whenever an early `return` or `?` fires mid-function. Kova enforces closure-scoped transaction lifetimes:

* PostgreSQL: `client.transaction(async |tx| { ... }).await`
* SQLite: `client.transaction(|tx| { ... })`

Returning `Ok(val)` commits automatically; returning `Err(e)` rolls back. The `tx` handle passed into the closure exposes the exact same `select`, `insert`, `update`, `delete`, and `statement` methods as `client`, and automatically inherits the outer client's codec context `X`.

Look at the actual signature of `client.transaction` (shown for the synchronous client):

```rust
pub type Transaction<'b, 't, DB, X> =
    Operations<DB, &'b mut <<DB as Database>::Connection as Connection>::Transaction<'t>, X>;

pub fn transaction<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(Transaction<'_, '_, DB, X>) -> Result<R>
```

`FnOnce(Transaction<'_, '_, DB, X>)` desugars to a Higher-Rank Trait Bound (`for<'b, 't> FnOnce(Transaction<'b, 't, DB, X>)`). Because the return type `R` is declared on the outer function, it cannot reference the universally quantified borrow lifetime `'b` or transaction lifetime `'t`. If anyone tries to smuggle the transaction handle out via `Ok(tx)` or return an unexecuted query builder borrowing `tx` via `Ok(tx.select::<User>())`, Rust's borrow checker stops it cold at compile time.

### 10.2 PostgreSQL Pessimistic Row-Level Locks (`FOR UPDATE` / `SKIP LOCKED`)

For high-contention transactions—such as balance updates or multi-worker job queues backed by database tables—PostgreSQL's `Select` builder provides two additive row-locking methods:

* `.lock::<M>(mode, wait_policy)`: Locks only the specified physical table `M`, rendering `for ... of "table"`. The compiler requires `M: Model`, preventing you from locking a read-only view or CTE;
* `.lock_all(mode, wait_policy)`: Locks all tables referenced in the query.

```rust
use kova::dsl::{LockMode, WaitPolicy};

let rows = client
    .select::<User>()
    .inner_join::<Order>(Order::user_id.eq(User::id))
    .filter(User::id.eq(1))
    .lock::<User>(LockMode::Update, Some(WaitPolicy::NoWait))
    .lock::<Order>(LockMode::NoKeyUpdate, Some(WaitPolicy::SkipLocked))
    .load()
    .await?;
// Renders: select * from "user" inner join "order" on "order"."user_id" = "user"."id" where "user"."id" = $1 for update of "user" nowait for no key update of "order" skip locked
```

### 10.3 The Raw SQL Escape Hatch (`statement`)

When executing DDL (`CREATE TABLE` / `CREATE INDEX`), administrative commands (`VACUUM` / `PRAGMA`), or custom native queries, call `client.statement(sql)` (or `tx.statement(sql)`).

`Statement` plugs directly into Kova's `IntoArg<Bind<'a, DB, X>, M>` parameter binding pipeline and row decoder: bind a single value or a heterogeneous tuple of up to 32 elements via `.bind(...)`—with full support for `Codec <X>` custom types and zero-copy slices—and finish with `.execute()`, `.load_as::<M>()`, or `.load_in::<U>()`:

```rust
// 1. Execute DDL
client
    .statement(r#"CREATE UNIQUE INDEX IF NOT EXISTS "idx_user_name" ON "user" (lower("name"))"#)
    .execute()
    .await?;

// 2. Bind a heterogeneous tuple and extract results positionally (checked against Encode and DecodeColumn at compile time)
let raw_bytes: [u8; 4] = [0xCA, 0xFE, 0xBA, 0xBE];
let rows = client
    .statement("SELECT $1::int2, $2::int4, $3::bytea")
    .bind((12_i8, 1000_u16, &raw_bytes))
    .load_in::<(i16, i32, [u8; 4])>()
    .await?;
```

---

## 11. Workspace Architecture & Design Trade-offs

As a project grows into a multi-crate Cargo workspace, the most fundamental architectural requirement is **extracting domain models into a standalone `models` crate** shared across API gateways, domain services, and background workers.

Almost every mainstream Rust ORM falls flat on its face here.

### 11.1 Why Traditional Rust ORMs Cannot Isolate `models` into a Standalone Crate

The culprit is Rust's **Orphan Rules**: to implement `Trait` for `T`, at least one of `Trait` or `T` must be defined in the current crate.

In a real system, a `Product` table doesn't just hold `i32` and `String`. It holds third-party types (`uuid::Uuid`, `url::Url`, `chrono::DateTime`, `jiff::Timestamp`), standard library collections (`HashMap<String, String>`), and custom domain types (`Decimal`, `ProductSpec`, `TagList`). To persist the struct, every field type must implement the ORM's `Encode` and `Decode` traits.

In traditional ORMs, `Encode` and `Decode` are rigid, context-free traits:

* You **cannot** implement the ORM's `Encode` for `uuid::Uuid` or `HashMap` in your service crate, because both the trait and the type are foreign—triggering `E0117` immediately;
* To work around the orphan rules, traditional ORMs either hardcode dozens of Cargo feature flags inside their core crate to bless a fixed list of third-party libraries (if you use an unblessed crate, or want `DateTime<Utc>` stored as a millisecond `BIGINT` instead of the ORM's hardcoded representation, you hit a brick wall); or they force you to wrap every field in an ugly Newtype (`pub struct DbUuid(pub Uuid)`) inside your `models` crate, dragging heavy database driver dependencies and serialization logic into what should have been a pure domain model crate.

Both workarounds obliterate the architectural boundary between domain models and infrastructure.

### 11.2 Kova's Solution: Parameterized Codec Contexts (`Codec <X>`)

Kova introduces a **context type parameter `X`** (defaulting to `()`) across all codec and domain capability traits, bypassing the Orphan Rules algebraically.

#### Step 1: Write a `models` Crate with Zero Database Driver Dependencies

In your `models` crate, define structs using native types and annotate them with `#[derive(Model)]`:

```rust
// crate: models (src/product.rs)
// Compiles standalone with zero database driver dependencies and zero Encode/Decode implementations.
use chrono::{DateTime, Utc};
use jiff::Timestamp;
use kova::Model;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;
use uuid::Uuid;

#[derive(Model)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub price: Decimal,
    pub vendor_url: Url,
    pub specs: ProductSpec,
    pub attributes: HashMap<String, String>,
    pub tags: TagList,
    pub created_at: DateTime<Utc>,
    pub updated_at: Timestamp,
}

pub struct Decimal(pub i64);

#[derive(Serialize, Deserialize)]
pub struct ProductSpec {
    pub warranty: String,
    pub weight_grams: i32,
}

pub struct TagList(pub Vec<String>);
```

This compiles cleanly on its own because `kova-derive` emits a **generic contract open over any context `X`**, automatically deduplicating repeated field types in the `where` clause:

```rust
impl<X> ::kova::types::Model<::kova::types::Engine, X> for Product
    where
        Self: ::kova::types::FromRow<::kova::types::Engine, X>,
        for<'a> &'a Uuid: ::kova::codec::Encode<'a, X>,
        for<'a> &'a String: ::kova::codec::Encode<'a, X>,
        // ... deduplicated bounds for remaining non-#[kova(auto)] field types
```

As soon as a downstream context `X` supplies `Encode<'a, X>` and `Decode<X>` for those field types, `Product` automatically becomes a valid `Model` under `X`. Physical serialization bindings are deferred entirely to downstream service crates.

#### Step 2: Define `AppCodec` & Implement Zero-Copy Codecs in Your Service Crate

In your downstream service crate, define a local zero-sized struct `pub struct AppCodec;`.

Because **`AppCodec` is a local type to the service crate**, Rust's uncovered type parameter rule allows you to implement `Encode<'a, AppCodec>` and `Decode<AppCodec>` for `Uuid`, `Url`, `DateTime`, `HashMap`, and any type from `models`:

```rust
// crate: app-service (src/codec.rs)
use chrono::{DateTime, Utc};
use jiff::Timestamp;
use kova::codec::*;
use kova::{Error, Result};
use models::product::{Decimal, ProductSpec, TagList};
use std::borrow::Cow;
use std::collections::HashMap;
use url::Url;
use uuid::Uuid;

pub struct AppCodec;

impl<'a> Encode<'a, AppCodec> for Uuid {
    fn encode(self) -> Result<Value<'a>> {
        Ok(Value::Blob(Cow::Owned(self.into_bytes().to_vec())))
    }
}
impl<'a, 'b: 'a> Encode<'a, AppCodec> for &'b Uuid {
    fn encode(self) -> Result<Value<'a>> {
        // Zero-copy encoding: borrows the 16-byte array slice directly from Uuid into Value::Blob with zero heap allocation
        Ok(Value::Blob(Cow::Borrowed(self.as_bytes())))
    }
}
impl Decode<AppCodec> for Uuid {
    fn decode(value: Value<'_>) -> Result<Self> {
        match value {
            // Zero-copy decoding: under engines like SQLite, Value::Blob(b) and Value::Text(t) borrow directly
            // from the underlying C buffer. Uuid::from_slice(&b) and serde_json::from_str(&t) allocate zero intermediate Strings or Vecs.
            Value::Blob(b) => Uuid::from_slice(&b).map_err(|e| Error::ParseError(e.to_string())),
            Value::Text(t) => Uuid::parse_str(&t).map_err(|e| Error::ParseError(e.to_string())),
            _ => Err(Error::TypeMismatch("Expected Blob or Text for Uuid".into())),
        }
    }
}

// ... Implement Url, DateTime<Utc>, Timestamp, HashMap, Decimal, ProductSpec, TagList the same way ...

// Finally, register positional column extraction for all custom types in a single macro call (enables .load_in::<Uuid>(), etc.):
decode_column!(
    AppCodec,
    Uuid,
    Url,
    DateTime<Utc>,
    Timestamp,
    HashMap<String, String>,
    Decimal,
    ProductSpec,
    TagList,
);
```

The exact same `models` crate can be bound to completely different physical representations across different services or database engines—storing `DateTime<Utc>` as a millisecond `Value::Int8` vs. RFC 3339 text, or encoding `TagList` using PostgreSQL's native `TEXT[]` wire protocol while serializing it as a JSON array string under SQLite.

#### Step 3: Unlock SQL Operators & Cross-Type Compatibility for Custom Types

In most ORMs, even if you manage to serialize a custom type, it is crippled in queries: you cannot call `.add()` or `Sum()` on a custom `Decimal` column, nor pass a `&str` literal directly into `Column<Url>`.

In Kova, every domain capability trait (`CanMath<X>`, `CanString<X>`, `CanPgJsonb<X>`, `CanPgArray<X>`, `CanSqliteJson<X>`) and the type compatibility matrix (`Compatible<Target, X>`) are parameterized by `X`. Add a few trait impls in `codec.rs` and your custom types gain first-class SQL operators under `AppCodec`—automatically penetrating into `Option<CustomType>` columns as well:

```rust
// 1. Grant arithmetic capability to Decimal, and allow mixing i64 with Decimal (and Option<Decimal>)
impl CanMath<AppCodec> for Decimal {}
impl Compatible<Static<Decimal>, AppCodec> for Static<i64> {}
impl Compatible<Static<Option<Decimal>>, AppCodec> for Static<i64> {}

// 2. Grant textual capability to Url (automatically unlocking PG ilike/matches and SQLite regexp/FTS5),
//    and allow passing &str or String directly to Column<Url> without manual Url::parse() calls
impl CanString<AppCodec> for Url {}
impl<'a> Compatible<Static<Url>, AppCodec> for Static<&'a str> {}
impl Compatible<Static<Url>, AppCodec> for Static<String> {}

// 3. Grant JSONB navigation to ProductSpec, and PG array operators to TagList
impl CanPgJsonb<AppCodec> for ProductSpec {}
impl CanPgArray<AppCodec> for TagList {}
impl<'a> Compatible<Static<TagList>, AppCodec> for Static<&'a [&'a str]> {}
impl<'a> Compatible<Static<Option<TagList>>, AppCodec> for Static<&'a [&'a str]> {}
```

Call `.codec::<AppCodec>()` once after building your client (zero runtime cost; it only swaps a `PhantomData` marker), and your entire custom type system is live:

```rust
let client = Options::from(conn_str).build()?.codec::<AppCodec>();

client
    .update::<Product>()
    .set((
        Product::price.increment(500_i64),            // Increment Decimal column by i64
        Product::vendor_url.set("https://apple.com"), // Assign &str directly to Url column
        Product::specs.set(&new_specs),               // Write ProductSpec as JSONB
        Product::tags.set(&new_tags),                 // Write TagList as TEXT[] array
    ))
    .filter(
        Product::id.eq(&target_uuid)                  // Zero-copy slice bind for &Uuid
            & Product::vendor_url.ilike("https://%")  // Call ILIKE directly on Url column
            & Product::tags.array_contains(&["featured"][..])
            & Product::specs.json_path_text("warranty").eq("2_years")
    )
    .execute()
    .await?;
```

If different modules in the same binary need different decoding strategies (e.g., `AppCodec` for transactional writes and `ViewCodec` for reporting views), call `client.clone().codec::<ViewCodec>()` (`Client` implements `Clone` manually without requiring `AppCodec: Clone`). Codecs are strictly isolated at compile time: querying `Product` with an unconfigured default `Client`, or calling `.add()` on `Decimal` under `OtherCodec`, is rejected immediately by the compiler.

---

### 11.3 Design Trade-offs

Kova's internal architecture rests on five non-negotiable engineering rules:

#### 1. Boundary Validation & In-Tree Type Erasure (`EraseX`): Eliminating Monomorphization Bloat

Heavily generic Rust libraries stuff the full type of every expression node into the query builder's type parameters. They call it "zero-cost," while triggering massive **monomorphization bloat** in the compiler backend—every permutation of filter conditions or `Codec` markers forces LLVM to duplicate and compile a brand-new copy of the entire AST rendering pipeline.
In Kova, **100% of static type and capability checking happens at the method call boundary; the instant an expression crosses into the AST, `EraseX` strips the phantom type markers**, collapsing nodes into homogeneous algebraic structures `Condition<'a, DB, ()>` and `Operand<'a, DB>` (any runtime error during parameter encoding is lazily sealed inside `OperandNode::Error` and surfaced when the query is built).
Builder types stay flat so they can be stored in structs or returned from functions effortlessly, and the core SQL rendering engine (`kova_kernel::render`) is compiled into machine code exactly once per binary.

#### 2. Tuple Polymorphism (`.columns((a, b))`) & the `SingleArg` Guard

Until Rust stabilizes variadic generics, accepting a variable number of heterogeneous arguments under a single method name leaves only three options:

1. Fall back to declarative macros (`.columns!(a, b)`)—breaking method chaining and killing IDE autocomplete;
2. Split into single-argument chains (`.column(a).column(b)`)—shattering the visual cohesion of SQL projection lists;
3. **Heterogeneous tuple polymorphism**. Kova's `IntoArg<A, M>` unifies a single item `T` and heterogeneous tuples of 1 to 32 elements `(T1, ..., T32)` behind a single method signature—aligned to the same 32-element tuple limit across `FromRow`, `DecodeColumn`, `USING` join columns, and `UPDATE ... FROM` table lists. Meanwhile, the `SingleArg` guard statically forbids nonsensical nested tuples like `((a, b), c)` and empty unit `()`, emitting clear compiler diagnostics.

#### 3. Why Custom Scalar Extraction Uses One Line of `decode_column!`

`.load_in::<U>()` and `.execute_in::<U>()` rely on the positional extraction trait `DecodeColumn<DB, X>`. It must support bare scalars like `Uuid`, generic containers like `Option<T>`, `Box<T>`, and `[u8; N]`, and heterogeneous tuples `(T1, T2, ...)` up to 32 elements. Due to Rust's trait coherence rules, once `DecodeColumn` is implemented for `Option<T>` and tuples, the compiler forbids a blanket implementation covering all bare types `T` (`impl<DB, T, X> DecodeColumn<DB, X> for T where DB: Decode<T, X>`).
Bypassing coherence with a dummy phantom parameter `M` on `DecodeColumn` would pollute every call site in user code whenever type inference ambiguities force you to write `.load_in::<Uuid, _>()`.
**Kova will never pollute ten thousand call sites just to save one line of registration where a type is defined.** Invoking `decode_column!(AppCodec, Uuid, ...)` once alongside your codec impls—expanding to 8 lines of standard trait code with zero black magic—keeps every `.load_in::<Uuid>()`, `.load_in::<Option<Uuid>>()`, and `.load_in::<(Uuid, Url, i64)>()` call across your entire codebase crystal clean.

#### 4. Why Struct and Field Rename Attributes Are Forbidden

Kova's `#[derive(Model)]` supports a single attribute—`#[kova(auto)]`—and explicitly rejects `#[kova(table_name = "...")]` or field-renaming attributes in the macro parser.
Allowing Rust identifiers and physical database schemas to drift apart is a breeding ground for technical debt. When `AccountProfile` in Rust maps to `tbl_usr_info_v2` in the database and `created_time` maps to `c_ts`, every engineer reading code or debugging a slow production query is forced to perform mental symbol translation. If a legacy database has a mangled schema, fix the schema or put a clean SQL `VIEW` in front of it. Do not paper over it by plastering macro rename attributes across your Rust codebase. Maintain a single source of truth: the struct name (in `snake_case`) is the table name; the field name is the column name. Call things what they are, and stop lying to your own codebase.

#### 5. Why `postgres` and `sqlite` Features Are Mutually Exclusive

Enabling both `postgres` and `sqlite` on the top-level `kova` crate triggers a `compile_error!`.
To cater to the vanishingly rare edge case of mixing multiple database drivers inside the same binary, many frameworks either box connections, rows, and values behind `dyn Trait` dynamic dispatch at runtime, or infect every model and query function signature in user code with a `<DB: Database>` generic parameter.
While Kova's internal engine (`kova-kernel`) is a fully parameterized polymorphic algebra, the user-facing `kova` facade locks the active engine at compile time via `types::Engine`. This allows `#[derive(Model)]` and `decode_column!` to emit static, zero-vtable code tailored to the exact physical engine, and frees your `Client`, `Transaction`, and repository structs from carrying `<DB>` generic noise. We refuse to tax 99.9% of real-world engineering projects with syntax noise and runtime overhead for an edge case almost nobody runs in production.

---

## Where Kova Stands

Kova is not built for everyone.

If you want to touch a database without learning SQL, if you want mutating a struct field in memory to silently trigger implicit `UPDATE` queries behind your back, or if you want an all-in-one kitchen sink with built-in migration runners and runtime cross-database dialect translation, the ecosystem is already full of traditional ORMs that will happily oblige.

Kova is built for engineers who **respect relational algebra, care about memory layout and compile times, and demand absolute control over every single SQL statement hitting production**. It will never hide the database from you. It is the sharpest, most honest scalpel in your hand.