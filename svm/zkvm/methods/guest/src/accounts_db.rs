use std::collections::HashMap;
use std::sync::Arc;
use solana_bpf_loader_program::syscalls::create_program_runtime_environment_v1;
use solana_program_runtime::loaded_programs::{ProgramCacheEntry, ProgramCacheForTxBatch, ProgramRuntimeEnvironments};
use solana_program_runtime::sysvar_cache::SysvarCache;
use solana_sdk::account::{AccountSharedData, WritableAccount};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::{native_loader, system_program};
use solana_sdk::clock::Epoch;
use solana_system_program::system_processor;

#[derive(Default)]
pub(crate) struct AccountsDb {
    inner: HashMap<Pubkey, AccountSharedData>,
    pub programs_cache: ProgramCacheForTxBatch,
    pub sysvar_cache: SysvarCache,
}

impl AccountsDb {
    pub fn new() -> Self {
        let mut env = ProgramRuntimeEnvironments::default();
        env.program_runtime_v1 = Arc::new(
            create_program_runtime_environment_v1(
                &Default::default(),
                &Default::default(),
                false,
                false,
            ).unwrap()
        );
        let mut programs_cache = ProgramCacheForTxBatch::new(0, env, None, 0);
        programs_cache.replenish(
            system_program::id(),
            Arc::new(ProgramCacheEntry::new_builtin(
                0,
                b"system_program".len(),
                system_processor::Entrypoint::vm,
            )),
        );
        let mut accounts_db = Self {
            inner: HashMap::new(),
            sysvar_cache: Default::default(),
            programs_cache,
        };
        accounts_db.add_account(
            system_program::id(),
            AccountSharedData::create(
                1,
                "system_program".as_bytes().into(),
                native_loader::id(),
                true,
                Epoch::MAX,
            ),
        );
        accounts_db
    }

    pub fn get_account(&self, pubkey: &Pubkey) -> Option<AccountSharedData> {
        self.inner.get(pubkey).cloned()
    }

    pub fn add_accounts_for_transfer(
        &mut self,
        from: (Pubkey, u64),
        to: Option<(Pubkey, u64)>,
    ) {
        self.add_account(
            from.0,
            AccountSharedData::create(
                from.1,
                Vec::new(),
                system_program::id(),
                false,
                Epoch::MAX,
            ),
        );
        if let Some(to) = to {
            self.add_account(
                to.0,
                AccountSharedData::create(
                    to.1,
                    Vec::new(),
                    system_program::id(),
                    false,
                    Epoch::MAX,
                ),
            );
        }
    }

    fn add_account(&mut self, pubkey: Pubkey, account: AccountSharedData) {
        self.inner.insert(pubkey, account);
    }
}