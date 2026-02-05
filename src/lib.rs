#[cfg(feature = "std-sync")]
pub mod std {
    use std::sync::{Arc, RwLock, RwLockReadGuard};

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

    impl<T> Clone for ContextGuard<T> {
        fn clone(&self) -> Self {
            ContextGuard {
                ctx: self.ctx.clone(),
            }
        }
    }
}

#[cfg(feature = "tokio-sync")]
pub mod tokio {
    use std::sync::Arc;
    use tokio::sync::{RwLock, RwLockReadGuard};

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

    impl<T> Clone for ContextGuard<T> {
        fn clone(&self) -> Self {
            ContextGuard {
                ctx: self.ctx.clone(),
            }
        }
    }
}
