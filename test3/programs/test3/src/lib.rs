use anchor_lang::prelude::*;

declare_id!("FjouJ8ACndtr7XGxWhmWdCtjwjAVV9yHd2SwMHE5V7cf");

#[program]
pub mod test3 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
