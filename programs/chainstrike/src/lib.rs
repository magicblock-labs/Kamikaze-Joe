use anchor_lang::prelude::*;

declare_id!("F91oPUhkygpaR4KAazG1mXhQ6yYavh6LbQq46r2LKM6b");

mod instructions;
mod states;
mod errors;
mod seeds;

use instructions::*;
pub use states::*;

#[program]
pub mod chainstrike {
    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        create_user::handler(ctx)
    }

    pub fn initialize_game(ctx: Context<InitializeGame>) -> Result<()> {
        create_game::handler(ctx)
    }

    pub fn join_game(ctx: Context<JoinGame>, x: u8, y: u8) -> Result<()> {
        join_game::handler(ctx, x, y)
    }

    pub fn make_move(ctx: Context<MakeMove>, direction: Facing, energy: u8) -> Result<()> {
        make_move::handler(ctx, direction, energy)
    }
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init, payer=payer, space = User::size(), seeds=[seeds::SEED_USER, payer.key().as_ref()], bump)]
    pub user: Account<'info, User>,

    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct InitializeGame<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(mut,address=User::pda(creator.key()).0)]
    pub user: Account<'info, User>,
    #[account(init, payer = creator, space = Game::size(), seeds = [seeds::SEED_GAME, user.key().as_ref(), &user.games.to_be_bytes()], bump)]
    pub game: Account<'info, Game>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct JoinGame<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut)]
    pub user: Account<'info, User>,
    #[account(mut)]
    pub game: Account<'info, Game>,
}

#[derive(Accounts)]
pub struct MakeMove<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut)]
    pub game: Account<'info, Game>,
}
