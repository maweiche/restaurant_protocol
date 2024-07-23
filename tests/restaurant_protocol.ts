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
  const provider = anchor.getProvider();
  const program = anchor.workspace.RestaurantProtocol as Program<RestaurantProtocol>;

  const connection = new Connection("http://localhost:8899", "finalized"); // LOCALHOST

  /// WALLETS /////////////////////////////////////////////////////////////////////////////////////////////////////////////////
  const wallet = anchor.Wallet.local();
  const _keypair = require('../test-wallet/keypair2.json')
  const _wallet = Keypair.fromSecretKey(Uint8Array.from(_keypair))
  console.log('local wallet', wallet.publicKey.toBase58());

  const buyer_keypair = require('../test-wallet/keypair.json')
  const buyer = Keypair.fromSecretKey(Uint8Array.from(buyer_keypair))
  console.log('buyer', buyer.publicKey.toBase58());

  const collection_keypair = require('../test-wallet/keypair3.json')
  const collection_wallet = Keypair.fromSecretKey(Uint8Array.from(collection_keypair))
  console.log('collection_wallet', collection_wallet.publicKey.toBase58()); 

  const RESTAURANT_OWNER = new PublicKey('2333')
  const RESTAURANT_ADMIN = new PublicKey('2333')
  const EMPLOYEE = new PublicKey('2333')
  const CUSTOMER = new PublicKey('2333')

  // ACCOUNT ADDRESSES /////////////////////////////////////////////////////////////////////////////////////////////////////////////////
  const protocol = PublicKey.findProgramAddressSync([Buffer.from('protocol')], program.programId)[0];

  const auth = PublicKey.findProgramAddressSync([Buffer.from('auth')], program.programId)[0];
  const admin_state = PublicKey.findProgramAddressSync([Buffer.from('admin_state'), wallet.publicKey.toBuffer()], program.programId)[0];
  
  const restaurant = PublicKey.findProgramAddressSync([Buffer.from('restaurant'), RESTAURANT_OWNER.toBuffer()], program.programId)[0];
  const restaurant_admin_state = PublicKey.findProgramAddressSync([Buffer.from('admin'), RESTAURANT_ADMIN.toBuffer(), restaurant.toBuffer()], program.programId)[0];
  const restaurant_mint = PublicKey.findProgramAddressSync([Buffer.from('mint'), restaurant.toBuffer()], program.programId)[0];

  const employee_state = PublicKey.findProgramAddressSync([Buffer.from('employee'), EMPLOYEE.toBuffer(), restaurant.toBuffer()], program.programId)[0];

  const customer_nft = PublicKey.findProgramAddressSync([Buffer.from('nft'), CUSTOMER.toBuffer()], program.programId)[0];
  const customer_nft_mint = PublicKey.findProgramAddressSync([Buffer.from('mint'), CUSTOMER.toBuffer()], program.programId)[0];


  // REFERENCE GROUPS /////////////////////////////////////////////////////////////////////////////////////////////////////////////////
  const RESTAURANT_REFERENCE = new PublicKey('2333')


  // RESTAURANT DATA /////////////////////////////////////////////////////////////////////////////////////////////////////////////////
  const name = "NAME";
  const symbol = "SYM";
  const url = "URL";
  const restaurant_admin_username = "MATT";  // 5 characters MAX

  // EMPLOYEE DATA /////////////////////////////////////////////////////////////////////////////////////////////////////////////////
  const employee_username = "MATT";  // 5 characters MAX

  // Helpers /////////////////////////////////////////////////////////////////////////////////////////////////////////////////
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

  it("Protocol lock is initialized and set!", async () => {
    

    const transaction = new Transaction().add(
      await program.methods
      .initializeProtocolAccount()
      .accounts({
        admin: wallet.publicKey,
        protocol: protocol,
        systemProgram: SystemProgram.programId,
      })
      .instruction()
    );

    await sendAndConfirmTransaction(connection, transaction, [wallet.payer], {commitment: "finalized", skipPreflight: true}).then(confirm).then(log);
  });

  it("Protocol lock is toggled!", async () => {
    const transaction = new Transaction().add(
      await program.methods
      .lockProtocol()
      .accounts({
        admin: wallet.publicKey,
        protocol: protocol,
        systemProgram: SystemProgram.programId,
      })
      .instruction()
    );
    
    await sendAndConfirmTransaction(connection, transaction, [wallet.payer], {commitment: "finalized", skipPreflight: true}).then(confirm).then(log);
  });

  it("Initialize Protocol Admin", async () => {
    const username = "MATT";  // 5 characters MAX

    const createAdminIx = await program.methods
      .initializeAdminAccount(username)
      .accounts({
        admin: wallet.publicKey,
        adminState: null,
        newAdmin: wallet.publicKey,
        newAdminState: admin_state,
        protocol: protocol,
        systemProgram: SystemProgram.programId,
      })
      .instruction()

    const tx = new anchor.web3.Transaction().add(createAdminIx);
    await sendAndConfirmTransaction(connection, tx, [wallet.payer], {commitment: "finalized", skipPreflight: true}).then(confirm).then(log);
  });

  it("Restaurant is created!", async () => {
    const createRestaurantIx = await program.methods
      .addRestaurant(
        RESTAURANT_REFERENCE,
        name,
        symbol,
        url
      )
      .accounts({
        admin: wallet.publicKey,
        adminState: null,
        owner: RESTAURANT_OWNER,
        restaurant: restaurant,
        mint: restaurant_mint,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        protocol: protocol,
        token2022Program: TOKEN_2022_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .instruction()

    const tx = new anchor.web3.Transaction().add(createRestaurantIx);
    await sendAndConfirmTransaction(connection, tx, [wallet.payer], {commitment: "finalized", skipPreflight: true}).then(confirm).then(log);
  });

  it("Admin Added to Restaurant!", async () => {
    const createEmployeeIx = await program.methods
      .initializeRestaurantAdmin(
        restaurant_admin_username
      )
      .accounts({
        restaurant: restaurant,
        restaurantOwner: RESTAURANT_OWNER,
        restaurantAdmin: RESTAURANT_ADMIN,
        restaurantAdminState: restaurant_admin_state,
        protocol: protocol,
        systemProgram: SystemProgram.programId,
      })
      .instruction()

    const tx = new anchor.web3.Transaction().add(createEmployeeIx);
    await sendAndConfirmTransaction(connection, tx, [wallet.payer], {commitment: "finalized", skipPreflight: true}).then(confirm).then(log);
  });

  it("Employee Added to Restaurant!", async () => {
    const createEmployeeIx = await program.methods
      .initializeEmployeeAccount(
        employee_username
      )
      .accounts({
        restaurantAdmin: RESTAURANT_ADMIN,
        restaurantAdminState: restaurant_admin_state,
        employee: EMPLOYEE,
        employeeState: employee_state,
        restaurant: restaurant,
        protocol: protocol,
        systemProgram: SystemProgram.programId,
      })
      .instruction()

    const tx = new anchor.web3.Transaction().add(createEmployeeIx);
    await sendAndConfirmTransaction(connection, tx, [RESTAURANT_ADMIN], {commitment: "finalized", skipPreflight: true}).then(confirm).then(log);
  });

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
