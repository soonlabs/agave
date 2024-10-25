use crate::scheduler_messages::TransactionId;

/// Simple reverse-sequential ID generator for `TransactionId`s.
/// These IDs uniquely identify transactions during the scheduling process.
pub struct TransactionIdGenerator {
    next_id: u64,
}

impl Default for TransactionIdGenerator {
    fn default() -> Self {
        Self { next_id: u64::MAX }
    }
}

impl TransactionIdGenerator {
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> TransactionId {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_sub(1);
        TransactionId::new(id)
    }
}
