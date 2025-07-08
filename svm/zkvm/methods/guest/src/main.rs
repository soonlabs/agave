
mod transaction;
mod svm;
mod accounts_db;

use risc0_zkvm::guest::env;
use solana_sdk::{pubkey::Pubkey, account::ReadableAccount};

use crate::transaction::PayTransaction;
use crate::accounts_db::AccountsDb;
use crate::svm::MockSVM;

fn main() {
    // read the input
    let transfer_amount: u64 = env::read();

    let alice = Pubkey::new_unique();
    let alice_balance = 10_000_000_000; // 10 SOL
    let bob = Pubkey::new_unique();
    let bob_balance = 1_000_000_000; // 1 SOL

    let mut accounts_db = AccountsDb::new();
    accounts_db.add_accounts_for_transfer(
        (alice, alice_balance), // 10 SOL
        Some((bob, bob_balance)), // 1 SOL
    );

    let svm = MockSVM::new(accounts_db);

    // Execute the transaction
    let tx = PayTransaction {
        from: alice,
        to: bob,
        amount: transfer_amount,
    }.into_sanitized_tx();

    match svm.process_simple_tx(&tx) {
        Ok(record) => {
            env::log("Transaction executed successfully");
            for (key, account) in record.accounts {
                if key == alice {
                    assert_eq!(account.lamports(), alice_balance - transfer_amount);
                } else if key == bob {
                    assert_eq!(account.lamports(), bob_balance + transfer_amount);
                }
            }
        }
        Err(e) => {
            env::log(&format!("Transaction execution failed: {}", e));
        }
    }

    // write public output to the journal
    env::commit(&true);
}
