#[cfg(feature = "std-sync")]
pub mod std {
    use std::sync::{Arc, RwLock};

    pub struct ContextGuard<T> {
        ctx: Arc<RwLock<T>>,
    }
}

#[cfg(feature = "tokio-sync")]
pub mod tokio {
    use std::sync::Arc;
    use tokio::sync::RwLock;

    pub struct ContextGuard<T> {
        ctx: Arc<RwLock<T>>,
    }
}
