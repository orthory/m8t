mod document;
mod testutil;
mod state;

use std::io::Cursor;
use axum::{Router};

async fn routes<S: crate::state::AppState>(
    r: Router,
    s: S,
) -> Router {
    document::register(r, s)
}
