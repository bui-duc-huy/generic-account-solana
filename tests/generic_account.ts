import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { GenericAccount } from "../target/types/generic_account";

describe("generic_account", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.GenericAccount as Program<GenericAccount>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
