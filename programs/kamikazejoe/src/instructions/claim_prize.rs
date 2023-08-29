use anchor_lang::prelude::*;
use crate::{ClaimPrize};
use crate::errors::KamikazeJoeError;

pub fn handler(ctx: Context<ClaimPrize>) -> Result<()> {

    let game = &mut ctx.accounts.game;

    if !game.can_claim(ctx.accounts.player.unsigned_key()){
        return Err(KamikazeJoeError::InvalidClaim.into());
    }

    game.prize_claimed = true;
    ctx.accounts.user.won += 1;

    // Calculate the reward (90% of the price pool, 10% is kept in the vault)
    let payout = ((game.ticket_price * game.players.len() as u64) * 9) / 10 ;

    // Transfer price to the winner
    **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= payout;
    **ctx.accounts.player.to_account_info().try_borrow_mut_lamports()? += payout;

    Ok(())
}
