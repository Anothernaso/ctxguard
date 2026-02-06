/// Ctxguard for rust std.
/// Uses the `RwLock` from the `std::sync` module.
#[cfg(feature = "std-sync")]
pub mod std {
    use std::sync::{Arc, RwLock, RwLockReadGuard};

    /// A tool for gatekeeping access to
    /// a shared context of type `T` with interior mutability.
    pub struct ContextGuard<T> {
        ctx: Arc<RwLock<T>>,
    }

    impl<T> ContextGuard<T> {
        pub fn new(ctx: T) -> Self {
            ContextGuard {
                ctx: Arc::new(RwLock::new(ctx)),
            }
        }
        /// Gets a reference to the shared context.
        ///
        /// # Panics
        ///
        /// Panics if `RwLock::<T>::read` returns `Err`.
        pub fn ctx(&self) -> RwLockReadGuard<'_, T> {
            self.ctx.read().expect("failed to read ctx")
        }
    }

    impl<T> Clone for ContextGuard<T> {
        /// Clones the pointer to the context
        /// while remaining a shared reference to the context.
        fn clone(&self) -> Self {
            ContextGuard {
                ctx: self.ctx.clone(),
            }
        }
    }
}

/// Ctxguard for the `tokio` async runtime.
/// Uses the `RwLock` from the `tokio::sync` module.
#[cfg(feature = "tokio-sync")]
pub mod tokio {
    use std::sync::Arc;
    use tokio::sync::{RwLock, RwLockReadGuard};

    /// A tool for gatekeeping access to
    /// a shared context of type `T` with interior mutability.
    pub struct ContextGuard<T> {
        ctx: Arc<RwLock<T>>,
    }

    impl<T> ContextGuard<T> {
        pub fn new(ctx: T) -> Self {
            ContextGuard {
                ctx: Arc::new(RwLock::new(ctx)),
            }
        }

        /// Gets a reference to the shared context.
        pub async fn ctx(&self) -> RwLockReadGuard<'_, T> {
            self.ctx.read().await
        }
    }

    impl<T> Clone for ContextGuard<T> {
        /// Clones the pointer to the context
        /// while remaining a shared reference to the context.
        fn clone(&self) -> Self {
            ContextGuard {
                ctx: self.ctx.clone(),
            }
        }
    }
}
