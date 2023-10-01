use anchor_lang::prelude::*;
use crate::{InitializeLeaderboard, Leaderboard};

pub fn handler(
    ctx: Context<InitializeLeaderboard>,
    game: Pubkey,
    leaderboard: Pubkey,
    top_entries: Pubkey,
) -> Result<()> {

    let leaderboard_account = &mut ctx.accounts.leaderboard;
    let mut leaderboard_object = Leaderboard::default();
    leaderboard_object.game = game;
    leaderboard_object.leaderboard = leaderboard;
    leaderboard_object.top_entries = top_entries;
    leaderboard_account.set_inner(leaderboard_object);
    Ok(())
}
