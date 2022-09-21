
pub use solana_program;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
};

use anchor_lang::prelude::*;

pub fn fn_increase_liquidity(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    liquidity_amount:u128,
    token_max_a:u64,
    token_max_b:u64
) -> ProgramResult {   
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let program = next_account_info(accounts_iter)?;
    let position_authority=next_account_info(accounts_iter)?;
    let position=next_account_info(accounts_iter)?;
    let position_token_account=next_account_info(accounts_iter)?;
    let token_program=next_account_info(accounts_iter)?;
    let whirlpool=next_account_info(accounts_iter)?;
    let token_owner_account_a=next_account_info(accounts_iter)?;
    let token_owner_account_b=next_account_info(accounts_iter)?;
    let token_vault_a=next_account_info(accounts_iter)?;
    let token_vault_b=next_account_info(accounts_iter)?;
    let tick_array_lower=next_account_info(accounts_iter)?;
    let tick_array_upper=next_account_info(accounts_iter)?;



    let accounts = whirlpool::cpi::accounts::ModifyLiquidity {
        position_authority: position_authority.clone(),
        position: position.clone(),
        position_token_account: position_token_account.clone(),        
        token_program: token_program.clone(),
        whirlpool: whirlpool.clone(),
        token_owner_account_a: token_owner_account_a.clone(),
        token_owner_account_b: token_owner_account_b.clone(),
        token_vault_a: token_vault_a.clone(),
        token_vault_b: token_vault_b.clone(),
        tick_array_lower: tick_array_lower.clone(),
        tick_array_upper: tick_array_upper.clone()
        };

    let ctx =CpiContext::new(program.clone(), accounts);
whirlpool::cpi::increase_liquidity(ctx, liquidity_amount, token_max_a, token_max_b); 
    Ok(())
}