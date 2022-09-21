use crate::instructions::portfolio_instruction::PortfolioInstruction;
use solana_program::{instruction::AccountMeta, pubkey::Pubkey};

pub fn close_position_instruction(
     program: &Pubkey,
     position_authority: &Pubkey,
     receiver: &Pubkey,
     position: &Pubkey,
     position_mint: &Pubkey,
     position_token_account: &Pubkey,
     token_program: &Pubkey,
) -> (Vec<u8>, Vec<AccountMeta>) {
    let data = PortfolioInstruction::ClosePosition{}.pack();
    let accounts = vec![
        AccountMeta::new_readonly(*program, false),
        AccountMeta::new(*position_authority, true),
        AccountMeta::new(*receiver, false),
        AccountMeta::new(*position, false),
        AccountMeta::new(*position_mint, false),
        AccountMeta::new(*position_token_account, false),
        AccountMeta::new_readonly(*token_program, false),
    ];
    (data, accounts)
}