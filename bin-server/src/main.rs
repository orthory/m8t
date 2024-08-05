use std::sync::Arc;
use clap::Parser;
use tokio::sync::RwLock;
use server::document_resolver::{Backend, DocumentResolver};

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Configuration {
    #[arg(short, long)]
    base_directory: String,

    #[arg(short, long)]
    listen_addr: String,
}

#[tokio::main]
async fn main() {
    let args = Configuration::parse();

    let app_state = server::state::AppState{
        document_resolver: Arc::new(RwLock::new(DocumentResolver::new(Backend::StdFS(args.base_directory.clone())))),
    };
    
    let router = axum::Router::new();
    let router = server::handlers::register(router);
    let router = router.with_state(app_state);

    let listener = tokio::net::TcpListener::bind(args.listen_addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
