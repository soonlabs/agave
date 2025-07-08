use std::sync::atomic::{AtomicU64, Ordering};
use risc0_zkvm::guest::env;
use solana_program_runtime::loaded_programs::{ProgramCacheEntry, ProgramCacheEntryOwner, ProgramCacheEntryType};
use solana_program_runtime::solana_rbpf::program::BuiltinProgram;
use solana_program_runtime::with_mock_invoke_context;
use solana_sdk::account::create_account_shared_data_for_test;
use solana_sdk::epoch_schedule::EpochSchedule;
use solana_bpf_loader_program::direct_deploy_program;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::{bpf_loader_upgradeable, sysvar};

fn main() {
    // read the input
    let elf = env::read_frame();

    let transaction_accounts = vec![(
        sysvar::epoch_schedule::id(),
        create_account_shared_data_for_test(&EpochSchedule::default()),
    )];
    with_mock_invoke_context!(invoke_context, transaction_context, transaction_accounts);
    let program_id = Pubkey::new_unique();
    let env = std::sync::Arc::new(BuiltinProgram::new_mock());
    let program = ProgramCacheEntry {
        program: ProgramCacheEntryType::Unloaded(env),
        account_owner: ProgramCacheEntryOwner::LoaderV2,
        account_size: 0,
        deployment_slot: 0,
        effective_slot: 0,
        tx_usage_counter: AtomicU64::new(100),
        ix_usage_counter: AtomicU64::new(100),
        latest_access_slot: AtomicU64::new(0),
    };
    invoke_context
        .program_cache_for_tx_batch
        .replenish(program_id, Arc::new(program));

    // deploy the program
    let res = direct_deploy_program(
        &mut invoke_context,
        &program_id,
        &bpf_loader_upgradeable::id(),
        elf.len(),
        &elf,
        2,
    );
    assert!(res.is_ok(), "Program deployment failed: {:?}", res);

    // find the upgraded program in the cache
    let updated_program = invoke_context
        .program_cache_for_tx_batch
        .find(&program_id)
        .expect("Didn't find upgraded program in the cache");

    assert_eq!(updated_program.deployment_slot, 2);
    assert_eq!(
        updated_program.tx_usage_counter.load(Ordering::Relaxed),
        100
    );
    assert_eq!(
        updated_program.ix_usage_counter.load(Ordering::Relaxed),
        100
    );
    
    // write public output to the journal
    env::commit(&true);
}
