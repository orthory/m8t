use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct CreateDocumentRequest {}

#[derive(Serialize, Deserialize)]
pub(crate) struct CreateDocumentResponse {}

pub(crate) async fn create(
    Json(payload): Json<CreateDocumentRequest>,
) -> (StatusCode, Json<CreateDocumentResponse>) {
    todo!()
}