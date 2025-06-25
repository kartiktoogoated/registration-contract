import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RegistrationContract } from "../target/types/registration_contract";
import { assert } from "chai";

describe("registration-contract", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const program = anchor.workspace.RegistrationContract as Program<RegistrationContract>;

  const user = provider.wallet;

  it("Registers a new profile", async () => {
    const id = new anchor.BN(1);
    const name = "kartik";

    // Derive PDA
    const [registrationPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("registration"), user.publicKey.toBuffer()],
      program.programId
    );

    // Call register
    const tx = await program.methods
      .register(id, name)
      .accounts({
        registration: registrationPda,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      }as any)
      .rpc();

    console.log("✅ Register tx:", tx);

    // Fetch the account
    const account = await program.account.registration.fetch(registrationPda);

    assert.equal(account.id.toNumber(), id.toNumber());
    assert.equal(account.name, name);
    assert.equal(account.user.toBase58(), user.publicKey.toBase58());
    assert.ok(account.timestamp.toNumber() > 0);
  });
});
