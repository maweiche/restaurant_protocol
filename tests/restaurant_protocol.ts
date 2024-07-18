import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { RestaurantProtocol } from "../target/types/restaurant_protocol";

import {
  PublicKey,
  Ed25519Program,
  SystemProgram,
  ComputeBudgetProgram,
  sendAndConfirmTransaction,
  Keypair,
  Transaction,
  Connection,
  GetProgramAccountsConfig,
  DataSizeFilter,
  MemcmpFilter,
  TransactionInstruction,
  VersionedTransaction,
  TransactionMessage,
  LAMPORTS_PER_SOL,
  SYSVAR_INSTRUCTIONS_PUBKEY,
  AddressLookupTableProgram,
  GetProgramAccountsFilter,
} from "@solana/web3.js";
import {
  getTokenMetadata,
} from "@solana/spl-token";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync, RawMint } from "@solana/spl-token";


describe("restaurant_protocol", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const wallet = anchor.Wallet.local();
  const _keypair = require('../test-wallet/keypair2.json')
  const _wallet = Keypair.fromSecretKey(Uint8Array.from(_keypair))
  console.log('local wallet', wallet.publicKey.toBase58());

  const buyer_keypair = require('../test-wallet/keypair.json')
  const buyer = Keypair.fromSecretKey(Uint8Array.from(buyer_keypair))
  console.log('buyer', buyer.publicKey.toBase58());

  // const collection_keypair = require('../test-wallet/keypair3.json')
  const collection_keypair = require('../test-wallet/keypair3.json')

  const collection_wallet = Keypair.fromSecretKey(Uint8Array.from(collection_keypair))
  console.log('collection_wallet', collection_wallet.publicKey.toBase58()); 


  const provider = anchor.getProvider();

  const program = anchor.workspace.RestaurantProtocol as Program<RestaurantProtocol>;


  const connection = new Connection("http://localhost:8899", "finalized"); // LOCALHOST

  // Helpers
  function wait(ms: number) {
    return new Promise( resolve => setTimeout(resolve, ms) );
  }

  const confirm = async (signature: string): Promise<string> => {
    const block = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      signature, 
      ...block
    })
    return signature
  }

  const log = async(signature: string): Promise<string> => {
    console.log(`Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`);
    return signature;
  }


  const id = Math.floor(Math.random() * 100000);

  const auth = PublicKey.findProgramAddressSync([Buffer.from('auth')], program.programId)[0];
  const adminState = PublicKey.findProgramAddressSync([Buffer.from('admin_state'), wallet.publicKey.toBuffer()], program.programId)[0];

  const RESTAURANT_OWNER = new PublicKey('2333')
  const restaurant = PublicKey.findProgramAddressSync([Buffer.from('restaurant'), RESTAURANT_OWNER.toBuffer()], program.programId)[0];
  const restaurant_mint = PublicKey.findProgramAddressSync([Buffer.from('mint'), restaurant.toBuffer()], program.programId)[0];

  const CUSTOMER = new PublicKey('2333')
  const customer_nft = PublicKey.findProgramAddressSync([Buffer.from('nft'), CUSTOMER.toBuffer()], program.programId)[0];
  const customer_nft_mint = PublicKey.findProgramAddressSync([Buffer.from('mint'), CUSTOMER.toBuffer()], program.programId)[0];

  // it("Protocol lock is initialized and set!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });

  // it("Protocol lock is toggled!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });

  // it("Protocol lock is toggled!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });

  // it("Restaurant is created!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });

  // it("Employee Added!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });

  // it("Employee clocked in!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });

  // it("Inventory Item added!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });

  // it("Customer placed first order and customer nft created!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });

  // it("Customer placed second order and customer nft updated!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });

  // it("Inventory Item updated!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });

  // it("Order Status updated!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });

  // it("Employee clocked out!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });

  // it("Employee got fired and removed!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });
  
  // it("Menu item removed!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });

  // it("Employee clocked out!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });

  // it("Menu item removed!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });

  // it("Restaurant has been closed!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });
});
