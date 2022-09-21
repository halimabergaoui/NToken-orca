use solana_program::pubkey::Pubkey;

/// Instructions supported by the portfolio program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum PortfolioInstruction {
    ///1
    OpenPosition {
        position_bump: u8,
        tick_lower_index: i32,
        tick_upper_index: i32,
    },
    ///2
    ClosePosition {},
    ///3
    IncreaseLiquidity {
        liquidity_amount: u128,
        token_max_a: u64,
        token_max_b: u64,
    },
    ///4
    DecreaseLiquidity {
        liquidity_amount: u128,
        token_min_a: u64,
        token_min_b: u64,
    },

    ///5
    OrcaSwap {
        amount: u64,
        other_amount_threshold: u64,
        sqrt_price_limit: u128,
    },

    ///6
    OrcaSwapReverse {
        amount: u64,
        other_amount_threshold: u64,
        sqrt_price_limit: u128,
    },
}
