use anchor_lang::prelude::*;
use crate::{Game, InitializeGame};
use crate::errors::KamikazeJoeError;

pub fn handler(
    ctx: Context<InitializeGame>,
    width: Option<u8>,
    height: Option<u8>,
    arena_seed: Option<u8>,
    price_pool_lamports: Option<u64>,
) -> Result<()> {
    let game_account = &mut ctx.accounts.game;
    let mut game_object = Game::default();

    game_object.owner = *ctx.accounts.creator.unsigned_key();

    if let Some(w) = width { game_object.width = w; }
    if let Some(h) = height { game_object.height = h; }
    if let Some(s) = arena_seed { game_object.seed = s; }
    if let Some(p) = price_pool_lamports { game_object.ticket_price = p; }

    // Check if size is valid
    if game_object.width <= 0 && game_object.height <= 0 {
        return Err(KamikazeJoeError::InvalidSize.into());
    }

    game_account.set_inner(game_object);

    let user_account = &mut ctx.accounts.user;
    user_account.increment_games();

    let matches = &mut ctx.accounts.matches;
    if let Some(m) = matches{
        m.register_game(game_account.key());
    }

    Ok(())
}
