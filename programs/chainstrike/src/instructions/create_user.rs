use anchor_lang::prelude::*;
use crate::{InitializeUser, User};

pub fn handler(
    ctx: Context<InitializeUser>,
) -> Result<()> {

    let user_account = &mut ctx.accounts.user;
    let user_object = User::default();
    user_account.set_inner(user_object);

    Ok(())
}
