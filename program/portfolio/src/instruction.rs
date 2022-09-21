//! Instruction types

use crate::instructions::{orca_pools::{open_position::open_position_instruction, increase_liquidity::increase_liquidity_instruction, decrease_liquidity::decrease_liquidity_instruction, close_position::close_position_instruction}, orca_swap::{orca_swap::{orca_swap_instruction}, orca_swap_reverse::orca_swap_reverse_instruction}};

use solana_program::{
    entrypoint::ProcessInstruction, instruction::Instruction, program_error::ProgramError,
    pubkey::Pubkey,
};

/// Creates a `claim rewards` instruction, to see more details for this instruction see [`claim_rewards_instruction`].
pub fn open_position(
    program_id: &Pubkey,
    program: &Pubkey,
    position: &Pubkey,
    position_mint: &Pubkey,
    position_token_account: &Pubkey,
    token_program: &Pubkey,
    funder: &Pubkey,
    owner: &Pubkey,
    whirlpool: &Pubkey,
    system_program: &Pubkey,
    rent: &Pubkey,
    associated_token_program: &Pubkey,
    position_bump: u8,
    tick_lower_index: i32,
    tick_upper_index: i32,
) -> Result<Instruction, ProgramError> {
    let (data, accounts) = open_position_instruction(
        position_bump,
        tick_lower_index,
        tick_upper_index,
        program,
        position,
        position_mint,
        position_token_account,
        token_program,
        funder,
        owner,
        whirlpool,
        system_program,
        rent,
        associated_token_program,
    );
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

pub fn close_position(
    program_id: &Pubkey,
    program: &Pubkey,
    position_authority: &Pubkey,
    receiver: &Pubkey,
    position: &Pubkey,
    position_mint: &Pubkey,
    position_token_account: &Pubkey,
    token_program: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let (data, accounts) = close_position_instruction(program,
         position_authority, receiver, position, position_mint, position_token_account, token_program);
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

pub fn increase_liquidity(
    program_id: &Pubkey,
    program : &Pubkey,
    position_authority: &Pubkey,
    position: &Pubkey,
    position_token_account: &Pubkey,
    token_program: &Pubkey,
    whirlpool: &Pubkey,
    token_owner_account_a: &Pubkey,
    token_owner_account_b: &Pubkey,
    token_vault_a: &Pubkey,
    token_vault_b: &Pubkey,
    tick_array_lower: &Pubkey,
    tick_array_upper: &Pubkey,
    liquidity_amount:u128,
    token_max_a:u64,
    token_max_b:u64,
) -> Result<Instruction, ProgramError> {
    let (data, accounts) = increase_liquidity_instruction(
        liquidity_amount, 
        token_max_a, 
        token_max_b, 
        program, 
        position_authority, 
        position, 
        position_token_account, 
        token_program, 
        whirlpool, 
        token_owner_account_a, 
        token_owner_account_b, 
        token_vault_a, 
        token_vault_b, 
        tick_array_lower, 
        tick_array_upper
    );
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

pub fn decrease_liquidity(
    program_id: &Pubkey,
    program : &Pubkey,
    position_authority: &Pubkey,
    position: &Pubkey,
    position_token_account: &Pubkey,
    token_program: &Pubkey,
    whirlpool: &Pubkey,
    token_owner_account_a: &Pubkey,
    token_owner_account_b: &Pubkey,
    token_vault_a: &Pubkey,
    token_vault_b: &Pubkey,
    tick_array_lower: &Pubkey,
    tick_array_upper: &Pubkey,
    liquidity_amount:u128,
    token_min_a:u64,
    token_min_b:u64,
) -> Result<Instruction, ProgramError> {
    let (data, accounts) = decrease_liquidity_instruction(liquidity_amount, token_min_a, token_min_b, program, position_authority, position, position_token_account, token_program, whirlpool, token_owner_account_a, token_owner_account_b, token_vault_a, token_vault_b, tick_array_lower, tick_array_upper);
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

pub fn orca_swap(
    program_id: &Pubkey,
    program: &Pubkey,
    token_program: &Pubkey,
    whirlpool: &Pubkey,
    token_authority: &Pubkey,
    token_owner_account_a: &Pubkey,
    token_vault_a: &Pubkey,
    token_owner_account_b: &Pubkey,
    token_vault_b: &Pubkey,
    tick_array_0: &Pubkey,
    tick_array_1: &Pubkey,
    tick_array_2: &Pubkey,
    oracle: &Pubkey,
    amount: u64,
    other_amount_threshold: u64,
    sqrt_price_limit: u128,
) -> Result<Instruction, ProgramError> {
    let (data, accounts) = orca_swap_instruction(amount, other_amount_threshold, sqrt_price_limit, program, token_program, whirlpool, token_authority, token_owner_account_a, token_vault_a, token_owner_account_b, token_vault_b, tick_array_0, tick_array_1, tick_array_2, oracle);
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

pub fn orca_swap_reverse(
    program_id: &Pubkey,
    program: &Pubkey,
    token_program: &Pubkey,
    whirlpool: &Pubkey,
    token_authority: &Pubkey,
    token_owner_account_a: &Pubkey,
    token_vault_a: &Pubkey,
    token_owner_account_b: &Pubkey,
    token_vault_b: &Pubkey,
    tick_array_0: &Pubkey,
    tick_array_1: &Pubkey,
    tick_array_2: &Pubkey,
    oracle: &Pubkey,
    amount: u64,
    other_amount_threshold: u64,
    sqrt_price_limit: u128,
) -> Result<Instruction, ProgramError> {
    let (data, accounts) = orca_swap_reverse_instruction(amount, other_amount_threshold, sqrt_price_limit, program, token_program, whirlpool, token_authority, token_owner_account_a, token_vault_a, token_owner_account_b, token_vault_b, tick_array_0, tick_array_1, tick_array_2, oracle);
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}
