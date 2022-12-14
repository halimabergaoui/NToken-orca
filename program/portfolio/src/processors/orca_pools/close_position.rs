
pub use solana_program;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
};

use anchor_lang::prelude::*;

pub fn fn_close_position(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> ProgramResult {
    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let program = next_account_info(accounts_iter)?;
    let position_authority=next_account_info(accounts_iter)?;
    let receiver=next_account_info(accounts_iter)?;
    let position=next_account_info(accounts_iter)?;
    let position_mint=next_account_info(accounts_iter)?;
    let position_token_account=next_account_info(accounts_iter)?;
    let token_program=next_account_info(accounts_iter)?;



    let accounts = whirlpool::cpi::accounts::ClosePosition {
        position_authority: position_authority.clone(),
        receiver: receiver.clone(),
        position: position.clone(),
        position_mint: position_mint.clone(),
        position_token_account: position_token_account.clone(),        
        token_program: token_program.clone(),
    };

    let ctx =CpiContext::new(program.clone(), accounts);
whirlpool::cpi::close_position(ctx);   

    Ok(())
}
