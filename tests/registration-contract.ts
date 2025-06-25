import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RegistrationContract } from "../target/types/registration_contract";
import { assert } from "chai";

describe("registration-contract", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const program = anchor.workspace.RegistrationContract as Program<RegistrationContract>;

  const user = provider.wallet;

  it("Initializes a profile and validator", async () => {
    const name = "kartik";
    const validatorName = "validator-kartik";
    const id = new anchor.BN(1);

    // Derive profile PDA
    const [profilePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("profile"), user.publicKey.toBuffer()],
      program.programId
    );

    // Step 1: Init Profile
    const tx1 = await program.methods
      .initProfile(name)
      .accounts({
        profile: profilePda,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .rpc();

    console.log("✅ init_profile tx:", tx1);

    // Step 2: Derive Validator PDA using ID
    const [validatorPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("validator"), user.publicKey.toBuffer(), id.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    // Step 3: Init Validator
    const tx2 = await program.methods
      .initValidator(id, validatorName)
      .accounts({
        validator: validatorPda,
        authority: user.publicKey,
        profile: profilePda,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .rpc();

    console.log("✅ init_validator tx:", tx2);

    // Step 4: Fetch and assert
    const validatorAccount = await program.account.validatorInfo.fetch(validatorPda);

    assert.equal(validatorAccount.id.toNumber(), id.toNumber());
    assert.equal(validatorAccount.name, validatorName);
    assert.ok(validatorAccount.isActive);
    assert.equal(validatorAccount.authority.toBase58(), user.publicKey.toBase58());
    assert.equal(validatorAccount.profile.toBase58(), profilePda.toBase58());
    assert.ok(typeof validatorAccount.bump === "number");
  });
});
