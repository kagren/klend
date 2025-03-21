use anchor_lang::{prelude::*, Accounts};

use crate::{
    utils::{seeds::BASE_SEED_REFERRER_TOKEN_STATE, REFERRER_TOKEN_STATE_SIZE},
    LendingMarket, ReferrerTokenState, Reserve,
};

pub fn process(ctx: Context<InitReferrerTokenState>) -> Result<()> {
    let mut referrer_token_state = ctx.accounts.referrer_token_state.load_init()?;
    let reserve = ctx.accounts.reserve.load()?;
    let referrer = ctx.accounts.referrer.key();
    let bump = ctx.bumps.referrer_token_state;

    *referrer_token_state = ReferrerTokenState {
        referrer,
        mint: reserve.liquidity.mint_pubkey,
        amount_unclaimed_sf: 0.into(),
        amount_cumulative_sf: 0.into(),
        bump: bump.into(),
        padding: [0; 31],
    };

    Ok(())
}

#[derive(Accounts)]
pub struct InitReferrerTokenState<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub lending_market: AccountLoader<'info, LendingMarket>,

    #[account(
        has_one = lending_market
    )]
    pub reserve: AccountLoader<'info, Reserve>,

    pub referrer: AccountInfo<'info>,

    #[account(init,
        seeds = [BASE_SEED_REFERRER_TOKEN_STATE, referrer.key().as_ref(), reserve.key().as_ref()],
        bump,
        payer = payer,
        space = REFERRER_TOKEN_STATE_SIZE + 8,
    )]
    pub referrer_token_state: AccountLoader<'info, ReferrerTokenState>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}
