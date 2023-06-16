import { PublicKey } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import {AnchorProvider, Program} from "@coral-xyz/anchor";
import {Chainstrike} from "../target/types/chainstrike";
import BN from "bn.js";

async function new_funded_address(provider: AnchorProvider) {
    let player = anchor.web3.Keypair.generate();//provider.wallet;

    const airdrop = await provider.connection.requestAirdrop(
        player.publicKey,
        anchor.web3.LAMPORTS_PER_SOL
    );

    await provider.connection.confirmTransaction(airdrop, "confirmed");
    return player;
}

async function InitializeUser(program: Program<Chainstrike>, player: anchor.web3.Keypair, userPda: PublicKey) {
    let tx = await program.methods
        .initializeUser()
        .accounts({
            payer: player.publicKey,
            user: userPda,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([player]).rpc()
    return tx;
}

async function InitializeGame(program: Program<Chainstrike>, player: anchor.web3.Keypair, userPda: PublicKey, gamePda: PublicKey) {
    return await program.methods
        .initializeGame()
        .accounts({
            creator: player.publicKey,
            user: userPda,
            game: gamePda,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([player]).rpc()
}

async function JoinGame(program: Program<Chainstrike>, player2: anchor.web3.Keypair, user2Pda: PublicKey, gamePda: PublicKey) {
    return await program.methods
        .joinGame(5, 5)
        .accounts({
            player: player2.publicKey,
            user: user2Pda,
            game: gamePda,
        })
        .signers([player2]).rpc()
}

function FindGamePda(userPda: PublicKey, id: BN, program: Program<Chainstrike>) {
    let gamePda = PublicKey.findProgramAddressSync(
        [Buffer.from("game"), userPda.toBuffer(), id.toBuffer("le", 8)],
        program.programId
    )[0];
    return gamePda;
}

function FindUserPda(player: PublicKey, program: Program<Chainstrike>) {
    let userPda = PublicKey.findProgramAddressSync(
        [Buffer.from("user"), player.toBuffer()],
        program.programId
    )[0];
    return userPda;
}

describe("chainstrike", () => {

    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.Chainstrike as Program<Chainstrike>;
    const maxId = 100000;

    it("Create User!", async () => {

        // First generate the account to initialize the game
        const provider = anchor.AnchorProvider.env();
        let player = await new_funded_address(provider);

        let userPda = FindUserPda(player.publicKey, program);

        // Initialize User.
        let tx = await InitializeUser(program, player, userPda);
        await provider.connection.confirmTransaction(tx, "confirmed");

        console.log("Create Game signature", tx);
    });

    it("Create Game!", async () => {
        const provider = anchor.AnchorProvider.env();
        let player = await new_funded_address(provider);

        let userPda = FindUserPda(player.publicKey, program);

        // Initialize User.
        let tx = await InitializeUser(program, player, userPda);
        console.log("Init User signature", tx);
        await provider.connection.confirmTransaction(tx, "confirmed");

        let id = new BN(0);
        let gamePda = FindGamePda(userPda, id, program);

        // Initialize Game.
        tx = await InitializeGame(program, player, userPda, gamePda);
        console.log("Create Game signature", tx);
        await provider.connection.confirmTransaction(tx, "confirmed");
    });

    it("Create and Join a Game! with two players", async () => {

        const provider = anchor.AnchorProvider.env();
        let player = await new_funded_address(provider);

        let userPda = FindUserPda(player.publicKey, program);

        // Initialize User.
        let tx = await InitializeUser(program, player, userPda);
        console.log("Init User signature", tx);
        await provider.connection.confirmTransaction(tx, "confirmed");

        let id = new BN(0);
        let gamePda = FindGamePda(userPda, id, program);

        // Initialize Game.
        tx = await InitializeGame(program, player, userPda, gamePda);
        console.log("Create Game signature", tx);
        await provider.connection.confirmTransaction(tx, "confirmed");

        // Join the game
        tx = await JoinGame(program, player, userPda, gamePda);


        console.log("Join Game signature", tx);
        await provider.connection.confirmTransaction(tx, "confirmed");

        // Join the game with a second player
        let player2 = await new_funded_address(provider);
        let user2Pda = FindUserPda(player2.publicKey, program);
        tx = await InitializeUser(program, player2, user2Pda);
        console.log("Init User 2 signature", tx);
        await provider.connection.confirmTransaction(tx, "confirmed");

        tx = await JoinGame(program, player2, user2Pda, gamePda);
        console.log("Join 2 Game signature", tx);
        await provider.connection.confirmTransaction(tx, "confirmed");
    });

    it("Create, Join a Game and Make a move", async () => {

        const provider = anchor.AnchorProvider.env();
        let player = await new_funded_address(provider);

        let userPda = FindUserPda(player.publicKey, program);

        // Initialize User.
        let tx = await InitializeUser(program, player, userPda);
        console.log("Init User signature", tx);
        await provider.connection.confirmTransaction(tx, "confirmed");

        let id = new BN(0);
        let gamePda = FindGamePda(userPda, id, program);

        // Initialize Game.
        tx = await InitializeGame(program, player, userPda, gamePda);
        console.log("Create Game signature", tx);
        await provider.connection.confirmTransaction(tx, "confirmed");

        // Join the game
        tx = await JoinGame(program, player, userPda, gamePda);
        console.log("Join Game signature", tx);
        await provider.connection.confirmTransaction(tx, "confirmed");

        // Make a move up
        tx = await program.methods
            .makeMove({up:{}}, 1)
            .accounts({
                player: player.publicKey,
                game: gamePda,
            }).signers([player]).rpc();
        console.log("Make move up signature", tx);
        await provider.connection.confirmTransaction(tx, "confirmed");

        // // Make a move right
        // tx = await program.methods
        //     .makeMove({right:{}}, 2)
        //     .accounts({
        //         player: player.publicKey,
        //         game: gamePda,
        //     }).signers([player]).rpc();
        // console.log("Make move rx signature", tx);
        // await provider.connection.confirmTransaction(tx, "confirmed");
        //
        // // Make a move right
        // tx = await program.methods
        //     .makeMove({up:{}}, 4)
        //     .accounts({
        //         player: player.publicKey,
        //         game: gamePda,
        //     }).signers([player]).rpc();
        // console.log("Make move up signature", tx);
        // await provider.connection.confirmTransaction(tx, "confirmed");

    });
});
