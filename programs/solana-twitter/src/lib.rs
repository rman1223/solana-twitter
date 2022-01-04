use anchor_lang::prelude::*;

declare_id!("BdoQ1KTMk4w9vAwX3JMumqAeuoUsJvYHT6qV2YtvXPKH");

#[program]
pub mod solana_twitter {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
