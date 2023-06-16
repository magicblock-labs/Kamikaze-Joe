use anchor_lang::prelude::*;
use crate::Cell::Block;

#[account]
#[derive(InitSpace, Debug, Default)]
pub struct Game {
    pub created_at: i64,
    pub owner: Pubkey,
    pub grid: Grid,
    pub game_state: GameState,

    #[max_len(10)]
    pub players: Vec<Player>,
}

impl Game {

    pub fn size() -> usize {
        8 + Game::INIT_SPACE
    }

    pub fn is_cell_valid(&self, x: usize, y: usize) -> bool {
        return x < self.width() && y < self.height() && self.grid.cells[x][y] == Cell::Empty;
    }

    pub fn is_game_active(&self) -> bool {
        return self.game_state == GameState::Active;
    }

    pub fn width(&self) -> usize {
        return self.grid.cells.len()
    }

    pub fn height(&self) -> usize {
        return self.grid.cells[0].len()
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
    pub cells: [[Cell; 28]; 28],
}

impl Default for Grid {
    fn default() -> Self {
        let mut grid = Grid {
            cells: [[Cell::Empty; 28]; 28],
        };

        let row = &mut grid.cells[10];
        row.iter_mut().skip(5).take(5).for_each(|cell| *cell = Block);

        grid.cells[3][3] = Block;
        grid.cells[27][23] = Block;
        grid.cells[0][13] = Block;
        grid.cells[24][4] = Block;

        grid
    }
}


#[derive(InitSpace, Copy, Clone, Debug, AnchorSerialize, AnchorDeserialize, PartialEq)]
pub enum Cell {
    Empty,
    Block,
    Teleport,
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