import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MySolanaProject } from "../target/types/my_solana_project";
import {
  PublicKey,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  Keypair,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
} from "@solana/spl-token";
import { assert } from "chai";

describe("my_solana_project", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MySolanaProject as Program<MySolanaProject>;
  const provider = anchor.getProvider();
  const wallet = (provider as anchor.AnchorProvider).wallet;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accounts({
      initializer: wallet.publicKey,
      systemProgram: SystemProgram.programId,
    }).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Can mint a music NFT", async () => {
    // Generate a new keypair for the mint
    const mintKeypair = Keypair.generate();
    
    // Find PDA for the music NFT account
    const [musicNftPda] = await PublicKey.findProgramAddressSync(
      [Buffer.from("music-nft"), mintKeypair.publicKey.toBuffer()],
      program.programId
    );

    // Find the associated token account for the artist
    const tokenAccount = await getAssociatedTokenAddress(
      mintKeypair.publicKey,
      wallet.publicKey
    );

    // NFT metadata
    const title = "My First Music NFT";
    const artist = "Test Artist";
    const description = "This is a test music NFT";
    const metadataUri = "https://ipfs.io/ipfs/QmXxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
    const royaltyPercentage = 10; // 10%

    try {
      // Mint the NFT
      const tx = await program.methods
        .mintMusicNft(
          title,
          artist,
          description,
          metadataUri,
          royaltyPercentage
        )
        .accounts({
          artist: wallet.publicKey,
          musicNft: musicNftPda,
          mint: mintKeypair.publicKey,
          tokenAccount: tokenAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
          rent: SYSVAR_RENT_PUBKEY,
        })
        .signers([mintKeypair])
        .rpc();

      console.log("Your transaction signature", tx);

      // Fetch the music NFT account to verify it was created correctly
      const musicNftAccount = await program.account.musicNFT.fetch(musicNftPda);
      
      // Verify the NFT data
      assert.equal(musicNftAccount.title, title);
      assert.equal(musicNftAccount.artist, artist);
      assert.equal(musicNftAccount.description, description);
      assert.equal(musicNftAccount.metadataUri, metadataUri);
      assert.equal(musicNftAccount.royaltyPercentage, royaltyPercentage);
      assert.isTrue(musicNftAccount.mint.equals(mintKeypair.publicKey));
      assert.isTrue(musicNftAccount.owner.equals(wallet.publicKey));
      
      console.log("Music NFT minted and verified successfully!");

      // Test transferring the NFT
      const newOwner = Keypair.generate();
      const newOwnerTokenAccount = await getAssociatedTokenAddress(
        mintKeypair.publicKey,
        newOwner.publicKey
      );

      const transferTx = await program.methods
        .transferNft(1) // Transfer 1 token
        .accounts({
          owner: wallet.publicKey,
          newOwner: newOwner.publicKey,
          musicNft: musicNftPda,
          mint: mintKeypair.publicKey,
          fromTokenAccount: tokenAccount,
          toTokenAccount: newOwnerTokenAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
          rent: SYSVAR_RENT_PUBKEY,
        })
        .rpc();

      console.log("Transfer transaction signature", transferTx);

      // Fetch the music NFT account again to verify the new owner
      const updatedMusicNftAccount = await program.account.musicNFT.fetch(musicNftPda);
      assert.isTrue(updatedMusicNftAccount.owner.equals(newOwner.publicKey));
      
      console.log("NFT transferred successfully!");
    } catch (error) {
      console.error("Error:", error);
      throw error;
    }
  });
});
