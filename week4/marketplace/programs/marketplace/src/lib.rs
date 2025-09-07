#![allow(deprecated)]
#![allow(unexpected_cfgs)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("9eq285CTju48tBaaenQbqQAfG3cK9qMSQ5xA9dpaZjoB");

#[program]
pub mod anchor_marketplace {
    use super::*;

    pub fn initialize_marketplace(ctx: Context<InitializeMarketplace>, fee_percentage: u8) -> Result<()> {
        ctx.accounts.initialize_marketplace(fee_percentage, ctx.bumps)?;
        Ok(())
    }

     pub fn list_nft(ctx: Context<ListNft>, price: u64) -> Result<()> {
        ctx.accounts.initialize_listing(price, ctx.bumps)?;
        ctx.accounts.transfer_nft()
    }

    pub fn purchase_nft(ctx: Context<PurchaseNft>) -> Result<()> {
        ctx.accounts.transfer_nft()?;
        ctx.accounts.transfer_sol()?;
        ctx.accounts.delist_nft()
    }

}
    pub fn delist_nft(ctx: Context<DelistNft>) -> Result<()> {
        ctx.accounts.transfer_back_nft()
    }

