use anchor_lang::prelude::*;

declare_id!("Eoc1inyPY4F8mMz2PpzwcYGTrB7LbxW1ssHhitmR2V6C");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
