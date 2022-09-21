
pub use solana_program;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
};

use anchor_lang::prelude::*;

pub fn fn_orca_swap(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64, 
    other_amount_threshold: u64, 
    sqrt_price_limit: u128, 
    amount_specified_is_input:bool, 
    a_to_b:bool

) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to  
    let program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let whirlpool = next_account_info(accounts_iter)?;
    let token_authority = next_account_info(accounts_iter)?;
    let token_owner_account_a = next_account_info(accounts_iter)?;
    let token_vault_a = next_account_info(accounts_iter)?;
    let token_owner_account_b = next_account_info(accounts_iter)?;
    let token_vault_b = next_account_info(accounts_iter)?;
    let tick_array_0 = next_account_info(accounts_iter)?;
    let tick_array_1 = next_account_info(accounts_iter)?;
    let tick_array_2 = next_account_info(accounts_iter)?;
    let oracle = next_account_info(accounts_iter)?;
    
    msg!( " amount, other_amount_threshold, sqrt_price_limit {:?}, {:?},{:?}",amount, other_amount_threshold, sqrt_price_limit);

    let accounts = whirlpool::cpi::accounts::Swap{

        
        token_program: token_program.clone(),
        whirlpool : whirlpool.clone(),
        token_authority : token_authority.clone(),
        token_owner_account_a : token_owner_account_a.clone(),
        token_vault_a : token_vault_a.clone(),
        token_owner_account_b : token_owner_account_b.clone(),
        token_vault_b : token_vault_b.clone(),
        tick_array_0 : tick_array_0.clone(),
        tick_array_1 : tick_array_1.clone(),
        tick_array_2 : tick_array_2.clone(),
        oracle : oracle.clone()
    };

    let ctx = CpiContext::new(program.clone(), accounts);
    whirlpool::cpi::swap(ctx, amount, other_amount_threshold, sqrt_price_limit, amount_specified_is_input, a_to_b);

    Ok(())
}
