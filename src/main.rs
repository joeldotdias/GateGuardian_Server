use std::sync::Arc;
use tokio::net::TcpListener;

use ggserver::{
    config::{ AppState, Config },
    database,
    router,
    shutdown
};

#[tokio::main]
async fn main() {
    let config = Config::from_env();

    let pool = database::db_connection(&config.database_url).await;

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let app = router::create_router(Arc::new(AppState::new(pool))).await;

    let listener = TcpListener::bind(&config.socket_addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown::cleanup())
        .await
        .unwrap();
}
