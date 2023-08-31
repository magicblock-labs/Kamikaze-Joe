use anchor_lang::prelude::*;
use crate::{Game, Explode, User};
use crate::checks::check_session_token;
use crate::errors::KamikazeJoeError;
use crate::id;

const ENERGY_TO_EXPLODE: u8 = 20;

pub fn handler(
    ctx: Context<Explode>,
) -> Result<()> {

    let player_key = ctx.accounts.user.authority;

    if ctx.accounts.user.key() != User::pda(ctx.accounts.user.authority).0 {
        return Err(KamikazeJoeError::InvalidAuthority.into());
    }

    // Check session token
    check_session_token(
        ctx.accounts.session_token.clone(),
        ctx.accounts.payer.clone().key,
        &ctx.accounts.user.authority,
        &id(),
    )?;

    // Check if game is active
    if !ctx.accounts.game.is_game_active() {
        return Err(KamikazeJoeError::GameEnded.into());
    }

    // Find player in game_account Players Vec
    let player_index = match ctx.accounts.game.get_player_index(player_key) {
        Ok(value) => value,
        Err(error) => return Err(error),
    };

    return explode(&mut ctx.accounts.game, player_index);
}


fn explode(game: &mut Account<Game>, player_index: usize) -> Result<()>  {

    // Check if energy is valid
    if game.players[player_index].energy <= 0 {
        return Err(KamikazeJoeError::NotValidEnergy.into());
    }

    let x = game.players[player_index].x as i16;
    let y = game.players[player_index].y as i16;

    let cells_to_check = &[
        (x, y),
        (x + 1, y),
        (x - 1, y),
        (x, y + 1),
        (x, y - 1),
        (x + 1, y + 1),
        (x - 1, y - 1),
        (x - 1, y + 1),
        (x + 1, y - 1),
    ];

    let mut killed = false;

    for c in cells_to_check {
        let x = c.0;
        let y = c.1;

        if x < 0 || y < 0 || x >= game.width() as i16 || y >= game.height() as i16 {
            continue;
        }

        let x = x as u8;
        let y = y as u8;

        for (index, player_object) in game.players.iter_mut().enumerate() {
            if index == player_index {
                continue;
            }
            if player_object.x == x && player_object.y == y {
                player_object.energy = 0;
                killed = true;
            }
        }
    }

    // Reset energy if killed someone
    if killed == true {
        game.players[player_index].energy = 100;
    }

    // Reduce energy
    game.reduce_energy(player_index, ENERGY_TO_EXPLODE);

    // Check if game ended
    game.check_if_won(player_index);

    Ok(())
}