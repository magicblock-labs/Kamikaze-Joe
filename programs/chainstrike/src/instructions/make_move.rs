use anchor_lang::prelude::*;
use crate::{Facing, Game, MakeMove};
use crate::errors::ChainstrikeError;

pub fn handler(
    ctx: Context<MakeMove>,
    direction: Facing,
    energy: u8,
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

    // Check if energy is valid
    if energy > 5 {
        return Err(ChainstrikeError::NotValidEnergy.into());
    }

    return move_player(&mut ctx.accounts.game, player_index, direction, energy);
}

fn move_player(game: &mut Account<Game>, player_index: usize, direction: Facing, energy: u8) -> Result<()>  {

    // Check if energy is valid
    if game.players[player_index].energy <= 0 {
        return Err(ChainstrikeError::NotValidEnergy.into());
    }

    let mut final_x = game.players[player_index].x;
    let mut final_y = game.players[player_index].y;
    let mut is_valid = false;

    // Movement loop
    for _ in 0..energy {

        let x: u8;
        let y: u8;

        match direction {
            Facing::Down => {
                if final_y == 0 {
                    break;
                }
                x = final_x;
                y = final_y - 1;
            },
            Facing::Up => {
                x = final_x;
                y = final_y + 1;
            },
            Facing::Right => {
                x = final_x + 1;
                y = final_y;
            },
            Facing::Left => {
                if final_x == 0 {
                    break
                }
                x = final_x - 1;
                y = final_y;
            },
        };

        msg!(&format!("Try moving to {x}, {y}"));

        // Check if movement is valid
        if game.is_cell_valid(x as usize, y as usize) {
            final_x = x;
            final_y = y;
            is_valid = true;
            if game.is_recharge(x as usize, y as usize){
                game.players[player_index].energy = 100;
            }
        }else {
            break;
        }
    }

    if !is_valid {
        return Err(ChainstrikeError::InvalidMovement.into());
    }

    // Move player
    game.players[player_index].x = final_x;
    game.players[player_index].y = final_y;
    game.players[player_index].facing = direction;

    if energy > game.players[player_index].energy {
        game.players[player_index].energy = 0;
    }else {
        game.players[player_index].energy = game.players[player_index].energy - energy
    }

    msg!(&format!("Moved to {final_x}, {final_y}"));

    Ok(())
}