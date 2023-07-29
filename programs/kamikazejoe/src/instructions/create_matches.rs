use anchor_lang::prelude::*;
use crate::{InitializeMatches, Matches};

pub fn handler(
    ctx: Context<InitializeMatches>,
) -> Result<()> {

    let matches_account = &mut ctx.accounts.matches;
    let match_object = Matches::default();
    matches_account.set_inner(match_object);

    Ok(())
}
