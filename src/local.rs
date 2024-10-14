use futures_lite::future::yield_now;
use std::future::Future;

/// Local executor.
pub struct LocalExecutor<'a>(edge_executor::LocalExecutor<'a>);

impl<'a> LocalExecutor<'a> {
    /// Creates a new executor.
    pub fn new() -> Self {
        Self(edge_executor::LocalExecutor::new())
    }

    /// Spawns and detach a task.
    pub fn spawn<T: Send + 'a>(&self, future: impl Future<Output = T> + 'a) {
        self.0.spawn(future).detach()
    }

    /// Runs the executor forever.
    pub async fn run(&self) {
        loop {
            for _ in 0..200 {
                // Wait until the tick completes.
                self.0.tick().await;
            }
            // Yield every now and then.
            yield_now().await;
        }
    }
}
