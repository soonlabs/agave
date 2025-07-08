use solana_sdk::{
    transaction::{Transaction, SanitizedTransaction},
    pubkey::Pubkey,
    system_instruction,
};
use std::collections::HashSet;

pub struct PayTransaction {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
}

impl PayTransaction {
    pub fn into_sanitized_tx(self) -> SanitizedTransaction {
        let instruction = system_instruction::transfer(&self.from, &self.to, self.amount);
        let transaction = Transaction::new_with_payer(&[instruction], Some(&self.from));
        SanitizedTransaction::try_from_legacy_transaction(transaction, &HashSet::new()).unwrap()
    }
}