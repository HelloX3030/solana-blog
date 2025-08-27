use anchor_lang::prelude::*;

declare_id!("ARJt1jkcVDbryjvgnEXjsR9Q8i84oxQSCq23nbynF5wK");

#[program]
pub mod commit_log {
    use super::*;

    pub fn create_entry(
        ctx: Context<CreateEntry>,
        title: String,
        description: String,
    ) -> Result<()> {
        let entry = &mut ctx.accounts.entry;
        entry.timestamp = Clock::get()?.unix_timestamp;
        entry.title = title;
        entry.description = description;
        Ok(())
    }

    pub fn update_title(ctx: Context<UpdateTitle>, new_title: String) -> Result<()> {
        let entry = &mut ctx.accounts.entry;
        entry.title = new_title;
        Ok(())
    }

    pub fn update_description(
        ctx: Context<UpdateDescription>,
        new_description: String,
    ) -> Result<()> {
        let entry = &mut ctx.accounts.entry;
        entry.description = new_description;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateEntry<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 8 + (4 + 64) + (4 + 256) // discriminator + timestamp + title + description
    )]
    pub entry: Account<'info, CommitEntry>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateTitle<'info> {
    #[account(mut)]
    pub entry: Account<'info, CommitEntry>,
}

#[derive(Accounts)]
pub struct UpdateDescription<'info> {
    #[account(mut)]
    pub entry: Account<'info, CommitEntry>,
}

// Account definition
#[account]
pub struct CommitEntry {
    pub timestamp: i64,
    pub title: String,
    pub description: String,
}
