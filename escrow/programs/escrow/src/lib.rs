use anchor_lang::prelude::*;

declare_id!("4rzg2P9UcKgwbuCbSYyw5XGV3Z5Uxxit85tuTK1tHYkU");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
