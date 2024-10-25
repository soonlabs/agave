use solana_prio_graph_scheduler::scheduler_messages::TransactionBatchId;

#[derive(Default)]
pub struct BatchIdGenerator {
    next_id: u64,
}

impl BatchIdGenerator {
    pub fn next(&mut self) -> TransactionBatchId {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_sub(1);
        TransactionBatchId::new(id)
    }
}
