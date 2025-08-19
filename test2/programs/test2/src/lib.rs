use anchor_lang::prelude::*;

declare_id!("5E4x6MoMgMRoyqvTiaVDrYCdiFiM7TvyWiG9MqN8ePTr");

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
