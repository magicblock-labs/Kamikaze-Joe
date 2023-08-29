import { PublicKey } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import {AnchorProvider, Program, web3} from "@coral-xyz/anchor";
import {KamikazeJoe} from "../target/types/kamikaze_joe";
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

async function InitializeUser(program: Program<KamikazeJoe>, player: anchor.web3.Keypair, userPda: PublicKey) {
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

async function Initialize(program: Program<KamikazeJoe>, player: anchor.web3.Keypair, matchesPda: PublicKey) {
    let tx = await program.methods
        .initialize()
        .accounts({
            payer: player.publicKey,
            matches: matchesPda,
            vault: FindVaultPda(program),
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([player]).rpc()
    return tx;
}

async function InitializeGame(program: Program<KamikazeJoe>, player: anchor.web3.Keypair, userPda: PublicKey, gamePda: PublicKey) {
    return await program.methods
        .initializeGame(50, 50, 0, null)
        .accounts({
            creator: player.publicKey,
            user: userPda,
            game: gamePda,
            matches: FindMatchesPda(program),
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([player]).rpc()
}

async function JoinGame(program: Program<KamikazeJoe>, player2: anchor.web3.Keypair, user2Pda: PublicKey, gamePda: PublicKey, x= 0, y = 0) {
    return await program.methods
        .joinGame(x, y)
        .accounts({
            player: player2.publicKey,
            user: user2Pda,
            game: gamePda,
            vault: FindVaultPda(program),
        })
        .signers([player2]).rpc()
}

function FindGamePda(userPda: PublicKey, id: BN, program: Program<KamikazeJoe>) {
    let gamePda = PublicKey.findProgramAddressSync(
        [Buffer.from("game"), userPda.toBuffer(), id.toBuffer("le", 8)],
        program.programId
    )[0];
    return gamePda;
}

function FindUserPda(player: PublicKey, program: Program<KamikazeJoe>) {
    let userPda = PublicKey.findProgramAddressSync(
        [Buffer.from("userPda"), player.toBuffer()],
        program.programId
    )[0];
    return userPda;
}

function FindMatchesPda(program: Program<KamikazeJoe>) {
    let matchesPda = PublicKey.findProgramAddressSync(
        [Buffer.from("matches")],
        program.programId
    )[0];
    return matchesPda;
}

function FindVaultPda(program: Program<KamikazeJoe>) {
    let vaultPda = PublicKey.findProgramAddressSync(
        [Buffer.from("vault")],
        program.programId
    )[0];
    return vaultPda;
}

describe("kamikaze_joe", () => {

    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.KamikazeJoe as Program<KamikazeJoe>;
    const maxId = 100000;

    it("Create User!", async () => {

        // First generate the account to initialize the game
        const provider = anchor.AnchorProvider.env();
        let player = await new_funded_address(provider);

        let userPda = FindUserPda(player.publicKey, program);

        // Initialize User.
        let tx = await InitializeUser(program, player, userPda);
        await provider.connection.confirmTransaction(tx, "confirmed");

        console.log("Create User signature", tx);
    });

    it("Create Matches!", async () => {

        // First generate the account to initialize the game
        const provider = anchor.AnchorProvider.env();
        let player = await new_funded_address(provider);

        let matchesPda = FindMatchesPda(program);

        // Initialize if needed
        if(await provider.connection.getAccountInfo(matchesPda) == null) {
            let tx = await Initialize(program, player, matchesPda);
            await provider.connection.confirmTransaction(tx, "confirmed");
            console.log("Initialize signature", tx);
        }else{
            console.log("Already initialized");
        }

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
            .makeMove({up:{}}, 3)
            .accounts({
                player: player.publicKey,
                game: gamePda,
            }).signers([player]).rpc();
        console.log("Make move up signature", tx);
        await provider.connection.confirmTransaction(tx, "confirmed");

        // Make a move right
        tx = await program.methods
            .makeMove({right:{}}, 2)
            .accounts({
                player: player.publicKey,
                game: gamePda,
            }).signers([player]).rpc();
        console.log("Make move rx signature", tx);
        await provider.connection.confirmTransaction(tx, "confirmed");

        // Make a move right
        tx = await program.methods
            .makeMove({up:{}}, 4)
            .accounts({
                player: player.publicKey,
                game: gamePda,
            }).signers([player]).rpc();
        console.log("Make move up signature", tx);
        await provider.connection.confirmTransaction(tx, "confirmed");

    });

    it("Create and Join and Explode", async () => {

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
        tx = await JoinGame(program, player, userPda, gamePda, 2,2);

        console.log("Join Game signature", tx);
        await provider.connection.confirmTransaction(tx, "confirmed");

        // Join the game with a second player
        let player2 = await new_funded_address(provider);
        let user2Pda = FindUserPda(player2.publicKey, program);
        tx = await InitializeUser(program, player2, user2Pda);
        console.log("Init User 2 signature", tx);
        await provider.connection.confirmTransaction(tx, "confirmed");

        tx = await JoinGame(program, player2, user2Pda, gamePda, 2, 3);
        console.log("Join 2 Game signature", tx);
        await provider.connection.confirmTransaction(tx, "confirmed");

        // Join the game with a third player
        let player3 = await new_funded_address(provider);
        let user3Pda = FindUserPda(player3.publicKey, program);
        tx = await InitializeUser(program, player3, user3Pda);
        console.log("Init User 3 signature", tx);
        await provider.connection.confirmTransaction(tx, "confirmed");

        // Explode transaction
        tx = await program.methods
            .explode()
            .accounts({
                player: player.publicKey,
                game: gamePda,
            }).signers([player]).rpc();

        console.log("Explode signature", tx);
        await provider.connection.confirmTransaction(tx, "confirmed");
    });

    it("Win and claim", async () => {

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
        tx = await JoinGame(program, player, userPda, gamePda, 2,2);

        console.log("Join Game signature", tx);
        await provider.connection.confirmTransaction(tx, "confirmed");

        // Join the game with a second player
        let player2 = await new_funded_address(provider);
        let user2Pda = FindUserPda(player2.publicKey, program);
        tx = await InitializeUser(program, player2, user2Pda);
        console.log("Init User 2 signature", tx);
        await provider.connection.confirmTransaction(tx, "confirmed");

        tx = await JoinGame(program, player2, user2Pda, gamePda, 2, 3);
        console.log("Join 2 Game signature", tx);
        await provider.connection.confirmTransaction(tx, "confirmed");

        // Join the game with a third player
        let player3 = await new_funded_address(provider);
        let user3Pda = FindUserPda(player3.publicKey, program);
        tx = await InitializeUser(program, player3, user3Pda);
        console.log("Init User 3 signature", tx);
        await provider.connection.confirmTransaction(tx, "confirmed");

        // Explode transaction
        tx = await program.methods
            .explode()
            .accounts({
                player: player.publicKey,
                game: gamePda,
            }).signers([player]).rpc();

        console.log("Explode signature", tx);
        await provider.connection.confirmTransaction(tx, "confirmed");

        // Claim Price transaction
        tx = await program.methods
            .claimPrize()
            .accounts({
                player: player.publicKey,
                user: userPda,
                game: gamePda,
                vault: FindVaultPda(program),
            })
            .signers([player]).rpc()

        console.log("Claim price signature", tx);
        await provider.connection.confirmTransaction(tx, "confirmed");
    });

});
