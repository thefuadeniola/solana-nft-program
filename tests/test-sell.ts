import * as anchor from "@coral-xyz/anchor";
import { NftCurrent } from "../target/types/nft_current";
import { createKeypairFromFile } from "./util";

describe("sell-nft", async () => {
    const provider = anchor.AnchorProvider.env();
    const wallet = provider.wallet as anchor.Wallet;

    anchor.setProvider(provider);

    const program = anchor.workspace.NftCurrent as anchor.Program<NftCurrent>;

    it("Sell!", async() => {
        const saleAmount = 1 * anchor.web3.LAMPORTS_PER_SOL;
        const mint: anchor.web3.PublicKey = new anchor.web3.PublicKey(
            "8Qtywc2Cs8iJSi4sTfHvSiQw1p5dsyTV8DpGHzPDFf5i"
        );

        const buyer: anchor.web3.Keypair = await createKeypairFromFile(__dirname + "/keypairs/buyer1.json");
        console.log(`buyer public key: ${buyer.publicKey}`);

        // Derive the associated token account address for owner & buyer

        const ownerTokenAddress = await anchor.utils.token.associatedAddress({
            mint: mint,
            owner: wallet.publicKey
        })

        const buyerTokenAddress = await anchor.utils.token.associatedAddress({
            mint: mint,
            owner: buyer.publicKey
        })

        console.log(`Request to sell NFT: ${mint} for ${saleAmount} lamports`);
        console.log(`Owner's token address: ${ownerTokenAddress}`);
        console.log(`Buyer's token address: ${buyerTokenAddress}`)

        // Transact with the 'sell' function

        await program.methods.sell(
            new anchor.BN(saleAmount)
        )
        .accounts({
            mint: mint,
            ownerTokenAccount: ownerTokenAddress,
            buyerTokenAccount: buyerTokenAddress,
            buyerAuthority: buyer.publicKey
        })
        .signers([buyer])
        .rpc();

    })
    
})