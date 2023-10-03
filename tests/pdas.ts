import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Pdas } from "../target/types/pdas";

describe("pdas", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Pdas as Program<Pdas>;

  // it("Is initialized!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });

  it("Is redeemed", async () => {
    const [vault, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("vault")],
      program.programId
    );

    console.log("vault", vault.toBase58());
    console.log("bump", bump);

    // Add your test here.
    const tx = await program.methods
      .redeem(bump, new anchor.BN("10000000"))
      .accounts({
        vault: vault,
        recipient: "EkHjnaeehtYAoDgxUXdgBtjsSgHtAHwbG4QyPZdWZiRt",
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
