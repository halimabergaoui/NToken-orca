use crate::instructions::portfolio_instruction::PortfolioInstruction;
use solana_program::{instruction::AccountMeta, pubkey::Pubkey};

pub fn orca_swap_reverse_instruction(
    amount: u64,
    other_amount_threshold: u64,
    sqrt_price_limit: u128,
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
) -> (Vec<u8>, Vec<AccountMeta>) {
    let data = PortfolioInstruction::OrcaSwapReverse{
        amount,
        other_amount_threshold,
        sqrt_price_limit,
    }
    .pack();
    let accounts = vec![
        AccountMeta::new_readonly(*program, false),
        AccountMeta::new_readonly(*token_program, false),
        AccountMeta::new(*whirlpool, false),
        AccountMeta::new(*token_authority, true),
        AccountMeta::new(*token_owner_account_a, false),
        AccountMeta::new(*token_vault_a, false),
        AccountMeta::new(*token_owner_account_b, false),
        AccountMeta::new(*token_vault_b, false),
        AccountMeta::new(*tick_array_0, false),
        AccountMeta::new(*tick_array_1, false),
        AccountMeta::new(*tick_array_2, false),
        AccountMeta::new(*oracle, false),
    ];
    (data, accounts)
}
