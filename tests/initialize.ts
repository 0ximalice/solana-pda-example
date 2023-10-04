import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Pdas } from "../target/types/pdas";

describe("Treasury", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.Pdas as Program<Pdas>;

  it("Is initialized", async () => {
    const wallet = anchor.Wallet.local();
    const tx = await program.methods
      .initialize()
      .accounts({
        user: wallet.publicKey,
      })
      .rpc();

    console.log("Using wallet:", wallet.publicKey.toBase58());
    console.log("Program ID:", program.programId.toBase58());
    console.log("Your transaction signature:", tx);
  });
});
