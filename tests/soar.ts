import {Keypair, PublicKey} from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import {AnchorProvider, Program} from "@coral-xyz/anchor";
import {KamikazeJoe} from "../target/types/kamikaze_joe";
import BN from "bn.js";
import {expect} from "chai";
import {AccountsBuilder, GameClient, GameType, Genre, SoarProgram} from "@magicblock-labs/soar-sdk";
import {
    FindGamePda,
    FindUserPda,
    FindVaultPda,
    InitializeGame,
    InitializeUser,
    JoinGame,
    new_funded_address
} from "./kamikazejoe";


async function fund_address(connection: anchor.web3.Connection, account: PublicKey) {
    const airdrop = await connection.requestAirdrop(
        account,
        anchor.web3.LAMPORTS_PER_SOL
    );

    await connection.confirmTransaction(airdrop, "confirmed");
    return account;
}

export function FindLeaderboardPda(program: Program<KamikazeJoe>) {
    let leaderboardPda = PublicKey.findProgramAddressSync(
        [Buffer.from("soar")],
        program.programId
    )[0];
    return leaderboardPda;
}

async function InitializeLeaderboard(program: Program<KamikazeJoe>, payer: anchor.web3.Keypair, game: PublicKey, leaderboard: PublicKey, topEntries: PublicKey) {
    return await program.methods
        .initializeLeaderboard(game, leaderboard, topEntries)
        .accounts({
            payer: payer.publicKey,
            leaderboard: FindLeaderboardPda(program),
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([payer]).rpc({skipPreflight: true});
}

describe("kamikaze_joe_soar", () => {

    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());
    const provider = anchor.getProvider();

    const kamikazeJoeProgramId = new PublicKey(
        "JoeXD3mj5VXB2xKUz6jJ8D2AC72pXCydA6fnQJg2JiG"
    );
    const kamikazeJoeProgram = anchor.workspace.KamikazeJoe as Program<KamikazeJoe>;

    let authority = Keypair.generate();
    const client = SoarProgram.get(provider as AnchorProvider);
    let auths = [authority.publicKey, new PublicKey("JoeXD3mj5VXB2xKUz6jJ8D2AC72pXCydA6fnQJg2JiG")];
    let gameClient: GameClient;
    let game = Keypair.generate();
    let leaderBoards: PublicKey[] = [];
    let achievements: PublicKey[] = [];
    let user = Keypair.generate();

    it("Register the game", async () => {
        await fund_address(provider.connection, authority.publicKey);
        let title = "Kamikaze Joe";
        let description = "PvP Survival Arena";
        let genre = Genre.Action;
        let gameType = GameType.Web;
        let nftMeta = PublicKey.default;
        let _auths = auths.map((keypair) => keypair);

        let {
            newGame,
            transaction
        } = await client.initializeNewGame(game.publicKey, title, description, genre, gameType, nftMeta, _auths);
        await client.sendAndConfirmTransaction(transaction, [game]);

        let info = await client.fetchGameAccount(newGame);

        expect(info.meta.title).to.equal(title);
        expect(info.meta.description).to.equal(description);
        expect(info.meta.genre).to.equal(genre);
        expect(info.meta.gameType).to.equal(gameType);
        expect(info.meta.nftMeta.toBase58()).to.equal(nftMeta.toBase58());
        expect(info.leaderboardCount.toNumber()).to.equal(0);

        expect(info.auth.length).to.equal(2);
        expect(info.auth[0].toBase58()).to.equal(auths[0].toBase58());
        expect(info.auth[1].toBase58()).to.equal(auths[1].toBase58());

        gameClient = new GameClient(client, newGame);
    });

    it("Create the leaderboard", async () => {
        let expectedDescription = "Kamnikaze Joe Leaderboard";
        let expectedNftMeta = PublicKey.default;
        let scoresToRetain = 5;
        let scoresOrder = false; //descending order
        let decimals = 0;
        let minScore = new BN(0);
        let maxScore = new BN(100);
        let {newLeaderBoard, topEntries, transaction} =
            await gameClient.addLeaderBoard(
                authority.publicKey,
                expectedDescription,
                expectedNftMeta,
                scoresToRetain,
                scoresOrder,
                decimals,
                minScore,
                maxScore
            );

        await client.sendAndConfirmTransaction(transaction, [authority]);
        leaderBoards.push(newLeaderBoard);

        let info = await gameClient.program.fetchLeaderBoardAccount(newLeaderBoard);

        expect(info.id.toNumber()).to.equal(1);
        expect(info.game.toBase58()).to.equal(gameClient.address.toBase58());
        expect(info.description).to.equal(expectedDescription);
        expect(info.nftMeta.toBase58()).to.equal(expectedNftMeta.toBase58());
        expect(info.decimals).to.equal(0);
        expect(info.minScore.toNumber()).to.equal(minScore.toNumber());
        expect(info.maxScore.toNumber()).to.equal(maxScore.toNumber());
        expect(info.topEntries.toBase58()).to.equal(topEntries.toBase58());

        let entries = await gameClient.program.fetchLeaderBoardTopEntriesAccount(
            topEntries
        );

        expect(entries.isAscending).to.be.false;
        expect(entries.topScores.length).to.equal(scoresToRetain);
        for (let score of entries.topScores) {
            expect(score.entry.score.toNumber()).to.equal(0);
            expect(score.entry.timestamp.toNumber()).to.equal(0);
            expect(score.player.toBase58()).to.equal(PublicKey.default.toBase58());
        }

        await gameClient.refresh();
        expect(gameClient.account.leaderboardCount.toNumber()).to.equal(1);
    });

    it("Register player/leaderboard", async () => {
        // Derive the soarStatePDA of the `tens` program.
        let soarPDA = PublicKey.findProgramAddressSync(
            [Buffer.from("soar")],
            kamikazeJoeProgram.programId
        )[0];

        // Make the leaderboard info PDA an authority of the game so it can permissionlessly
        // sign CPI requests to SOAR that require the authority's signature.
        let newAuths = auths.concat([soarPDA]);
        auths = newAuths;

        let { transaction: update } = await gameClient.program.updateGameAccount(
            game.publicKey,
            authority.publicKey,
            undefined,
            newAuths
        );
        await client.sendAndConfirmTransaction(update, [authority]);

        // Initialize a SOAR player account, required for interacting with the `KamikazeJoe` game.
        let { transaction: initPlayer } = await gameClient.program.initializePlayerAccount(
            user.publicKey,
            "player1",
            PublicKey.default
        );
        const txp = await client.sendAndConfirmTransaction(initPlayer, [user]);
        console.log(`Initialize player account: ${txp}\n`);
    });

    it("Register the leaderboard info into Kamikaze Joe", async () => {

        // First generate the account to initialize the game
        const provider = anchor.AnchorProvider.env();
        let payer = await new_funded_address(provider);

        const leaderboardPda = FindLeaderboardPda(kamikazeJoeProgram);

        const accounts = await new AccountsBuilder(
            provider,
            SoarProgram.get(provider).program.programId
        ).submitScoreAccounts(user.publicKey, leaderboardPda, leaderBoards[0]);

        if(await provider.connection.getAccountInfo(leaderboardPda) == null) {
            let tx = await InitializeLeaderboard(kamikazeJoeProgram, payer, game.publicKey, accounts.leaderboard, accounts.topEntries);
            await provider.connection.confirmTransaction(tx, "confirmed");
            console.log("Register Game/Leaderboard signature", tx);
        }else{
            console.log("Already initialized: ", leaderboardPda.toString());
        }
    });

    it("Register the player to the leaderboard, if not already", async () => {
        // First generate the account to initialize the game
        const provider = anchor.AnchorProvider.env();
        const leaderboardPda = FindLeaderboardPda(kamikazeJoeProgram);
        const leaderboardInfo
            = await kamikazeJoeProgram.account.leaderboard.fetch(leaderboardPda)

        const accounts = await new AccountsBuilder(
            provider,
            SoarProgram.get(provider).program.programId
        ).submitScoreAccounts(user.publicKey, leaderboardPda, leaderboardInfo.leaderboard, leaderboardInfo.game);

        if(await provider.connection.getAccountInfo(accounts.playerScores) == null) {
            // Register the player to the leaderboard.
            let { newList: playerScoresList, transaction: regPlayer } =
                await gameClient.program.registerPlayerEntryForLeaderBoard(
                    user.publicKey,
                    leaderboardInfo.leaderboard
                );
            const tx = await client.sendAndConfirmTransaction(regPlayer, [user]);
            console.log(`Registered player/leaderboard tx: ${tx}\n`);
        }else{
            console.log("Already registered: ", accounts.playerScores.toString());
        }
    });

    it("Win, Claim and Submit to SOAR with CPI", async () => {

        const leaderboardPda = FindLeaderboardPda(kamikazeJoeProgram);

        const provider = anchor.AnchorProvider.env();
        const soar = SoarProgram.get(provider);

        const leaderboardInfo =
            await kamikazeJoeProgram.account.leaderboard.fetch(leaderboardPda)

        const accounts = await new AccountsBuilder(
            provider,
            soar.program.programId
        ).submitScoreAccounts(user.publicKey, leaderboardPda, leaderboardInfo.leaderboard, leaderboardInfo.game);

        let {userPda, gamePda} = await PlayAndWin(user, provider);

        console.log("Player account", accounts.playerAccount.toString());

        // Claim Price transaction
        let tx = await kamikazeJoeProgram.methods
            .claimPrizeSoar()
            .accounts({
                payer: user.publicKey,
                user: userPda,
                game: gamePda,
                receiver: null,
                vault: FindVaultPda(kamikazeJoeProgram),
                leaderboardInfo: accounts.authority,
                soarGame: accounts.game,
                soarLeaderboard: accounts.leaderboard,
                soarPlayerAccount: accounts.playerAccount,
                soarPlayerScores: accounts.playerScores,
                soarTopEntries: accounts.topEntries,
                soarProgram: soar.program.programId,
            })
            .signers([user]).rpc({skipPreflight: true});

        console.log("Claim and submit to Soar signature", tx);
        await provider.connection.confirmTransaction(tx, "confirmed");
    });


    // Utils

    async function PlayAndWin(player: Keypair, provider: AnchorProvider) {
        // Fund the player.
        await fund_address(provider.connection, player.publicKey);

        let userPda = FindUserPda(player.publicKey, kamikazeJoeProgram);

        // Initialize User.
        let tx = await InitializeUser(kamikazeJoeProgram, player, userPda);
        await provider.connection.confirmTransaction(tx, "confirmed");

        let id = new BN(0);
        let gamePda = FindGamePda(userPda, id, kamikazeJoeProgram);

        // Initialize Game.
        tx = await InitializeGame(kamikazeJoeProgram, player, userPda, gamePda);
        await provider.connection.confirmTransaction(tx, "confirmed");

        // Join the game
        tx = await JoinGame(kamikazeJoeProgram, player, userPda, gamePda, 2, 2);

        await provider.connection.confirmTransaction(tx, "confirmed");

        // Join the game with a second player
        let player2 = await new_funded_address(provider);
        let user2Pda = FindUserPda(player2.publicKey, kamikazeJoeProgram);
        tx = await InitializeUser(kamikazeJoeProgram, player2, user2Pda);
        await provider.connection.confirmTransaction(tx, "confirmed");

        tx = await JoinGame(kamikazeJoeProgram, player2, user2Pda, gamePda, 2, 3);
        await provider.connection.confirmTransaction(tx, "confirmed");

        // Explode transaction
        tx = await kamikazeJoeProgram.methods
            .explode()
            .accounts({
                payer: player.publicKey,
                user: userPda,
                game: gamePda,
                sessionToken: null,
            }).signers([player]).rpc();

        console.log("Explode and Win signature", tx);
        await provider.connection.confirmTransaction(tx, "confirmed");
        return {userPda, gamePda};
    }

});
