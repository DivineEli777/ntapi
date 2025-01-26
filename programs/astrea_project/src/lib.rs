use anchor_lang::prelude::*;

declare_id!("5jDeDKprZXSX2U2NqWDLxSLodTJ1L5UkMaJYTp2A4oyg");

#[program]
pub mod astrea_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, InitializeMint, MintTo, TokenInitializeAccount};

declare_id!("5jDeDKprZXSX2U2NqWDLxSLodTJ1L5UkMaJYTp2A4oyg");

#[program]
pub mod astrea_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, total_supply: u64) -> Result<()> {
        // Initialize the mint
        let cpi_accounts = InitializeMint {
            mint: ctx.accounts.mint.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::initialize_mint(cpi_ctx, 9, &ctx.accounts.authority.key(), None)?;

        // Mint tokens to the token account
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, total_supply)?;

        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority, mint::decimals = 9, mint::authority = authority)]
    pub mint: Account<'info, Mint>,
    #[account(init, payer = authority, token::mint = mint, token::authority = authority)]
    pub token_account: Account<'info, TokenAccount>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}Initialize