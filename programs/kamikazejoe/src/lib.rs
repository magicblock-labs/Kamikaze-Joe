use anchor_lang::prelude::*;

declare_id!("JoeXD3mj5VXB2xKUz6jJ8D2AC72pXCydA6fnQJg2JiG");

mod instructions;
mod states;
mod errors;
mod seeds;

use instructions::*;
pub use states::*;

#[program]
pub mod kamikaze_joe {
    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        create_user::handler(ctx)
    }

    pub fn initialize_game(ctx: Context<InitializeGame>) -> Result<()> {
        create_game::handler(ctx)
    }

    pub fn initialize_matches(ctx: Context<InitializeMatches>) -> Result<()> {
        create_matches::handler(ctx)
    }

    pub fn join_game(ctx: Context<JoinGame>, x: u8, y: u8) -> Result<()> {
        join_game::handler(ctx, x, y)
    }

    pub fn make_move(ctx: Context<MakeMove>, direction: Facing, energy: u8) -> Result<()> {
        make_move::handler(ctx, direction, energy)
    }

    pub fn explode(ctx: Context<Explode>) -> Result<()> {
        explode::handler(ctx)
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
pub struct InitializeMatches<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init, payer=payer, space = Matches::size(), seeds=[seeds::SEED_MATCHES], bump)]
    pub matches: Account<'info, Matches>,

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
    #[account(mut,address=Matches::pda().0)]
    pub matches: Option<Account<'info, Matches>>,
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

#[derive(Accounts)]
pub struct Explode<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut)]
    pub game: Account<'info, Game>,
}
