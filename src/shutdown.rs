use tokio::signal;

pub async fn cleanup() {
    let ctrl_c = async {
        signal::ctrl_c()
        .await
        .expect("Failed to load interrupt handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to load signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {}
    }
}
