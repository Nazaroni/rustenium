use std::future::Future;
use std::time::{Duration, Instant};
use tokio::time::sleep;

pub mod wait {
    use super::*;

    pub async fn wait_for<F, Fut>(timeout: Duration, mut condition: F) -> Result<(), ()>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = bool>,
    {
        let start = Instant::now();
        while start.elapsed() < timeout {
            if condition().await {
                return Ok(());
            }
            sleep(Duration::from_millis(100)).await;
        }
        Err(())
    }
}
