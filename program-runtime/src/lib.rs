#![cfg_attr(RUSTC_WITH_SPECIALIZATION, feature(min_specialization))]
#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::indexing_slicing)]

#[cfg(not(target_os = "zkvm"))]
#[macro_use]
extern crate eager;

#[cfg(not(target_os = "zkvm"))]
#[macro_use]
extern crate solana_metrics;

pub use solana_rbpf;
pub mod invoke_context;
pub mod loaded_programs;
pub mod log_collector;
pub mod mem_pool;
pub mod stable_log;
pub mod sysvar_cache;
#[cfg(not(target_os = "zkvm"))]
pub mod timings;
