use crate::instructions::portfolio_instruction::PortfolioInstruction;
use solana_program::{instruction::AccountMeta, pubkey::Pubkey};

pub fn increase_liquidity_instruction(
    liquidity_amount:u128,
    token_max_a:u64,
    token_max_b:u64,
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
) -> (Vec<u8>, Vec<AccountMeta>) {
    let data = PortfolioInstruction::IncreaseLiquidity { liquidity_amount, token_max_a, token_max_b }.pack();
    let accounts = vec![
        AccountMeta::new_readonly(*program, false),
        AccountMeta::new(*position_authority, true),
        AccountMeta::new(*position, false),
        AccountMeta::new(*position_token_account, false),
        AccountMeta::new_readonly(*token_program, true),
        AccountMeta::new(*whirlpool, false),
        AccountMeta::new(*token_owner_account_a, false),
        AccountMeta::new(*token_owner_account_b, false),
        AccountMeta::new(*token_vault_a, false),
        AccountMeta::new(*token_vault_b, false),
        AccountMeta::new(*tick_array_lower, false),
        AccountMeta::new(*tick_array_upper, false),
    ];
    (data, accounts)
}