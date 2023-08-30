use anchor_lang::prelude::*;
use crate::errors::KamikazeJoeError;
use crate::seeds::SEED_GAME;

#[account]
#[derive(InitSpace, Debug)]
pub struct Game {
    pub id: u32,
    pub width: u8,
    pub height: u8,
    pub seed: u8,
    pub ticket_price: u64,
    pub prize_claimed: bool,
    pub owner: Pubkey,
    pub game_state: GameState,

    #[max_len(30)]
    pub players: Vec<Player>,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            id: 0,
            width: 30,
            height: 30,
            seed: 0,
            ticket_price: 100000000,
            prize_claimed: false,
            owner: Pubkey::default(),
            game_state: GameState::Waiting,
            players: vec![],
        }
    }
}

impl Game {

    pub fn size() -> usize {
        8 + Game::INIT_SPACE
    }

    pub fn pda(user: Pubkey, id: &[u8]) -> (Pubkey, u8) {
        Pubkey::find_program_address(&[SEED_GAME, user.as_ref(), id], &crate::ID)
    }

    pub fn width(&self) -> usize {
        return self.width as usize
    }

    pub fn height(&self) -> usize {
        return self.height as usize
    }

    pub fn is_cell_valid(&self, x: usize, y: usize) -> bool {
        return x < self.width as usize && y < self.height as usize && (self.is_recharge(x, y) || !self.is_block(x, y));
    }

    pub fn is_recharge(&self, x: usize, y: usize) -> bool {
        let shift = self.seed % 14;
        let x_plus_shift = x as i64 + shift as i64;
        let y_minus_shift = y as i64 - shift as i64;

        let x_mod_13 = x_plus_shift % 13;
        let y_mod_14 = y_minus_shift % 14;
        let x_mod_28 = x_plus_shift % 28;
        let y_mod_28 = y_minus_shift % 28;

        x_mod_13 == y_mod_14
            && (x_mod_28 == 27 || x_plus_shift == 1)
            && x_plus_shift != y_minus_shift
            && x_mod_28 - y_mod_28 < 15
    }

    pub fn is_block(&self, x: usize, y: usize) -> bool {
        let len = (4 + self.seed % 6) as usize;
        let x_mod_28 = x % 28;
        let y_mod_28 = y % 28;

        if (y_mod_28 == 5 && x_mod_28 > 3 && x_mod_28 < 3 + len)
            || (y_mod_28 == 23 && x_mod_28 > 7 && x_mod_28 < 7 + std::cmp::max(5, len))
            || (y_mod_28 == 12 && x_mod_28 > 12 && x_mod_28 < 12 + len)
            || (x_mod_28 == 19 && y_mod_28 > 12 && y_mod_28 < 12 + std::cmp::max(5, len))
        {
            return true;
        }

        let x_squared_plus_y = x * x + x * y;
        let y_squared = y * y;
        let divisor = (47 % 60) - (self.seed % 59);
        let remainder = (x_squared_plus_y + y_squared + self.seed as usize) % divisor as usize;

        return remainder == 7;
    }

    pub fn is_game_active(&self) -> bool {
        return self.game_state == GameState::Waiting || self.game_state == GameState::Active;
    }

    pub fn can_claim(&self, player: &Pubkey ) -> bool {
        return self.game_state == GameState::Won { winner: *player }
            && self.prize_claimed == false
            && self.ticket_price > 0;
    }

    pub fn get_player_index(&self, player_key: Pubkey) -> Result<usize> {
        let mut player_index = 0;
        let mut player_found = false;
        for (index, player_object) in self.players.iter().enumerate() {
            if player_object.address == player_key {
                player_index = index;
                player_found = true;
                break;
            }
        }
        if !player_found {
            return Err(KamikazeJoeError::PlayerNotFound.into());
        }
        Ok(player_index)
    }

    // Mutable methods

    pub fn reduce_energy(&mut self, player_index: usize, energy: u8) {
        if energy > self.players[player_index].energy {
            self.players[player_index].energy = 0;
        }else {
            self.players[player_index].energy = self.players[player_index].energy - energy
        }
    }

    pub fn check_if_won(&mut self, player_index: usize) {
        if let GameState::Active = self.game_state {
            let target_address = self.players[player_index].address;
            let mut all_players_match = true;

            for player in &self.players {
                if player.energy > 0 && player.address != target_address {
                    all_players_match = false;
                    break;
                }
            }

            if all_players_match {
                self.game_state = GameState::Won {
                    winner: target_address,
                };
            }
        }
    }
}

#[derive(InitSpace, Debug, AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GameState {
    Waiting,
    Active,
    Won { winner: Pubkey },
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Waiting
    }
}

#[derive(InitSpace, Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct Grid {
    pub cells: [[Cell; 30]; 30],
}


#[derive(InitSpace, Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum Cell {
    Empty,
    Block,
    Recharge,
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Empty
    }
}


#[derive(InitSpace, Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct Player {
    pub x: u8,
    pub y: u8,
    pub energy: u8,
    pub address: Pubkey,
    pub facing: Facing,
}

impl Default for Player {
    fn default() -> Self {
        Player{
            x: 0,
            y: 0,
            energy: 100,
            address: Pubkey::default(),
            facing: Facing::Down
        }
    }
}

#[derive(InitSpace, Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum Facing {
    Up,
    Down,
    Left,
    Right
}

impl Default for Facing {
    fn default() -> Self {
        Facing::Down
    }
}