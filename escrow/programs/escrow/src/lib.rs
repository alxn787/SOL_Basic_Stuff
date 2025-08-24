use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

declare_id!("4rzg2P9UcKgwbuCbSYyw5XGV3Z5Uxxit85tuTK1tHYkU");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize_escrow(ctx: Context<InitializeEscrow>) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;

        escrow.authority = ctx.accounts.authority.key();
        escrow.recipient = ctx.accounts.recipient.key();
        escrow.mint = ctx.accounts.mint.key();
        escrow.amount = 0;
        escrow.vault_authority_bump = ctx.bumps.vault_authority;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeEscrow<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 32 + 8 + 1,
    )]
    pub escrow : Account<'info, Escrow>,

    #[account(
        seeds = [b"vault_authority".as_ref(), escrow.key().as_ref()],
        bump
    )]
    /// CHECK: This is the vault authority account
    pub vault_authority : UncheckedAccount<'info>,

    pub mint : Account<'info, Mint>,

    #[account(
        init,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = vault_authority,
    )]
    pub escrow_token_account : Account<'info, TokenAccount>,

    #[account(mut)]
    pub authority : Signer<'info>,

    #[account(mut)]
    pub recipient : AccountInfo<'info>,

    pub system_program : Program<'info, System>,
    pub token_program : Program<'info, Token>,
    pub associated_token_program : Program<'info, AssociatedToken>,
    pub rent : Sysvar<'info, Rent>,
}

#[account]
pub struct Escrow {
    pub authority : Pubkey,
    pub recipient : Pubkey,
    pub mint : Pubkey,
    pub amount : u64,
    pub vault_authority_bump : u8,
}
