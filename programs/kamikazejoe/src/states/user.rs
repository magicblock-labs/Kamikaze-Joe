use anchor_lang::prelude::*;
use crate::seeds::SEED_USER;

#[account]
#[derive(InitSpace, Debug, Default)]
pub struct User {
    pub authority: Pubkey,
    pub games: u32,
    pub won: u32,
}

impl User {

    pub fn size() -> usize {
        8 + User::INIT_SPACE
    }

    pub fn pda(owner: Pubkey) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SEED_USER, owner.as_ref()], &crate::ID)
    }

    pub fn increment_games(&mut self) {
        self.games += 1;
    }

}