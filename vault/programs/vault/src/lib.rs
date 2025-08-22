use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

declare_id!("Eoc1inyPY4F8mMz2PpzwcYGTrB7LbxW1ssHhitmR2V6C");

#[program]
pub mod vault {
    use anchor_spl::token::{self, Transfer};

    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.authority = ctx.accounts.authority.key();
        vault.authority_bump = ctx.bumps.vault_authority;

        Ok(())
    }

    pub fn deposit_to_vault(ctx: Context<DepositToVault>, amount: u64) -> Result<()> {
        
        let cpi_context = CpiContext::new(
           ctx.accounts.token_program.to_account_info(),
           Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.vault_token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
           }
        );

        token::transfer(cpi_context, amount)?;

        msg!("Vault deposited successfully");

        Ok(())
    }

    pub fn withdraw_from_vault(ctx: Context<WithdrawFromVault>, amount: u64) -> Result<()>  {

        require!(ctx.accounts.vault.authority == ctx.accounts.authority.key(), VaultError::Unauthorized);

        let program = ctx.accounts.token_program.to_account_info();
        let accounts = Transfer {
            from: ctx.accounts.vault_token_account.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        };

        let vault_key  = ctx.accounts.vault.key();

        let signer_seeds: &[&[&[u8]]] = &[&[b"vault_authority", vault_key.as_ref(), &[ctx.accounts.vault.authority_bump]]];   

        let cpi_context = CpiContext::new_with_signer(program, accounts, signer_seeds);

        token::transfer(cpi_context, amount)?;

        msg!("Vault withdrawn successfully");

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
pub struct DepositToVault<'info> {

    #[account(mut)]
    pub vault : Account<'info, Vault>,

    #[account(
        seeds = [b"vault_authority".as_ref(), vault.key().as_ref()],
        bump
    )]
    /// CHECK: This is the vault authority account
    pub vault_authority : UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = vault_token_account.mint == mint.key(),
        constraint = vault_token_account.owner == vault_authority.key(),
    )]
    pub vault_token_account : Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = user_token_account.mint == mint.key(),
        constraint = user_token_account.owner == authority.key(),
    )]
    pub user_token_account : Account<'info, TokenAccount>,

    pub mint : Account<'info, Mint>,

    #[account(mut)]
    pub authority : Signer<'info>,

    pub system_program : Program<'info, System>,
    pub token_program : Program<'info, Token>,
    pub rent : Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct WithdrawFromVault<'info> {

    #[account(mut)]
    pub vault : Account<'info, Vault>,

    #[account(
        mut,
        seeds = [b"vault_authority".as_ref(), vault.key().as_ref()],
        bump
    )]
    /// CHECK: This is the vault authority account
    pub vault_authority : UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = vault_token_account.mint == mint.key(),
        constraint = vault_token_account.owner == vault_authority.key(),
    )]
    pub vault_token_account : Account<'info, TokenAccount>,
 
    #[account(
        mut,
        constraint = user_token_account.mint == mint.key(),
        constraint = user_token_account.owner == authority.key(),
    )]
    pub user_token_account : Account<'info, TokenAccount>,

    pub mint : Account<'info, Mint>,

    #[account(mut)]
    pub authority : Signer<'info>,

    pub token_program : Program<'info, Token>,
}

#[account]
pub struct Vault {
    pub authority : Pubkey,
    pub authority_bump : u8   
}

#[error_code]
pub enum VaultError {
    #[msg("Unauthorized access to vault")]
    Unauthorized,
}
