use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgmt5yqE6PqL"); // Replace with your program ID

// Define maximum lengths for strings (adjust as needed)
const TITLE_LEN: usize = 50;
const ARTIST_LEN: usize = 50;
const DESCRIPTION_LEN: usize = 200;
const METADATA_URI_LEN: usize = 200;

#[account]
pub struct MusicNFT {
    pub title: String,         // Name of the music track
    pub artist: String,        // Name of the artist
    pub description: String,   // Short description of the track
    pub metadata_uri: String,  // IPFS link for metadata and cover image
    pub mint: Pubkey,          // Public key of the NFT mint
    pub owner: Pubkey,         // Public key of the NFT owner
    pub royalty_percentage: u8,// Percentage of royalties for the artist
}

impl MusicNFT {
    pub const LEN: usize = 
          4 + TITLE_LEN
        + 4 + ARTIST_LEN
        + 4 + DESCRIPTION_LEN
        + 4 + METADATA_URI_LEN
        + 32  // mint: Pubkey
        + 32  // owner: Pubkey
        + 1;  // royalty_percentage: u8
}

#[program]
pub mod my_solana_project {
    use super::*;

    pub fn initialize_nft(
        ctx: Context<InitializeNFT>,
        title: String,
        artist: String,
        description: String,
        metadata_uri: String,
        mint: Pubkey,
        royalty_percentage: u8,
    ) -> Result<()> {
        let nft = &mut ctx.accounts.nft_account;
        nft.title = title;
        nft.artist = artist;
        nft.description = description;
        nft.metadata_uri = metadata_uri;
        nft.mint = mint;
        nft.owner = *ctx.accounts.artist.key;
        nft.royalty_percentage = royalty_percentage;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeNFT<'info> {
    #[account(init, payer = artist, space = 8 + MusicNFT::LEN)]
    pub nft_account: Account<'info, MusicNFT>,
    
    #[account(mut)]
    pub artist: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}
