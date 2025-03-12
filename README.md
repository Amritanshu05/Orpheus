# Music NFT Platform on Solana

This project implements a Music NFT platform on the Solana blockchain using Anchor framework. It allows artists to mint NFTs for their music tracks with royalty support.

## Features

- Create music NFTs with metadata (title, artist, description, etc.)
- Set royalty percentages for secondary sales
- Mint NFTs with zero decimals (non-fungible)
- Transfer NFTs between users

## Prerequisites

- Solana CLI (v2.1.15 or later)
- Anchor CLI (v0.30.0)
- Rust (v1.85.0 or later)
- Node.js (v23.9.0 or later)
- Yarn (v1.22.22 or later)

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd my_solana_project
```

2. Install dependencies:
```bash
yarn install
```

3. Build the program:
```bash
anchor build
```

## Program Structure

The program consists of the following components:

1. **MusicNFT Struct**: Defines the data structure for music NFTs with fields for title, artist, description, metadata URI, mint, owner, and royalty percentage.

2. **Mint Function**: Implements the functionality to mint a new music NFT.

3. **Transfer Function**: Implements the functionality to transfer an NFT to a new owner.

## Usage

### Initializing the Program

```typescript
const tx = await program.methods.initialize().accounts({
  initializer: wallet.publicKey,
  systemProgram: SystemProgram.programId,
}).rpc();
```

### Minting a Music NFT

```typescript
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

// Mint the NFT
const tx = await program.methods
  .mintMusicNft(
    "My First Music NFT",
    "Artist Name",
    "This is a description of my music track",
    "https://ipfs.io/ipfs/QmXxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
    10 // 10% royalty
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
```

### Transferring an NFT

```typescript
const newOwner = new PublicKey("..."); // The new owner's public key
const newOwnerTokenAccount = await getAssociatedTokenAddress(
  mintPublicKey, // The mint of the NFT
  newOwner
);

const tx = await program.methods
  .transferNft(1) // Transfer 1 token
  .accounts({
    owner: wallet.publicKey,
    newOwner: newOwner,
    musicNft: musicNftPda,
    mint: mintPublicKey,
    fromTokenAccount: tokenAccount,
    toTokenAccount: newOwnerTokenAccount,
    tokenProgram: TOKEN_PROGRAM_ID,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    systemProgram: SystemProgram.programId,
    rent: SYSVAR_RENT_PUBKEY,
  })
  .rpc();
```

## Testing

Run the tests with:

```bash
anchor test
```

## Deployment

1. Update the program ID in `Anchor.toml` and `lib.rs` with your program ID:

```bash
solana-keygen new -o target/deploy/my_solana_project-keypair.json
anchor keys list
```

2. Update the program ID in `lib.rs` and `Anchor.toml`.

3. Deploy the program:

```bash
anchor deploy
```

## License

This project is licensed under the MIT License. 