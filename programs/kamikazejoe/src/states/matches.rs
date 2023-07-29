use anchor_lang::prelude::*;
use crate::seeds::SEED_MATCHES;

pub const MAX_GAMES: usize = 10;

#[account]
#[derive(InitSpace, Debug, Default)]
pub struct Matches {
    #[max_len(10)]
    pub active_games: Vec<Pubkey>,
}

impl Matches {

    pub fn size() -> usize {
        8 + Matches::INIT_SPACE
    }

    pub fn pda() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SEED_MATCHES], &crate::ID)
    }

    pub fn register_game(&mut self, game: Pubkey) {
        // Remove first game if we have MAX_GAMES
        if self.active_games.len() >= MAX_GAMES {
            self.active_games.drain(..1); // Efficiently remove the first game
        }
        self.active_games.push(game);
    }

}