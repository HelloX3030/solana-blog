use anchor_lang::prelude::*;

declare_id!("D4Svs9X2j7BQb6yusBV3Qyp51fXSmjBDBZsmr9dZhboZ");

#[program]
pub mod test2 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
