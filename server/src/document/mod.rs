mod get;
mod create;

use axum::{Router};
use serde::{Deserialize, Serialize};
use crate::state::AppState;

pub fn register<S: AppState>(router: Router, state: S) -> Router {
    router
        .route("/documents/*document_path", axum::routing::get(get::get::<S>))
        .route("/documents", axum::routing::post(create::create))
        .with_state(state)
}
