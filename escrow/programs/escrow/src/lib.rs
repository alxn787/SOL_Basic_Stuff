use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

declare_id!("4rzg2P9UcKgwbuCbSYyw5XGV3Z5Uxxit85tuTK1tHYkU");

#[program]
pub mod escrow {
    use anchor_spl::token::Transfer;

    use super::*;

    pub fn initialize_escrow(ctx: Context<InitializeEscrow>, amount: u64) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;

        escrow.authority = ctx.accounts.authority.key();
        escrow.recipient = ctx.accounts.recipient.key();
        escrow.mint = ctx.accounts.mint.key();
        escrow.amount = amount;
        escrow.vault_authority_bump = ctx.bumps.vault_authority;

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer{
                from : ctx.accounts.authority_token_account.to_account_info(),
                to : ctx.accounts.escrow_token_account.to_account_info(),
                authority : ctx.accounts.authority.to_account_info(),
            },
        );
        anchor_spl::token::transfer(cpi_ctx, amount)?;

        Ok(())
    }
    pub fn release_escrow(ctx: Context<ReleaseEscrow>) -> Result<()> {
        require!(ctx.accounts.escrow.recipient == ctx.accounts.recipient.key(), EscrowError::InvalidRecipient);

        let escrow_key = ctx.accounts.escrow.key();

        let signer_seeds: &[&[&[u8]]] = &[&[b"vault_authority", escrow_key.as_ref(),&[ctx.accounts.escrow.vault_authority_bump]]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer{
                from : ctx.accounts.escrow_token_account.to_account_info(),
                to : ctx.accounts.recipient_token_account.to_account_info(),
                authority : ctx.accounts.vault_authority.to_account_info(),
            },
            signer_seeds
        );
        anchor_spl::token::transfer(cpi_ctx, ctx.accounts.escrow.amount)?;
        
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

    #[account(
        constraint = authority_token_account.mint == mint.key(),
        constraint = authority_token_account.owner == authority.key(),
    )]
    pub authority_token_account : Account<'info, TokenAccount>,

    #[account(mut)]
    pub recipient : AccountInfo<'info>,

    pub system_program : Program<'info, System>,
    pub token_program : Program<'info, Token>,
    pub associated_token_program : Program<'info, AssociatedToken>,
    pub rent : Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ReleaseEscrow<'info> {
    #[account(
        mut,
        has_one = recipient,
        
    )]
    pub escrow : Account<'info, Escrow>,

    #[account(mut,
        seeds = [b"vault_authority".as_ref(), escrow.key().as_ref()],
        bump,
    )]
    /// CHECK: This is the vault authority account
    pub vault_authority : UncheckedAccount<'info>,

    pub mint : Account<'info, Mint>,

    #[account(
        constraint = escrow_token_account.mint == mint.key(),
        constraint = escrow_token_account.owner == vault_authority.key(),
    )]
    pub escrow_token_account : Account<'info, TokenAccount>,

    #[account(
        constraint = recipient_token_account.mint == mint.key(),
        constraint = recipient_token_account.owner == recipient.key(),
    )]
    pub recipient_token_account : Account<'info, TokenAccount>,

    pub recipient : Signer<'info>,
    pub token_program : Program<'info,Token>
}

#[account]
pub struct Escrow {
    pub authority : Pubkey,
    pub recipient : Pubkey,
    pub mint : Pubkey,
    pub amount : u64,
    pub vault_authority_bump : u8,
}

#[error_code]
pub enum EscrowError {
    #[msg("Invalid recipient")]
    InvalidRecipient,
}