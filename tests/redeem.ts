import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Pdas } from "../target/types/pdas";

describe("Treasury", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Pdas as Program<Pdas>;

  it("Is redeemed", async () => {
    // 1. Load the user's wallet to their public key
    const wallet = anchor.Wallet.local();

    // 2. Find the wallet's treasury address (PDA) and bump
    const [treasuryAddress, bump] =
      anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("treasury"), wallet.publicKey.toBuffer()], // seeds
        program.programId
      );

    console.log("Using treasury address (PDA):", treasuryAddress.toBase58());
    console.log("Bump:", bump);

    // 3. Redeem 1 token on derived pda address to user wallet
    const tx = await program.methods
      .redeem(bump, new anchor.BN("1000000000"))
      .accounts({
        treasury: treasuryAddress,
        user: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("Your transaction signature:", tx);
  });
});
