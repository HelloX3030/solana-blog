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
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([entryKeypair])
      .rpc();

    // Fetch and assert
    const account = await program.account.commitEntry.fetch(entryKeypair.publicKey);
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
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([entryKeypair])
      .rpc();

    // Update the title
    await program.methods
      .updateTitle("New Title")
      .accounts({
        entry: entryKeypair.publicKey,
      }).rpc();
    {
		const updatedAccount = await program.account.commitEntry.fetch(entryKeypair.publicKey);
		assert.equal(updatedAccount.title, "New Title");
	}

	// Update the Description
	await program.methods
	  .updateDescription("New Description")
	  .accounts({
		entry: entryKeypair.publicKey,
	  }).rpc();
	{
		const updatedAccount = await program.account.commitEntry.fetch(entryKeypair.publicKey);
		assert.equal(updatedAccount.description, "New Description");
	}
  });
});
