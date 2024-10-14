use edge_executor::{Executor, Task};
use futures_lite::{future::yield_now, prelude::*};
use std::future::Future;

/// An executor with task priorities.
///
/// Tasks with lower priorities only get polled when there are no tasks with
/// higher priorities.
pub struct PriorityExecutor<'a> {
    executors: [Executor<'a>; 3],
}

impl<'a> PriorityExecutor<'a> {
    /// Creates a new executor.
    pub const fn new() -> Self {
        Self {
            executors: [Executor::new(), Executor::new(), Executor::new()],
        }
    }

    /// Spawns a task with the given priority.
    pub fn spawn<T: Send + 'a>(
        &self,
        priority: Priority,
        future: impl Future<Output = T> + Send + 'a,
    ) -> Task<T> {
        self.executors[priority as usize].spawn(future)
    }

    /// Runs the executor forever.
    pub async fn run(&self) {
        loop {
            for _ in 0..200 {
                let t0 = self.executors[0].tick();
                let t1 = self.executors[1].tick();
                let t2 = self.executors[2].tick();

                // Wait until one of the ticks completes, trying them in order
                // from highest priority to lowest priority.
                t0.or(t1).or(t2).await;
            }

            // Yield every now and then.
            yield_now().await;
        }
    }
}

/// Priority
#[repr(usize)]
#[derive(Clone, Copy, Debug, Default)]
pub enum Priority {
    High = 0,
    #[default]
    Medium = 1,
    Low = 2,
}
