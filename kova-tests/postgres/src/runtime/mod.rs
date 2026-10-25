pub mod client_patterns;
pub mod codec;
mod codec_and_types;
mod crud_and_batch;
mod queries_and_srf;
mod tx_pool_tls;

use crate::render::render_client;
use kova::Client;

/// Provides a session-isolated PostgreSQL `Client` for runtime execution tests.
pub async fn runtime_client() -> Client {
    render_client().await
}