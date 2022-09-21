

use crate::constants::instructions_number::*;
use crate::error::PortfolioError;
use solana_program::pubkey::Pubkey;
use solana_program::{
    program_error::ProgramError
};
use std::convert::TryInto;
use std::mem::size_of;

use super::portfolio_instruction::PortfolioInstruction;


impl PortfolioInstruction {
    /// Unpacks a byte buffer into a [PortfolioInstruction](enum.PortfolioInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        use PortfolioError::InvalidInstruction;
        
        let (&tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        Ok(match tag {
            1 =>{
                let (position_bump, rest) = rest.split_at(1);
                let position_bump = position_bump
                    .try_into()
                    .ok()
                    .map(u8::from_le_bytes)
                    .ok_or(InvalidInstruction)?;
               
                    let (tick_lower_index, rest1) = rest.split_at(4);
                    let tick_lower_index = tick_lower_index
                        .try_into()
                        .ok()
                        .map(i32::from_le_bytes)
                        .ok_or(InvalidInstruction)?;

                    let (tick_upper_index, rest2) = rest1.split_at(4);
                    let tick_upper_index = tick_upper_index
                        .try_into()
                        .ok()
                        .map(i32::from_le_bytes)
                        .ok_or(InvalidInstruction)?;
                Self::OpenPosition { position_bump, tick_lower_index, tick_upper_index }
            } ,

            2 =>{
                Self::ClosePosition {  } 
            },

            3 =>{
                let (liquidity_amount, rest) = rest.split_at(16);
                let liquidity_amount = liquidity_amount
                    .try_into()
                    .ok()
                    .map(u128::from_le_bytes)
                    .ok_or(InvalidInstruction)?;
               
                    let (token_max_a, rest1) = rest.split_at(8);
                    let token_max_a = token_max_a
                        .try_into()
                        .ok()
                        .map(u64::from_le_bytes)
                        .ok_or(InvalidInstruction)?;

                    let (token_max_b, rest2) = rest1.split_at(8);
                    let token_max_b = token_max_b
                        .try_into()
                        .ok()
                        .map(u64::from_le_bytes)
                        .ok_or(InvalidInstruction)?;
                Self::IncreaseLiquidity { liquidity_amount, token_max_a, token_max_b }
            },

            4 =>{
                let (liquidity_amount, rest) = rest.split_at(16);
                let liquidity_amount = liquidity_amount
                    .try_into()
                    .ok()
                    .map(u128::from_le_bytes)
                    .ok_or(InvalidInstruction)?;
               
                    let (token_min_a, rest1) = rest.split_at(8);
                    let token_min_a = token_min_a
                        .try_into()
                        .ok()
                        .map(u64::from_le_bytes)
                        .ok_or(InvalidInstruction)?;

                    let (token_min_b, rest2) = rest1.split_at(8);
                    let token_min_b = token_min_b
                        .try_into()
                        .ok()
                        .map(u64::from_le_bytes)
                        .ok_or(InvalidInstruction)?;
                Self::DecreaseLiquidity { liquidity_amount, token_min_a, token_min_b }
            },

            5 =>{
                let (amount, rest) = rest.split_at(8);
                let amount = amount
                    .try_into()
                    .ok()
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?;
               
                    let (other_amount_threshold, rest1) = rest.split_at(8);
                    let other_amount_threshold = other_amount_threshold
                        .try_into()
                        .ok()
                        .map(u64::from_le_bytes)
                        .ok_or(InvalidInstruction)?;

                    let (sqrt_price_limit, rest2) = rest1.split_at(16);
                    let sqrt_price_limit = sqrt_price_limit
                        .try_into()
                        .ok()
                        .map(u128::from_le_bytes)
                        .ok_or(InvalidInstruction)?;

                Self::OrcaSwap { amount, other_amount_threshold, sqrt_price_limit}
            },

            6 =>{
                let (amount, rest) = rest.split_at(8);
                let amount = amount
                    .try_into()
                    .ok()
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?;
               
                    let (other_amount_threshold, rest1) = rest.split_at(8);
                    let other_amount_threshold = other_amount_threshold
                        .try_into()
                        .ok()
                        .map(u64::from_le_bytes)
                        .ok_or(InvalidInstruction)?;

                    let (sqrt_price_limit, rest2) = rest1.split_at(16);
                    let sqrt_price_limit = sqrt_price_limit
                        .try_into()
                        .ok()
                        .map(u128::from_le_bytes)
                        .ok_or(InvalidInstruction)?;

                Self::OrcaSwapReverse { amount, other_amount_threshold, sqrt_price_limit}
            } 
            _ => return Err(PortfolioError::InvalidInstruction.into())

        })
    }

    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match self {
            &Self::OpenPosition {position_bump , tick_lower_index , tick_upper_index } => {
                buf.push(1);
                buf.extend_from_slice(&position_bump.to_le_bytes());
                buf.extend_from_slice(&tick_lower_index.to_le_bytes());
                buf.extend_from_slice(&tick_upper_index.to_le_bytes());
            },
            &Self::ClosePosition {} => {
                buf.push(2);
            },
            &Self::IncreaseLiquidity { liquidity_amount, token_max_a, token_max_b } => {
                buf.push(3);
                buf.extend_from_slice(&liquidity_amount.to_le_bytes());
                buf.extend_from_slice(&token_max_a.to_le_bytes());
                buf.extend_from_slice(&token_max_b.to_le_bytes());
            },
            &Self::DecreaseLiquidity { liquidity_amount, token_min_a, token_min_b } => {
                buf.push(4);
                buf.extend_from_slice(&liquidity_amount.to_le_bytes());
                buf.extend_from_slice(&token_min_a.to_le_bytes());
                buf.extend_from_slice(&token_min_b.to_le_bytes());
            },
            &Self::OrcaSwap { amount, other_amount_threshold, sqrt_price_limit } => {
                buf.push(5);
                buf.extend_from_slice(&amount.to_le_bytes());
                buf.extend_from_slice(&other_amount_threshold.to_le_bytes());
                buf.extend_from_slice(&sqrt_price_limit.to_le_bytes());
            },
            &Self::OrcaSwapReverse { amount, other_amount_threshold, sqrt_price_limit } => {
                buf.push(5);
                buf.extend_from_slice(&amount.to_le_bytes());
                buf.extend_from_slice(&other_amount_threshold.to_le_bytes());
                buf.extend_from_slice(&sqrt_price_limit.to_le_bytes());
            },
            
        };
        buf
    }
      
    pub(crate) fn unpack_pubkey(input: &[u8]) -> Result<(Pubkey, &[u8]), ProgramError> {
        if input.len() >= 32 {
            let (key, rest) = input.split_at(32);
            let pk = Pubkey::new(key);
            Ok((pk, rest))
        } else {
            Err(PortfolioError::InvalidInstruction.into())
        }
    }
}
