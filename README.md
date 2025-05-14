# Solana NFT Program
In this project built with Rust, Anchor and Metaplex, we write the logic to mint nfts to a solana wallet on the devnet. (Same works for mainnet)

## Minting NFT: Mechanism
What happens when an NFT is minted? 
First, a mint account is created and initialized (this is the address of the token we are creating), then an associated token account is created to hold and keep count of the amount of those tokens present in a wallet. That means that a wallet does not hold a token directly, but a token account.

### mint.rs
In our mint.rs file, we write the logic to mint. Anchor requires that we declare a program id which is where our program will be deployed to. This comes right out of the box and is present in the `lib.rs` and `Anchor.toml` files.

Our overall mint function takes the following arguments
- `metadata_title, metadata_symbol, metadata_string` which all come from the client side calling the function
- a Context<T> argument which is how Anchor receives all the accounts to be involved in our program transactions.
In this case, we pass `Context<MintNft>` which means our Context holds `MintNft` struct. This MintNft struct holds the accounts and their types involved in minting an nft.

#### MintNft struct
In the MintNft struct, some accounts are already pre existing while some will be created by our program (PDAs). We have to call the `#[derive(Accounts)]` macro on the MintNft struct and also allocate it with a `MintNft<'info>` lifetime. This is how Anchor annotates lifetimes. For the struct fields, pdas are of the type `UncheckedAccount` while CPI programs (eg system program, token program) are of the type `Program`.  The full `MintNft` struct looks like:
```
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
```
Notice that these struct fields are only initialized and an instance of the struct is never assigned. Anchor handles this with a .accounts() method called on the client side which we will handle later.

#### Creating the mint account (the actual token)
Creating an account of any type on solana is handled by the system program which we get from `use anchor_lang::system_program`. We use `system_program::create()` function which takes a context, amount of lamports, owner (owner of all tokens is the token program and space) Thus, our mint creation function:
```
system_program::create_account(
    CpiContext::new(
        ctx.accounts.token_program.to_account_info() // involved program
        system_program::CreateAccount { // CreateAccount struct eby system program
            from: ctx.accounts.mint_authority.to_account_info(),
            to: ctx.accounts.mint.to_account_info()
        }
        1000000000,
        82,
        &ctx.accounts.token_program.key()
    )
)?;
```

The rest of our mint.rs file follows the same pattern.