use anchor_lang::{system_program, prelude::*};
use anchor_lang::solana_program::program::invoke;
use anchor_spl::{associated_token, token};
use anchor_spl::{associated_token::AssociatedToken, token::Token};
use mpl_token_metadata::instructions as token_instruction;
use mpl_token_metadata::types::DataV2;


pub fn mint(ctx: Context<MintNft>, metadata_title: String, metadata_symbol: String, metadata_uri: String) -> Result<()> {
        msg!("Creating mint account...");
        msg!("Mint: {}", &ctx.accounts.mint.key());

        system_program::create_account(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                system_program::CreateAccount{
                    from: ctx.accounts.mint_authority.to_account_info(),
                    to: ctx.accounts.mint.to_account_info()
                }
            ),
            100000000,
            82,
            &ctx.accounts.token_program.key()
        )?;

        msg!("Initializing mint account...");
        msg!("Mint: {}", &ctx.accounts.mint.key());
        token::initialize_mint(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), token::InitializeMint{
                mint: ctx.accounts.mint.to_account_info(),
                rent: ctx.accounts.rent.to_account_info()
            }),
            0,
            &ctx.accounts.mint_authority.key(),
            Some(&ctx.accounts.mint_authority.key())
        )?;

        msg!("Creating token account...");
        msg!("Token Address: {}", &ctx.accounts.token_account.key());
        associated_token::create(
            CpiContext::new(
                ctx.accounts.associated_token_program.to_account_info(),
                associated_token::Create { 
                    payer: ctx.accounts.mint_authority.to_account_info(),
                    associated_token: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(), 
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    
                }
            )
        )?;

        msg!("Minting token to token account...");
        msg!("Mint: {}", &ctx.accounts.mint.to_account_info().key());
        msg!("Token Address: {}", &ctx.accounts.token_account.key());
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo{
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.mint_authority.to_account_info()
                }
            ),
            1
        )?;

        msg!("Creating metadata account...");
        msg!("Metadata account address: {}", &ctx.accounts.metadata.to_account_info().key());

        let meta_data = DataV2 { name: metadata_title, symbol: metadata_symbol, uri: metadata_uri, seller_fee_basis_points: 1, creators: None, collection: None, uses: None };

        let create_metadata = token_instruction::CreateMetadataAccountV3Builder::new()
                                                            .metadata(ctx.accounts.metadata.key())
                                                            .mint(ctx.accounts.mint.key())
                                                            .mint_authority(ctx.accounts.mint_authority.key())
                                                            .payer(ctx.accounts.mint_authority.key())
                                                            .update_authority(ctx.accounts.mint_authority.key(), true)
                                                            .is_mutable(false)
                                                            .data(meta_data)
                                                            .instruction();
        invoke(
            &create_metadata,
            &[
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
                ctx.accounts.token_metadata_program.to_account_info()            
            ]
        )?;

        msg!("creating master edition metadata account...");
        msg!("Master edition metadata account address: {}", &ctx.accounts.master_edition.to_account_info().key());
        let create_master = token_instruction::CreateMasterEditionV3Builder::new()
                                                            .edition(ctx.accounts.master_edition.key())
                                                            .mint(ctx.accounts.mint.key())
                                                            .mint_authority(ctx.accounts.mint_authority.key())
                                                            .update_authority(ctx.accounts.mint_authority.key())
                                                            .payer(ctx.accounts.mint_authority.key())
                                                            .metadata(ctx.accounts.metadata.key())
                                                            .payer(ctx.accounts.mint_authority.key())
                                                            .max_supply(0)
                                                            .instruction();
        invoke(
            &create_master,
            &[
                ctx.accounts.master_edition.to_account_info(),
                ctx.accounts.mint.to_account_info(),
                ctx.accounts.mint_authority.to_account_info(),
                ctx.accounts.metadata.to_account_info(),
                ctx.accounts.token_account.to_account_info(),
                ctx.accounts.rent.to_account_info()
            ]
        )?;

        msg!("Token mint process completed successfully");
        Ok(())

}


#[derive(Accounts)]
pub struct MintNft<'info> {
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: We're about to create this with Metaplex
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint: Signer<'info>,
    /// CHECK: We're about to create this with Anchor
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: Metaplex will check this
    pub token_metadata_program: UncheckedAccount<'info>
}
