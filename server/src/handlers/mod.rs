mod document;
mod hierarchy;

use crate::state::AppState;
use axum::{routing, Router};

pub fn register(router: Router<AppState>) -> Router<AppState> {
    router
        .route("/documents/:fragment_type/*document_path", routing::get(document::get_document))
        .route("/hierarchy/*segments", routing::get(hierarchy::get_hierarchy))
}
