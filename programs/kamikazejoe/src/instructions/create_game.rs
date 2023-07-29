use anchor_lang::prelude::*;
use crate::{Game, InitializeGame};

pub fn handler(
    ctx: Context<InitializeGame>,
) -> Result<()> {

    //game_meta_input.check()?;

    let game_account = &mut ctx.accounts.game;
    let mut game_object = Game::default();

    game_object.owner = *ctx.accounts.creator.unsigned_key();

    game_account.set_inner(game_object);

    let user_account = &mut ctx.accounts.user;
    user_account.increment_games();

    let matches = &mut ctx.accounts.matches;
    if let Some(m) = matches{
        m.register_game(game_account.key());
    }

    Ok(())
}
