#[cfg(feature = "std-sync")]
pub mod std {
    use std::sync::{Arc, RwLock, RwLockReadGuard};

    #[derive(Clone)]
    pub struct ContextGuard<T> {
        ctx: Arc<RwLock<T>>,
    }

    impl<T> ContextGuard<T> {
        pub fn new(ctx: T) -> Self {
            ContextGuard {
                ctx: Arc::new(RwLock::new(ctx)),
            }
        }

        pub fn ctx(&self) -> RwLockReadGuard<'_, T> {
            self.ctx.read().expect("failed to read ctx")
        }
    }
}

#[cfg(feature = "tokio-sync")]
pub mod tokio {
    use std::sync::Arc;
    use tokio::sync::{RwLock, RwLockReadGuard};

    #[derive(Clone)]
    pub struct ContextGuard<T> {
        ctx: Arc<RwLock<T>>,
    }

    impl<T> ContextGuard<T> {
        pub fn new(ctx: T) -> Self {
            ContextGuard {
                ctx: Arc::new(RwLock::new(ctx)),
            }
        }

        pub async fn ctx(&self) -> RwLockReadGuard<'_, T> {
            self.ctx.read().await
        }
    }
}
