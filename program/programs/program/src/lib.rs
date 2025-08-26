use anchor_lang::prelude::*;

declare_id!("8mNm7ttyJtW2iwMSTdQp96qbR7UdomJD5FsJUjKCxj8Y");

#[program]
pub mod program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
