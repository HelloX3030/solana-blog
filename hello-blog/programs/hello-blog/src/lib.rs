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
        entry.owner = ctx.accounts.user.key();
        Ok(())
    }

    pub fn update_title(ctx: Context<UpdateTitle>, new_title: String) -> Result<()> {
        let entry = &mut ctx.accounts.entry;
        require_keys_eq!(entry.owner, ctx.accounts.user.key());
        entry.title = new_title;
        Ok(())
    }

    pub fn update_description(
        ctx: Context<UpdateDescription>,
        new_description: String,
    ) -> Result<()> {
        let entry = &mut ctx.accounts.entry;
        require_keys_eq!(entry.owner, ctx.accounts.user.key());
        entry.description = new_description;
        Ok(())
    }

    pub fn close_entry(ctx: Context<CloseEntry>) -> Result<()> {
        require_keys_eq!(ctx.accounts.entry.owner, ctx.accounts.user.key());
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
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateDescription<'info> {
    #[account(mut)]
    pub entry: Account<'info, CommitEntry>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct CloseEntry<'info> {
    #[account(mut, close = user)]
    pub entry: Account<'info, CommitEntry>,

    #[account(mut)]
    pub user: Signer<'info>,
}

// Account definition
#[account]
pub struct CommitEntry {
    pub timestamp: i64,
    pub title: String,
    pub description: String,
    pub owner: Pubkey,
}
