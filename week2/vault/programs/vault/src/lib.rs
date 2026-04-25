#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::prelude::*;

declare_id!("39AzeK3okakk8JgggwnXhK45ZSoEJ9LNyXpwB1WgSg3S");


#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.vault.set_inner(Vault {
            authority: ctx.accounts.authority.key(),
            balance: 0,
        });
        Ok(())
    }

    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        require!(ctx.accounts.authority.key() == vault.authority, VaultError::Unauthorized);

        **ctx.accounts.authority.try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.vault_account.try_borrow_mut_lamports()? += amount;

        vault.balance = vault
            .balance
            .checked_add(amount)
            .ok_or(VaultError::Overflow)?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        require!(ctx.accounts.authority.key() == vault.authority, VaultError::Unauthorized);
        require!(vault.balance >= amount, VaultError::InsufficientFunds);

        **ctx.accounts.vault_account.try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.authority.try_borrow_mut_lamports()? += amount;

        vault.balance = vault
            .balance
            .checked_sub(amount)
            .ok_or(VaultError::Overflow)?;

        Ok(())
    }

    pub fn close(ctx: Context<Close>) -> Result<()> {
        let vault = &ctx.accounts.vault;
        require!(ctx.accounts.authority.key() == vault.authority, VaultError::Unauthorized);

        let remaining_balance = **ctx.accounts.vault_account.lamports.borrow();
        **ctx.accounts.vault_account.try_borrow_mut_lamports()? -= remaining_balance;
        **ctx.accounts.authority.try_borrow_mut_lamports()? += remaining_balance;

        Ok(())
    }
}

#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub balance: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 8, seeds = [b"vault", authority.key().as_ref()], bump)]
    /// CHECK: I’ll check this account manually in the code
    pub vault: Account<'info, Vault>,

    /// CHECK: I’ll check this account manually in the code
    #[account(mut, seeds = [b"vault_account", authority.key().as_ref()], bump)]
    pub vault_account: AccountInfo<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut, seeds = [b"vault", authority.key().as_ref()], bump)]
    pub vault: Account<'info, Vault>,
    
/// CHECK: This account holds lamports and is accessed securely via seeds + bump
    #[account(mut, seeds = [b"vault_account", authority.key().as_ref()], bump)]
    pub vault_account: AccountInfo<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut, close = authority, seeds = [b"vault", authority.key().as_ref()], bump)]
    pub vault: Account<'info, Vault>,
    /// CHECK: Vault account manually validated by seeds + bump, only holds lamports
    #[account(mut, seeds = [b"vault_account", authority.key().as_ref()], bump)]
    pub vault_account: AccountInfo<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
}
#[error_code]
pub enum VaultError {
    #[msg("Unauthorized access.")]
    Unauthorized,
    #[msg("Insufficient funds.")]
    InsufficientFunds,
    #[msg("Math overflow.")]
    Overflow,
}
