use risc0_zkvm::guest::env;
use solana_sdk::feature_set::FeatureSet;
use solana_sdk::hash::Hash;
use solana_sdk::k256::ecdsa::SigningKey;
use solana_sdk::secp256k1_instruction::new_secp256k1_instruction;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

fn main() {
    // read the input
    let input = env::read_frame();
    
    let signing_key = SigningKey::from_slice(&input).expect("Invalid input for signing key");
    let message = b"Hello, world!";

    let mint_keypair = Keypair::new();
    let feature_set = FeatureSet::all_enabled();
    let instruction = new_secp256k1_instruction(&signing_key, message);
    let tx = Transaction::new_signed_with_payer(
        &[instruction.clone()],
        Some(&mint_keypair.pubkey()),
        &[&mint_keypair],
        Hash::default(),
    );
    let res = tx.verify_precompiles(&feature_set).is_ok();

    // write public output to the journal
    env::commit(&res);
}
