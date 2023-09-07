use anchor_lang::prelude::*;
use crate::{ClaimPrize, User};
use crate::errors::KamikazeJoeError;
use anchor_lang::solana_program::{system_program};
use crate::{checks::{check_address}};


pub fn handler(ctx: Context<ClaimPrize>) -> Result<()> {

    let mut receiver_account = ctx.accounts.payer.to_account_info();

    if ctx.accounts.user.key() != User::pda(ctx.accounts.user.authority).0 {
        return Err(KamikazeJoeError::InvalidAuthority.into());
    }

    // Check delegation
    if let Some(destination) = ctx.accounts.receiver.clone() {
        if ctx.accounts.user.authority.key() != destination.key() {
            return Err(KamikazeJoeError::InvalidAuthority.into());
        }
        receiver_account = destination.to_account_info();
    }

    check_address(
        ctx.accounts.system_program.to_account_info().key,
        &system_program::ID,
        "system_program"
    )?;

    let game = &mut ctx.accounts.game;

    if !game.can_claim(receiver_account.unsigned_key()){
        return Err(KamikazeJoeError::InvalidClaim.into());
    }

    game.prize_claimed = true;
    ctx.accounts.user.won += 1;

    // Calculate the reward (90% of the price pool, 10% is kept in the vault)
    let payout = ((game.ticket_price * game.players.len() as u64) * 9) / 10 ;

    let vault_lamports = ctx.accounts.vault.to_account_info().lamports().checked_sub(payout);
    let user_lamports = receiver_account.lamports().checked_add(payout);

    // Transfer price to the winner
    **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? = vault_lamports.ok_or(KamikazeJoeError::Overflow)?;
    **receiver_account.try_borrow_mut_lamports()? = user_lamports.ok_or(KamikazeJoeError::Overflow)?;

    Ok(())
}
