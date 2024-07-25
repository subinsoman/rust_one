use std::sync::atomic::{AtomicUsize, Ordering};

// Atomic counter for generating unique IDs
static COUNTER: AtomicUsize = AtomicUsize::new(1);

/// Generates a unique transaction ID.
pub fn generate_transaction_id() -> i32 {
    COUNTER.fetch_add(1, Ordering::SeqCst) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_transaction_id() {
        let id1 = generate_transaction_id();
        let id2 = generate_transaction_id();
        assert_ne!(id1, id2);
    }
}
