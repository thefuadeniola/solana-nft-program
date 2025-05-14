use anchor_lang::prelude::*;

declare_id!("5xcosctyu88U7nAcRniYisw96kppr3KNZxqYMogRrE2j");

pub mod mint;
pub mod sell;

use mint::*;
use sell::*;

#[program]
pub mod nft_current {
    use super::*;

    pub fn mint(ctx: Context<MintNft>, metadata_title: String, metadata_symbol: String, metadata_uri: String) -> Result<()> {
        mint::mint(ctx, metadata_title, metadata_symbol, metadata_uri)
    }

    pub fn sell(ctx: Context<SellNft>, sale_lamports: u64) -> Result<()> {
        sell::sell(ctx, sale_lamports)
    }
}
