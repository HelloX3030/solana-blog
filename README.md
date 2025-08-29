# solana-blog

# Smart Contract

```rs
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
```

# Tests

```ts
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CommitLog } from "../target/types/commit_log";
import { assert } from "chai";

describe("commit-log", () => {
  // Use local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.CommitLog as Program<CommitLog>;

  it("Creates a commit entry!", async () => {
    const entryKeypair = anchor.web3.Keypair.generate();

    // Create the commit entry
    await program.methods
      .createEntry("First commit", "Hello blockchain!")
      .accounts({
        entry: entryKeypair.publicKey,
        user: provider.wallet.publicKey,
      })
      .signers([entryKeypair])
      .rpc();

    // Fetch and assert
    const account = await program.account.commitEntry.fetch(
      entryKeypair.publicKey
    );
    assert.equal(account.title, "First commit");
    assert.equal(account.description, "Hello blockchain!");
  });

  it("Updates a commit entry!", async () => {
    const entryKeypair = anchor.web3.Keypair.generate();

    // First, create the entry
    await program.methods
      .createEntry("Default Title", "Default Description")
      .accounts({
        entry: entryKeypair.publicKey,
        user: provider.wallet.publicKey,
      })
      .signers([entryKeypair])
      .rpc();

    // Update the title
    await program.methods
      .updateTitle("New Title")
      .accounts({
        entry: entryKeypair.publicKey,
      })
      .rpc();
    {
      const updatedAccount = await program.account.commitEntry.fetch(
        entryKeypair.publicKey
      );
      assert.equal(updatedAccount.title, "New Title");
    }

    // Update the Description
    await program.methods
      .updateDescription("New Description")
      .accounts({
        entry: entryKeypair.publicKey,
      })
      .rpc();
    {
      const updatedAccount = await program.account.commitEntry.fetch(
        entryKeypair.publicKey
      );
      assert.equal(updatedAccount.description, "New Description");
    }

    // Check Permissions
    const otherKeypair = anchor.web3.Keypair.generate();
    try {
      await program.methods
        .updateTitle("Other Title")
        .accounts({
          entry: entryKeypair.publicKey,
          user: otherKeypair.publicKey,
        })
        .signers([otherKeypair])
        .rpc();

      assert.fail("Expected updateTitle to fail but it succeeded");
    } catch (err: any) {
      assert.ok(
        err.message.includes("RequireKeysEqViolated") ||
          err.message.includes("require_keys_eq"),
        "Expected RequireKeysEqViolated error"
      );
      const accountData = await program.account.commitEntry.fetch(
        entryKeypair.publicKey
      );
      assert.equal(accountData.title, "New Title");
    }
    try {
      await program.methods
        .updateDescription("Other Description")
        .accounts({
          entry: entryKeypair.publicKey,
          user: otherKeypair.publicKey,
        })
        .signers([otherKeypair])
        .rpc();

      assert.fail("Expected updateDescription to fail but it succeeded");
    } catch (err: any) {
      assert.ok(
        err.message.includes("RequireKeysEqViolated") ||
          err.message.includes("require_keys_eq"),
        "Expected RequireKeysEqViolated error"
      );
      const accountData = await program.account.commitEntry.fetch(
        entryKeypair.publicKey
      );
      assert.equal(accountData.description, "New Description");
    }
  });

  it("Close Commit Entry!", async () => {
    const entryKeypair = anchor.web3.Keypair.generate();

    // Create Account
    await program.methods
      .createEntry("Closing Entry", "Closing Entry Description")
      .accounts({
        entry: entryKeypair.publicKey,
        user: provider.wallet.publicKey,
      })
      .signers([entryKeypair])
      .rpc();

    // Check Account
    {
      const accountData = await program.account.commitEntry.fetch(
        entryKeypair.publicKey
      );
      assert.equal(accountData.title, "Closing Entry");
      assert.equal(accountData.description, "Closing Entry Description");
    }

    // Check Permissions
    const otherKeypair = anchor.web3.Keypair.generate();
    try {
      await program.methods
        .closeEntry()
        .accounts({
          entry: entryKeypair.publicKey,
          user: otherKeypair.publicKey,
        })
        .signers([otherKeypair])
        .rpc();
      assert.fail("Expected closeEntry to fail but it succeeded");
    } catch (err: any) {
      assert.ok(
        err.message.includes("RequireKeysEqViolated") ||
          err.message.includes("require_keys_eq"),
        "Expected RequireKeysEqViolated error"
      );
      try {
        await program.account.commitEntry.fetch(entryKeypair.publicKey);
      } catch (err: any) {
        assert.fail(
          "Account does no longer exist, even with permission issues"
        );
      }
    }

    // Test Closing
    const beforeBalance = await provider.connection.getBalance(
      provider.wallet.publicKey
    );
    await program.methods
      .closeEntry()
      .accounts({
        entry: entryKeypair.publicKey,
        user: provider.wallet.publicKey,
      })
      .rpc();
    const afterBalance = await provider.connection.getBalance(
      provider.wallet.publicKey
    );
    assert.ok(
      afterBalance > beforeBalance,
      "User should receive refunded lamports"
    );

    // Check if Account still exists
    try {
      await program.account.commitEntry.fetch(entryKeypair.publicKey);
      assert.fail("Account should no longer exist");
    } catch (err: any) {
      assert.ok(
        err.message.includes("Account does not exist") ||
          err.message.includes("Failed to find account"),
        "Expected error for missing account"
      );
    }
  });
});
```
