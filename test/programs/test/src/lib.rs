use anchor_lang::prelude::*;

declare_id!("7jtP68SpPirgaZdnpwHq7GPczZ3s9zCMw4W8tc6DQ5LW");

#[program]
pub mod test {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
