use anchor_lang::prelude::*;
use session_keys::{Session, SessionToken};

declare_id!("JoeXD3mj5VXB2xKUz6jJ8D2AC72pXCydA6fnQJg2JiG");

mod instructions;
mod states;
mod errors;
mod seeds;
mod checks;

use instructions::*;
pub use states::*;

#[cfg(not(feature = "no-entrypoint"))]
solana_security_txt::security_txt! {
    name: "KamikazeJoe",
    project_url: "https://magicblock.gg",
    contacts: "email:dev@magicblock.gg,twitter:@magicblock",
    policy: "",
    preferred_languages: "en",
    source_code: "https://github.com/magicblock-labs/Kamikaze-Joe"
}

#[program]
pub mod kamikaze_joe {
    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        create_user::handler(ctx)
    }

    pub fn initialize_game(ctx: Context<InitializeGame>,
                           width: Option<u8>,
                           height: Option<u8>,
                           arena_seed: Option<u8>,
                           price_pool_lamports: Option<u64>) -> Result<()> {
        create_game::handler(ctx, width, height, arena_seed, price_pool_lamports)
    }

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        initialize::handler()
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

    pub fn claim_prize(ctx: Context<ClaimPrize>) -> Result<()> {
        claim_prize::handler(ctx)
    }

    pub fn claim_prize_soar(ctx: Context<ClaimPrizeSoar>) -> Result<()> {
        claim_prize_soar::handler(ctx)
    }

    pub fn initialize_leaderboard(ctx: Context<InitializeLeaderboard>,
                                  game: Pubkey,
                                  leaderboard: Pubkey,
                                  top_entries: Pubkey
    ) -> Result<()> {
        initialize_leaderboard::handler(ctx, game, leaderboard, top_entries)
    }

}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init, payer=payer, space = Matches::size(), seeds=[seeds::SEED_MATCHES], bump)]
    pub matches: Account<'info, Matches>,

    #[account(init, payer=payer, space = Vault::size(), seeds = [seeds::SEED_VAULT], bump)]
    pub vault: Account<'info, Vault>,

    pub system_program: Program<'info, System>,
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
    #[account(mut, address=User::pda(creator.key()).0)]
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
    #[account(mut, address=User::pda(player.key()).0)]
    pub user: Account<'info, User>,
    #[account(mut, address=Game::pda(User::pda(game.owner).0, &game.id.to_be_bytes()).0)]
    pub game: Account<'info, Game>,
    #[account(mut, address=Vault::pda().0)]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts, Session)]
pub struct MakeMove<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut, owner=crate::id())]
    pub user: Account<'info, User>,
    #[account(mut, address=Game::pda(User::pda(game.owner).0, &game.id.to_be_bytes()).0)]
    pub game: Account<'info, Game>,
    #[session(signer = payer, authority = user.authority)]
    pub session_token: Option<Account<'info, SessionToken>>,
}

#[derive(Accounts, Session)]
pub struct Explode<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut, owner=crate::id())]
    pub user: Account<'info, User>,
    #[account(mut, address=Game::pda(User::pda(game.owner).0, &game.id.to_be_bytes()).0)]
    pub game: Account<'info, Game>,
    #[session(signer = payer, authority = user.authority)]
    pub session_token: Option<Account<'info, SessionToken>>,
}

#[derive(Accounts)]
pub struct ClaimPrize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub receiver: Option<AccountInfo<'info>>,
    #[account(mut, address=User::pda(game.get_winner()).0)]
    pub user: Account<'info, User>,
    #[account(mut, address=Game::pda(User::pda(game.owner).0, &game.id.to_be_bytes()).0)]
    pub game: Account<'info, Game>,
    #[account(mut, address=Vault::pda().0)]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeLeaderboard<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(init, payer=payer, space = Leaderboard::size(), seeds = [seeds::LEADERBOARD], bump)]
    pub leaderboard: Account<'info, Leaderboard>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct ClaimPrizeSoar<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mut)]
    pub receiver: Option<AccountInfo<'info>>,
    #[account(mut, address=User::pda(game.get_winner()).0)]
    pub user: Account<'info, User>,
    #[account(mut, address=Game::pda(User::pda(game.owner).0, &game.id.to_be_bytes()).0)]
    pub game: Account<'info, Game>,
    #[account(mut, address=Vault::pda().0)]
    pub vault: Account<'info, Vault>,
    /// CHECK: The SOAR game account for this program.
    #[account(
        mut,
        seeds = [seeds::LEADERBOARD], bump,
        constraint = leaderboard_info.leaderboard == soar_leaderboard.key(),
        constraint = leaderboard_info.top_entries == soar_top_entries.key(),
    )]
    pub leaderboard_info: Account<'info, Leaderboard>,
    /// CHECK: The SOAR game for this program.
    pub soar_game: UncheckedAccount<'info>,
    /// CHECK: The SOAR leaderboard for this program.
    pub soar_leaderboard: UncheckedAccount<'info>,
    /// CHECK: The SOAR player account for this user.
    pub soar_player_account: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: The SOAR player scores account for this user.
    pub soar_player_scores: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: The SOAR top entries account for this leaderboard.
    pub soar_top_entries: UncheckedAccount<'info>,
    /// CHECK: The SOAR program ID.
    #[account(address = soar_cpi::ID)]
    pub soar_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}