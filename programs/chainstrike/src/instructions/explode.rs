use anchor_lang::prelude::*;
use crate::{Game, Explode};
use crate::errors::ChainstrikeError;

const ENERGY_TO_EXPLODE: u8 = 20;

pub fn handler(
    ctx: Context<Explode>,
) -> Result<()> {
    let player_key = *ctx.accounts.player.unsigned_key();

    // Check if game is active
    if !ctx.accounts.game.is_game_active() {
        return Err(ChainstrikeError::GameEnded.into());
    }

    // Find player in game_account Players Vec
    let mut player_index = 0;
    let mut player_found = false;
    for (index, player_object) in ctx.accounts.game.players.iter().enumerate() {
        if player_object.address == player_key {
            player_index = index;
            player_found = true;
            break;
        }
    }

    // Check if player is found
    if !player_found {
        return Err(ChainstrikeError::PlayerNotFound.into());
    }

    return explode(&mut ctx.accounts.game, player_index);
}

fn explode(game: &mut Account<Game>, player_index: usize) -> Result<()>  {

    // Check if energy is valid
    if game.players[player_index].energy <= 0 {
        return Err(ChainstrikeError::NotValidEnergy.into());
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
            }
        }
    }

    // Reduce energy
    if ENERGY_TO_EXPLODE > game.players[player_index].energy {
        game.players[player_index].energy = 0;
    }else {
        game.players[player_index].energy = game.players[player_index].energy - ENERGY_TO_EXPLODE
    }

    Ok(())
}