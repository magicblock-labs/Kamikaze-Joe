use anchor_lang::prelude::*;

#[error_code]
pub enum KamikazeJoeError {
    #[msg("Invalid Grid size")]
    InvalidSize,
    #[msg("Unable to join a game that ended")]
    GameEnded,
    #[msg("Player is not part of this game")]
    PlayerNotFound,
    #[msg("Energy is not a valid value")]
    NotValidEnergy,
    #[msg("Unable to move into a not empty cell")]
    MovingIntoNotEmptyCell,
    #[msg("This movement is not valid")]
    InvalidMovement,
    #[msg("This position is not valid for joining the game")]
    InvalidJoin,
    #[msg("Price can't be claimed")]
    InvalidClaim
}
