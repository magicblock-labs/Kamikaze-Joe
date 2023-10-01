use anchor_lang::prelude::*;
use crate::seeds::LEADERBOARD;

#[account]
#[derive(InitSpace, Default, Debug)]
pub struct Leaderboard {
    pub game: Pubkey,
    pub leaderboard: Pubkey,
    pub top_entries: Pubkey,
}

impl Leaderboard {

    pub fn size() -> usize {
        8 + Leaderboard::INIT_SPACE
    }

    pub fn pda() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[LEADERBOARD], &crate::ID)
    }

}
