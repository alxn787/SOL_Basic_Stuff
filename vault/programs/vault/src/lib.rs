use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

declare_id!("Eoc1inyPY4F8mMz2PpzwcYGTrB7LbxW1ssHhitmR2V6C");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.authority = ctx.accounts.authority.key();
        vault.authority_bump = ctx.bumps.vault_authority;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeVault <'info>{

    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 1,
    )]
    pub vault : Account<'info, Vault>,

    #[account(
        seeds = [b"vault_authority".as_ref(), vault.key().as_ref()],
        bump
    )]
    /// CHECK: This is the vault authority account
    pub vault_authority : UncheckedAccount<'info>,

    #[account(
        init,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = vault_authority,
    )]
    pub vault_token_account : Account<'info, TokenAccount>,

    pub mint : Account<'info, Mint>,

    #[account(mut)]
    pub authority : Signer<'info>,
    pub system_program : Program<'info, System>,
    pub token_program : Program<'info, Token>,
    pub associated_token_program : Program<'info, AssociatedToken>,
    pub rent : Sysvar<'info, Rent>,

}

#[derive(Accounts)]
pub struct DepositVault<'info> {

    #[account(mut)]
    pub vault : Account<'info, Vault>,

    #[account(
        mut,
        seeds = [b"vault_authority".as_ref(), vault.key().as_ref()],
        bump
    )]
    pub vault_authority : UncheckedAccount<'info>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = vault_authority,
    )]
    pub vault_token_account : Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = authority,
    )]
    pub user_token_account : Account<'info, TokenAccount>,

    pub mint : Account<'info, Mint>,

    #[account(mut)]
    pub authority : Signer<'info>,

    pub system_program : Program<'info, System>,
    pub token_program : Program<'info, Token>,
    pub rent : Sysvar<'info, Rent>,
}

#[account]
pub struct Vault {
    pub authority : Pubkey,
    pub authority_bump : u8
}
