use std::sync::Arc;
use tokio::sync::RwLock;
use fs_backend::Backend;
use server::document_resolver;
use server::document_resolver::{DocumentResolver};

const LISTEN_ADDR: &str = "127.0.0.1:3421";

#[tokio::main]
async fn main() {
    let current_file = std::env::current_exe().unwrap();
    let project_root = current_file.parent().unwrap().parent().unwrap().parent().unwrap();
    let base_path = project_root.join("examples/sample").to_string_lossy().into_owned();
    
    let document_resolver = DocumentResolver::new(Backend::StdFS(base_path))
        .expect("failed to initialize document resolver");
    
    let app_state = server::state::AppState{
        document_resolver: Arc::new(RwLock::new(document_resolver)),
    };

    let router = axum::Router::new();
    let router = server::handlers::register(router);
    let router = router.with_state(app_state);

    let listener = tokio::net::TcpListener::bind(LISTEN_ADDR).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
