use anchor_lang::prelude::*;

declare_id!("t31NQydRNTh8T2QWFmodA9j1HZxTK6EeAXjk7WWy8j1");

#[program]
pub mod test_hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
