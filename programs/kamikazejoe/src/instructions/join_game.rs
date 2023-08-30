use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use crate::{Facing, GameState, JoinGame, Player};
use crate::errors::KamikazeJoeError;

pub fn handler(
    ctx: Context<JoinGame>, x: u8, y: u8
) -> Result<()> {

    let game_account = &mut ctx.accounts.game;

    if !game_account.is_cell_valid(x as usize, y as usize){
        return Err(KamikazeJoeError::InvalidJoin.into());
    }

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
    user_account.games += 1;

    if game_account.ticket_price > 0 {

        // Transfer to system owned account
        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.player.to_account_info(),
                    to: ctx.accounts.vault.to_account_info(),
                },
            ),
            game_account.ticket_price,
        )?;
    }

    msg!("Joined game");

    Ok(())
}
