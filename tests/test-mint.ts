import * as anchor from "@coral-xyz/anchor";
import { NftCurrent } from "../target/types/nft_current";
import { ComputeBudgetProgram } from "@solana/web3.js";

describe("nft_current", () => {

  const testNftTitle = "Na SCRA"
  const testNftSymbol = "SCRA"
  const testNftUri = "https://blush-soft-cricket-531.mypinata.cloud/ipfs/bafkreibgxjsckrkrubkh6nt6ewrwofkcn2sqx7qdodndu5vcvhh6o5xai4"

  const provider = anchor.AnchorProvider.env();
  const wallet = provider.wallet as anchor.Wallet;

  anchor.setProvider(provider);

  const program = anchor.workspace.NftCurrent as anchor.Program<NftCurrent>;

  const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

  it("Mint!", async () => {
    // Derive the mint address and the asscociated token account address.
    const mintKeypair = anchor.web3.Keypair.generate();
    const tokenAddress = anchor.utils.token.associatedAddress({
      mint: mintKeypair.publicKey,
      owner: wallet.publicKey
    })
    console.log(`New token: ${mintKeypair.publicKey}`);

    // Derive the metadata and master edition addresses
    const metadataAddress = (anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mintKeypair.publicKey.toBuffer()
      ],
      TOKEN_METADATA_PROGRAM_ID
    ))[0];

        const masterEditionAddress = (anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mintKeypair.publicKey.toBuffer(),
        Buffer.from("edition")
      ],
      TOKEN_METADATA_PROGRAM_ID
    ))[0];

    console.log("Master edition metadata initialized")

    console.log(`All accounts: ${masterEditionAddress}, ${metadataAddress}, ${mintKeypair.publicKey}, ${tokenAddress}, ${wallet.publicKey}, ${TOKEN_METADATA_PROGRAM_ID}`)
    // Transact with the "mint" function in our onchain program

    await program.methods.mint(testNftTitle, testNftSymbol, testNftUri)
    .accounts({
      masterEdition: masterEditionAddress,
      metadata: metadataAddress,
      mint: mintKeypair.publicKey,
      tokenAccount: tokenAddress,
      mintAuthority: wallet.publicKey,
      tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID
    })
    .preInstructions([
      ComputeBudgetProgram.setComputeUnitLimit({ units: 400_000 })
    ])
    .signers([mintKeypair])
    .rpc()
  });
});
