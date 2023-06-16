use anchor_lang::prelude::*;
use crate::seeds::SEED_USER;

#[account]
#[derive(InitSpace, Debug, Default)]
pub struct User {
    pub current_game: Option<Pubkey>,
    pub games: u64,
}

impl User {

    pub fn size() -> usize {
        8 + User::INIT_SPACE
    }

    pub fn pda(owner: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SEED_USER, owner.as_ref()], &crate::ID)
    }

    pub fn set_game(&mut self, game: Pubkey) {
        self.current_game = Some(game);
    }

    pub fn increment_games(&mut self) {
        self.games += 1;
    }

    pub fn in_game(&self) -> bool {
        self.current_game.is_some()
    }

    pub fn not_in_game(&self) -> bool {
        self.current_game.is_none()
    }

}