use anchor_lang::prelude::*;

#[account]
#[derive(Default)] // defaults to zeros -- which we want 
pub struct PoolState {
    pub total_amount_minted: u64,
    pub fee_numerator: u64,
    pub fee_denominator: u64,
}


impl PoolState {
    // total_amount_minted: u64 needs 8 bytes
    // fee_numerator: u64 needs 8 bytes
    // fee_denominator: u64 needs 8 bytes
    pub fn init_size() -> usize {
        let total_amount_minted_size: usize = 8;
        let fee_numerator_size: usize = 8;
        let fee_denominator_size: usize = 8;

        let total_size: usize = (total_amount_minted_size + fee_numerator_size + fee_denominator_size) * 2;

        return total_size;
    }
}

