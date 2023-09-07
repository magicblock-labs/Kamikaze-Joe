use anchor_lang::prelude::Account;
use anchor_lang::{Key, Result};
use anchor_lang::prelude::SolanaSysvar;
use gpl_session::{SessionError, SessionToken};
use solana_program::clock::Clock;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use crate::errors::KamikazeJoeError;

pub fn check_address(
    actual_address: &Pubkey,
    reference_address: &Pubkey,
    field_name: &str,
) -> ProgramResult {
    if actual_address == reference_address {
        Ok(())
    } else {
        msg!(
            "Invalid {} address: expected {} got {}",
            field_name,
            reference_address,
            actual_address
        );
        Err(ProgramError::InvalidArgument)
    }
}


pub fn check_session_token(
    local_session_token: Option<Account<SessionToken>>,
    local_payer: &Pubkey,
    authority: &Pubkey,
    target_program: &Pubkey,
) -> Result<()> {
    if let Some(session_token) = local_session_token {

        // Check if the authority in the session token matches the provided authority
        if !authority.eq(&session_token.authority) {
            return Err(SessionError::InvalidToken.into());
        }

        let seeds = &[
            SessionToken::SEED_PREFIX.as_bytes(),
            target_program.as_ref(),
            local_payer.as_ref(),
            session_token.authority.as_ref(),
        ];

        // Check if the derived address matches the session token's address
        let (pda, _) = Pubkey::find_program_address(seeds, &gpl_session::id());
        if pda != session_token.key() {
            return Err(SessionError::InvalidToken.into());
        }

        // Check if the session token is still valid
        let now = Clock::get()?.unix_timestamp;
        if now >= session_token.valid_until {
            return Err(SessionError::InvalidToken.into());
        }
    } else {
        // Check if authority matches local payer
        if authority != local_payer {
            return Err(KamikazeJoeError::InvalidUser.into());
        }
    }
    Ok(())
}

