use anchor_lang::prelude::*;

declare_id!("LJ5BVoPLqaAZ8VzxCcin3uEjKt3tXrPh6YJ7AfDyw6M");

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
