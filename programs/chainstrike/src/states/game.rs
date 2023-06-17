use anchor_lang::prelude::*;
use crate::Cell::Block;
use crate::Cell::Recharge;

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
        return x < self.width() && y < self.height() && (
            self.grid.cells[x][y] == Cell::Empty || self.grid.cells[x][y] == Cell::Recharge);
    }

    pub fn is_recharge(&self, x: usize, y: usize) -> bool {
        return self.grid.cells[x][y] == Cell::Recharge
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

        let cells_to_set = &[(4, 5), (5, 5), (6, 5), (7, 5)];
        for &(x, y) in cells_to_set {
            grid.cells[x][y] = Block;
        }

        let cells_to_set = &[(4, 23), (5, 23), (6, 23), (7, 23)];
        for &(x, y) in cells_to_set {
            grid.cells[x][y] = Block;
        }

        let cells_to_set = &[(13, 13), (14, 13), (15, 13), (16, 13)];
        for &(x, y) in cells_to_set {
            grid.cells[x][y] = Block;
        }

        grid.cells[3][3] = Block;
        grid.cells[24][23] = Block;
        grid.cells[24][22] = Block;

        grid.cells[0][13] = Block;
        grid.cells[24][4] = Block;

        grid.cells[1][14] = Recharge;
        grid.cells[26][14] = Recharge;

        grid
    }
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