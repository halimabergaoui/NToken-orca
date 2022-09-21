//! Program state processor

use crate::{
    error::PortfolioError,
    instructions::portfolio_instruction::PortfolioInstruction,
    processors::{
        orca_pools::{
            close_position::fn_close_position, decrease_liquidity::fn_decrease_liquidity,
            increase_liquidity::fn_increase_liquidity, open_position::fn_open_position,
        },
        orca_swap::orca_swap::fn_orca_swap,
    },
};

pub use solana_program;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

/// Program state handler.
pub struct Processor {}
impl Processor {
    /// Processes an [Instruction](enum.Instruction.html).
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        let instruction = PortfolioInstruction::unpack(input)?;

        match instruction {
            PortfolioInstruction::OpenPosition {
                position_bump,
                tick_lower_index,
                tick_upper_index,
            } => {
                msg!("Instruction:  Open Position");
                Self::process_open_position(
                    program_id,
                    accounts,
                    position_bump,
                    tick_lower_index,
                    tick_upper_index,
                )
            }

            PortfolioInstruction::ClosePosition {} => {
                msg!("Instruction:  Open Position");
                Self::process_close_position(program_id, accounts)
            }

            PortfolioInstruction::IncreaseLiquidity {
                liquidity_amount,
                token_max_a,
                token_max_b,
            } => {
                msg!("Instruction:  Open Position");
                Self::process_increase_liquidity(
                    program_id,
                    accounts,
                    liquidity_amount,
                    token_max_a,
                    token_max_b,
                )
            }

            PortfolioInstruction::DecreaseLiquidity {
                liquidity_amount,
                token_min_a,
                token_min_b,
            } => {
                msg!("Instruction:  Decrese Liquidity");
                Self::process_decrese_liquidity(
                    program_id,
                    accounts,
                    liquidity_amount,
                    token_min_a,
                    token_min_b,
                )
            }

            PortfolioInstruction::OrcaSwap {
                amount,
                other_amount_threshold,
                sqrt_price_limit,
            } => {
                msg!("Instruction:  Decrese Liquidity");
                Self::process_orca_swap(
                    program_id,
                    accounts,
                    amount,
                    other_amount_threshold,
                    sqrt_price_limit,
                )
            }

            PortfolioInstruction::OrcaSwapReverse {
                amount,
                other_amount_threshold,
                sqrt_price_limit,
            } => {
                msg!("Instruction:  Decrese Liquidity");
                Self::process_orca_swap_reverse(
                    program_id,
                    accounts,
                    amount,
                    other_amount_threshold,
                    sqrt_price_limit,
                )
            }
        }
    }

    /// update the state of splu tertiary
    pub fn process_open_position(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        position_bump: u8,
        tick_lower_index: i32,
        tick_upper_index: i32,
    ) -> ProgramResult {
        fn_open_position(
            program_id,
            accounts,
            position_bump,
            tick_lower_index,
            tick_upper_index,
        )?;
        Ok(())
    }

    /// update the state of splu tertiary
    pub fn process_close_position(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        fn_close_position(program_id, accounts)?;
        Ok(())
    }

    /// update the state of splu tertiary
    pub fn process_increase_liquidity(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        liquidity_amount: u128,
        token_max_a: u64,
        token_max_b: u64,
    ) -> ProgramResult {
        fn_increase_liquidity(
            program_id,
            accounts,
            liquidity_amount,
            token_max_a,
            token_max_b,
        )?;
        Ok(())
    }

    /// update the state of splu tertiary
    pub fn process_decrese_liquidity(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        liquidity_amount: u128,
        token_min_a: u64,
        token_min_b: u64,
    ) -> ProgramResult {
        fn_decrease_liquidity(
            program_id,
            accounts,
            liquidity_amount,
            token_min_a,
            token_min_b,
        )?;
        Ok(())
    }

    /// update the state of splu tertiary
    pub fn process_orca_swap(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
        other_amount_threshold: u64,
        sqrt_price_limit: u128,
    ) -> ProgramResult {
        fn_orca_swap(
            program_id,
            accounts,
            amount,
            other_amount_threshold,
            sqrt_price_limit,
            true,
            true,
        )?;
        Ok(())
    }

    /// update the state of splu tertiary
    pub fn process_orca_swap_reverse(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
        other_amount_threshold: u64,
        sqrt_price_limit: u128,
    ) -> ProgramResult {
        fn_orca_swap(
            program_id,
            accounts,
            amount,
            other_amount_threshold,
            sqrt_price_limit,
            true,
            false,
        )?;
        Ok(())
    }
}
