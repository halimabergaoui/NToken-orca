#![forbid(unsafe_code)]

//! Portfolio program for the Solana blockchain

//pub mod error;
pub use ntoken_solana_models::error as  error;
pub mod instruction;
pub mod processor;


#[warn(non_snake_case)]
pub mod processors;
pub mod instructions;
// pub mod states;
#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;
// pub mod constants;
pub use ntoken_solana_models::states as  states;
pub  use ntoken_solana_models::constants as  constants;
// Export current sdk types for downstream users building with a different sdk version
pub use solana_program;
//use solana_program::declare_id;
//declare_id!("Novahs7LgN54oaHtE8ztEy1ayJwfBydry85DhVjdZwk");

