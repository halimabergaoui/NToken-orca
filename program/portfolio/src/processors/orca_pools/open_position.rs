
pub use solana_program;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

use anchor_lang::prelude::*;

pub fn fn_open_position(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    position_bump: u8,
    tick_lower_index:i32,
    tick_upper_index:i32,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
   
    // Get the account to say hello to
    let program = next_account_info(accounts_iter)?;
    let position=next_account_info(accounts_iter)?;
    let position_mint=next_account_info(accounts_iter)?;
    let position_token_account=next_account_info(accounts_iter)?;
    let token_program=next_account_info(accounts_iter)?;
    let funder=next_account_info(accounts_iter)?;
    let owner=next_account_info(accounts_iter)?;
    let whirlpool=next_account_info(accounts_iter)?;
    let system_program=next_account_info(accounts_iter)?;
    let rent=next_account_info(accounts_iter)?;
    let associated_token_program=next_account_info(accounts_iter)?;



    let accounts = whirlpool::cpi::accounts::OpenPosition {
        position: position.clone(),
        position_mint: position_mint.clone(),
        position_token_account: position_token_account.clone(),        
        token_program: token_program.clone(),
        funder: funder.clone(),
        owner: owner.clone(),
        whirlpool: whirlpool.clone(),
        system_program: system_program.clone(),
        rent: rent.clone(),
        associated_token_program: associated_token_program.clone(),
    };
    let ctx =CpiContext::new(program.clone(), accounts);
whirlpool::cpi::open_position(ctx,whirlpool::state::OpenPositionBumps { position_bump: position_bump },tick_lower_index,tick_upper_index);   

    Ok(())
}
