import * as anchor from "@coral-xyz/anchor";
import { web3, Program, BN } from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { PiggyBank } from "../target/types/piggy_bank";
import { SimpleUser, findProgramAddress} from "@solardev/simple-web3";
const assert = require("assert"); 

function wait(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

describe("Anchor PiggyBank Program", () => {

    const provider = anchor.AnchorProvider.env()
    anchor.setProvider(provider);

    const program = anchor.workspace.PiggyBank as Program<PiggyBank>;
    const owner = SimpleUser.generate(provider.connection)

    let bankPda: web3.PublicKey; 
    let vaultPda: web3.PublicKey;
    let initBalance: number;

    before(async () => {

        console.log("Owner: ", owner.publicKey.toBase58());

        await owner.faucet();
        await owner.mint("PEPE").commit();
        ({amount: initBalance} = await owner.balance("PEPE"));

        [bankPda, ] = findProgramAddress(
            program.programId,
            ["bank", owner.publicKey, owner.tokens["PEPE"].mint],
        );

        [vaultPda, ] = findProgramAddress(
            program.programId,
            ["bank_vault", bankPda],
        );
    });

    it("piggy bank is opened properly", async () => {
        const timeout_secs = new BN(2);
        await program.methods.initBank(timeout_secs)
            .accounts({
                bank: bankPda,
                vault: vaultPda,
                owner: owner.publicKey,
                mint: owner.tokens["PEPE"].mint,
            })
            .signers([owner])
            .rpc();

        const bankAccount = await program.account.piggyBank.fetch(bankPda);
        assert.ok(bankAccount.owner.toBase58() === owner.publicKey.toBase58());
        assert.ok(bankAccount.mint.toBase58() === owner.tokens["PEPE"].mint.toBase58());
        assert.ok(bankAccount.balance.toNumber() === 0);

        const {value: {amount}} = await provider.connection.getTokenAccountBalance(vaultPda);
        assert.ok(+amount === 0);
    });

    it("deposit into piggy bank", async () => {

        await program.methods.deposit(new BN(100e9))
            .accounts({
                bank: bankPda,
                ownerTokenAccount: owner.tokens["PEPE"].tokenAccount,
                vault: vaultPda,
                owner: owner.publicKey,
                tokenProgram: TOKEN_PROGRAM_ID,
            })
            .signers([owner])
            .rpc();

        const {value: {amount}} = await provider.connection.getTokenAccountBalance(vaultPda);
        assert.ok(+amount === 100e9);

        const {amount: newBalance} = await owner.balance("PEPE");
        assert.ok(newBalance === initBalance - 100);
    });

    it("close the piggy bank should fail before timeout", async () => {
        
        let failed = false;
        try {
            await program.methods.closeBank()
                .accounts({
                    bank: bankPda,
                    vault: vaultPda,
                    ownerTokenAccount: owner.tokens["PEPE"].tokenAccount,
                    owner: owner.publicKey,
                    tokenProgram: TOKEN_PROGRAM_ID,
                })
                .signers([owner])
                .rpc();
        
        } catch(e) {
            failed = true;
        }

        assert.ok(failed);
    });


    it("close the piggy bank", async () => {
        
        console.log("Wait for 2 seconds...");
        await wait(2000);

        await program.methods.closeBank()
            .accounts({
                bank: bankPda,
                vault: vaultPda,
                ownerTokenAccount: owner.tokens["PEPE"].tokenAccount,
                owner: owner.publicKey,
                tokenProgram: TOKEN_PROGRAM_ID,
            })
            .signers([owner])
            .rpc();

        const {amount: newBalance} = await owner.balance("PEPE");
        assert.ok(newBalance === initBalance);
    });

});
