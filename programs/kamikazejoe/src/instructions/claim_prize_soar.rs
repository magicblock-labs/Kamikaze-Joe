use anchor_lang::prelude::*;
use crate::{ClaimPrizeSoar, User};
use crate::errors::KamikazeJoeError;
use anchor_lang::solana_program::{system_program};
use soar_cpi::cpi;
use soar_cpi::cpi::accounts::SubmitScore;
use crate::{checks::{check_address}};
use crate::seeds::LEADERBOARD;


pub fn handler(ctx: Context<ClaimPrizeSoar>) -> Result<()> {

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

    // Submit to the Soar leaderboard
    msg!("Submitting the score to Soar!");

    msg!("Receiver account: {}, Payer: {}", receiver_account.key().to_string(), ctx.accounts.payer.key.to_string());

    let accounts = SubmitScore {
        payer: ctx.accounts.payer.to_account_info(),
        authority: ctx.accounts.leaderboard_info.to_account_info(),
        player_account: ctx.accounts.soar_player_account.to_account_info(),
        game: ctx.accounts.soar_game.to_account_info(),
        leaderboard: ctx.accounts.soar_leaderboard.to_account_info(),
        player_scores: ctx.accounts.soar_player_scores.to_account_info(),
        top_entries: ctx.accounts.soar_top_entries.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
    };

    let state_bump = ctx.bumps.leaderboard_info;
    let seeds = &[LEADERBOARD, &[state_bump]];
    let signer = &[&seeds[..]];

    let cpi_ctx = CpiContext::new(
        ctx.accounts.soar_program.to_account_info(), accounts)
        .with_signer(signer);
    cpi::submit_score(cpi_ctx,ctx.accounts.user.won as u64)?;

    msg!("Submitting score {} for user.", ctx.accounts.user.won);

    // Calculate the reward (90% of the price pool, 10% is kept in the vault)
    let payout = ((game.ticket_price * game.players.len() as u64) * 9) / 10 ;

    msg!("Paying out {} lamports to {}", payout, receiver_account.key().to_string());

    // Transfer price to the winner
    let vault_lamports = ctx.accounts.vault.to_account_info().lamports().checked_sub(payout);
    let user_lamports = receiver_account.lamports().checked_add(payout);
    **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? = vault_lamports.ok_or(KamikazeJoeError::Overflow)?;
    **receiver_account.try_borrow_mut_lamports()? = user_lamports.ok_or(KamikazeJoeError::Overflow)?;

    Ok(())
}
