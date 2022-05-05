import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { BlogDApp } from "../target/types/blog_d_app";

describe("BlogDApp", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BlogDApp as Program<BlogDApp>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
