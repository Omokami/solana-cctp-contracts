//! RemoveLocalToken instruction handler

use {
    crate::token_minter::{
        error::TokenMinterError,
        events::LocalTokenRemoved,
        state::{LocalToken, TokenMinter},
    },
    anchor_lang::prelude::*,
    anchor_spl::token::{Token, TokenAccount},
};

// Instruction accounts
#[derive(Accounts)]
pub struct RemoveLocalTokenContext<'info> {
    #[account(mut)]
    pub payee: Signer<'info>,

    #[account()]
    pub token_controller: Signer<'info>,

    #[account(
        has_one = token_controller @ TokenMinterError::InvalidAuthority,
    )]
    pub token_minter: Box<Account<'info, TokenMinter>>,

    #[account(
        mut,
        seeds = [
            b"local_token",
            local_token.mint.as_ref(),
        ],
        bump = local_token.bump,
        close = payee
    )]
    pub local_token: Box<Account<'info, LocalToken>>,

    #[account(
        mut,
        seeds = [
            b"custody",
            local_token.mint.as_ref()
        ],
        bump = local_token.custody_bump,
    )]
    pub custody_token_account: Box<Account<'info, TokenAccount>>,

    pub token_program: Program<'info, Token>,
}

// Instruction parameters
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone)]
pub struct RemoveLocalTokenParams {}

// Instruction handler
pub fn remove_local_token(
    ctx: Context<RemoveLocalTokenContext>,
    _params: &RemoveLocalTokenParams,
) -> Result<()> {
    let local_token = ctx.accounts.local_token.as_ref();

    ctx.accounts.token_minter.close_token_account(
        ctx.accounts.payee.to_account_info(),
        ctx.accounts.custody_token_account.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        ctx.accounts.token_minter.to_account_info(),
    )?;

    emit!(LocalTokenRemoved {
        custody: local_token.custody,
        mint: local_token.mint,
    });

    Ok(())
}