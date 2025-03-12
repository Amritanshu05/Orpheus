use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

declare_id!("2ABtyKKMJA13gzVWUxMhT3EeuzZWm2KCGKUAx7jghUtA");

#[program]
pub mod my_solana_project {
    use super::*;
    use anchor_spl::token::{self, MintTo, Transfer};

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Music NFT Platform initialized!");
        Ok(())
    }

    pub fn mint_music_nft(
        ctx: Context<MintMusicNFT>,
        title: String,
        artist: String,
        description: String,
        metadata_uri: String,
        royalty_percentage: u8,
    ) -> Result<()> {
        // Validate royalty percentage (0-100)
        require!(
            royalty_percentage <= 100,
            MusicNFTError::InvalidRoyaltyPercentage
        );

        let music_nft = &mut ctx.accounts.music_nft;
        let artist_account = &ctx.accounts.artist;
        
        // Initialize the NFT data
        music_nft.title = title;
        music_nft.artist = artist;
        music_nft.description = description;
        music_nft.metadata_uri = metadata_uri;
        music_nft.mint = ctx.accounts.mint.key();
        music_nft.owner = artist_account.key();
        music_nft.royalty_percentage = royalty_percentage;
        music_nft.bump = ctx.bumps.music_nft;

        // Mint 1 token to the artist's token account
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.artist.to_account_info(),
                },
            ),
            1,
        )?;

        msg!("Music NFT minted successfully!");
        Ok(())
    }

    pub fn transfer_nft(
        ctx: Context<TransferNFT>,
        amount: u64,
    ) -> Result<()> {
        // Transfer the NFT to the new owner
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.from_token_account.to_account_info(),
                    to: ctx.accounts.to_token_account.to_account_info(),
                    authority: ctx.accounts.owner.to_account_info(),
                },
            ),
            amount,
        )?;

        // Update the owner in the music NFT account
        let music_nft = &mut ctx.accounts.music_nft;
        music_nft.owner = ctx.accounts.new_owner.key();

        msg!("NFT transferred successfully!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
    title: String,
    artist: String,
    description: String,
    metadata_uri: String,
    royalty_percentage: u8
)]
pub struct MintMusicNFT<'info> {
    #[account(mut)]
    pub artist: Signer<'info>,
    
    #[account(
        init,
        payer = artist,
        space = MusicNFT::LEN,
        seeds = [b"music-nft", mint.key().as_ref()],
        bump
    )]
    pub music_nft: Account<'info, MusicNFT>,
    
    #[account(
        init,
        payer = artist,
        mint::decimals = 0,
        mint::authority = artist,
        mint::freeze_authority = artist,
    )]
    pub mint: Account<'info, Mint>,
    
    #[account(
        init,
        payer = artist,
        associated_token::mint = mint,
        associated_token::authority = artist,
    )]
    pub token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct TransferNFT<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    
    /// CHECK: This is the new owner of the NFT
    pub new_owner: UncheckedAccount<'info>,
    
    #[account(
        mut,
        seeds = [b"music-nft", mint.key().as_ref()],
        bump = music_nft.bump,
        constraint = music_nft.owner == owner.key() @ MusicNFTError::NotOwner,
    )]
    pub music_nft: Account<'info, MusicNFT>,
    
    pub mint: Account<'info, Mint>,
    
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = owner,
    )]
    pub from_token_account: Account<'info, TokenAccount>,
    
    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = mint,
        associated_token::authority = new_owner,
    )]
    pub to_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct MusicNFT {
    pub title: String,
    pub artist: String,
    pub description: String,
    pub metadata_uri: String,
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub royalty_percentage: u8,
    pub bump: u8,
}

impl MusicNFT {
    const LEN: usize = 8 + // discriminator
        4 + 200 + // title (String with max 200 chars)
        4 + 100 + // artist (String with max 100 chars)
        4 + 500 + // description (String with max 500 chars)
        4 + 200 + // metadata_uri (String with max 200 chars)
        32 + // mint (Pubkey)
        32 + // owner (Pubkey)
        1 + // royalty_percentage (u8)
        1; // bump (u8)
}

#[error_code]
pub enum MusicNFTError {
    #[msg("Royalty percentage must be between 0 and 100")]
    InvalidRoyaltyPercentage,
    #[msg("Only the owner can transfer this NFT")]
    NotOwner,
}
