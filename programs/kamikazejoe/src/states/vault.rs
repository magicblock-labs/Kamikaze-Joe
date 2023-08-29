use anchor_lang::prelude::*;
use crate::seeds::SEED_VAULT;

#[account]
#[derive(InitSpace, Debug, Default)]
pub struct Vault {
}


impl Vault {

    pub fn size() -> usize {
        8 + Vault::INIT_SPACE
    }

    pub fn pda() -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SEED_VAULT], &crate::ID)
    }

}