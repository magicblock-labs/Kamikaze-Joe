use anchor_lang::prelude::*;
use crate::{Facing, GameState, JoinGame, Player};
use crate::errors::KamikazeJoeError;

pub fn handler(
    ctx: Context<JoinGame>, x: u8, y: u8
) -> Result<()> {

    let game_account = &mut ctx.accounts.game;
    game_account.players.push(Player{
        x,
        y,
        energy: 100,
        address: *ctx.accounts.player.unsigned_key(),
        facing: Facing::Down
    });

    // Check if game_state is waiting
    if game_account.game_state == GameState::Waiting && game_account.players.len() > 1 {
        game_account.game_state = GameState::Active;
    }

    if !game_account.is_game_active(){
        return Err(KamikazeJoeError::GameEnded.into());
    }

    let user_account = &mut ctx.accounts.user;
    user_account.current_game = Some(game_account.key());

    msg!("Joined game");

    Ok(())
}
