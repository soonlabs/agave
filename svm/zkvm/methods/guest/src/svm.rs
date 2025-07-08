use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use solana_compute_budget::compute_budget::ComputeBudget;
use solana_program_runtime::invoke_context::{EnvironmentConfig, InvokeContext};
use solana_program_runtime::log_collector::LogCollector;
use solana_sdk::feature_set::FeatureSet;
use solana_sdk::rent::Rent;
use solana_sdk::transaction::{SanitizedTransaction, TransactionError};
use solana_sdk::transaction_context::{ExecutionRecord, IndexOfAccount, TransactionAccount, TransactionContext};
use solana_svm::message_processor::MessageProcessor;
use crate::accounts_db::AccountsDb;

#[derive(Default)]
pub struct MockSVM {
    feature_set: Arc<FeatureSet>,
    accounts_db: AccountsDb,
    log_collector: Rc<RefCell<LogCollector>>,
}

impl MockSVM {
    pub fn new(accounts_db: AccountsDb) -> Self {
        Self {
            accounts_db,
            log_collector: LogCollector::new_ref(),
            feature_set: Arc::new(FeatureSet::default()),
        }
    }

    pub fn process_simple_tx(
        &self,
        tx: &SanitizedTransaction,
    ) -> Result<ExecutionRecord, TransactionError> {
        let compute_budget = ComputeBudget::default();
        let mut program_cache = self.accounts_db.programs_cache.clone();

        let accounts = tx
            .message()
            .account_keys()
            .iter()
            .map(|key| {
                Ok((
                    *key,
                    self.accounts_db.get_account(key).ok_or(TransactionError::AccountNotFound)?
                ))
            })
            .collect::<Result<Vec<_>, TransactionError>>()?;
        let mut context = self.create_transaction_context(&compute_budget, accounts);

        let program_indices = tx
            .message()
            .instructions()
            .iter()
            .map(|c| {
                vec![c.program_id_index as IndexOfAccount]
            })
            .collect::<Vec<_>>();
        let mut accumulated_consume_units = 0;
        MessageProcessor::process_message(
            tx.message(),
            &program_indices,
            &mut InvokeContext::new(
                &mut context,
                &mut program_cache,
                EnvironmentConfig::new(
                    Default::default(),
                    None,
                    None,
                    self.feature_set.clone(),
                    0,
                    &self.accounts_db.sysvar_cache,
                ),
                Some(self.log_collector.clone()),
                compute_budget,
            ),
            &mut accumulated_consume_units,
        )?;
        Ok(context.into())
    }

    fn create_transaction_context(
        &self,
        compute_budget: &ComputeBudget,
        accounts: Vec<TransactionAccount>,
    ) -> TransactionContext {
        TransactionContext::new(
            accounts,
            Rent::default(),
            compute_budget.max_instruction_stack_depth,
            compute_budget.max_instruction_trace_length,
        )
    }
}