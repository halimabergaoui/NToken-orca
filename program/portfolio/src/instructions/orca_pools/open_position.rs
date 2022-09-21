use crate::instructions::portfolio_instruction::PortfolioInstruction;
use solana_program::{instruction::AccountMeta, pubkey::Pubkey};

pub fn open_position_instruction(
    position_bump: u8,
    tick_lower_index: i32,
    tick_upper_index: i32,
    program : &Pubkey,
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
) -> (Vec<u8>, Vec<AccountMeta>) {
    let data = PortfolioInstruction::OpenPosition{ position_bump, tick_lower_index, tick_upper_index }.pack();
    let accounts = vec![
        AccountMeta::new_readonly(*program, false),
        AccountMeta::new(*position, false),
        AccountMeta::new(*position_mint, false),
        AccountMeta::new(*position_token_account, false),
        AccountMeta::new_readonly(*token_program, false),
        AccountMeta::new(*funder, true),
        AccountMeta::new(*owner, true),
        AccountMeta::new(*whirlpool, false),
        AccountMeta::new_readonly(*system_program, false),
        AccountMeta::new_readonly(*rent, false),
        AccountMeta::new_readonly(*associated_token_program, false),
    ];
    (data, accounts)
}