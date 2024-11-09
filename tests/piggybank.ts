import * as anchor from "@coral-xyz/anchor";
import { web3, Program, BN } from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from '@solana/spl-token';
import { PiggyBank } from "../target/types/piggy_bank";
import { SimpleUser, findProgramAddress} from "@solardev/simple-web3";
const assert = require("assert"); 

describe("Anchor Counter Program", () => {

    const provider = anchor.AnchorProvider.env()
    anchor.setProvider(provider);

    const program = anchor.workspace.PiggyBank as Program<PiggyBank>;
    const owner = SimpleUser.generate(provider.connection)

    let bankPda: web3.PublicKey; 
    let vaultPda: web3.PublicKey;
    let initBalance: number;

    before(async () => {
        await owner.faucet();
        await owner.mint("PEPE").commit();
        ({amount: initBalance} = await owner.balance("PEPE"));

        [bankPda, ] = findProgramAddress(
            program.programId,
            ["bank", owner.publicKey, owner.tokens["PEPE"].mint],
        );

        [vaultPda, ] = findProgramAddress(
            ASSOCIATED_TOKEN_PROGRAM_ID,
            [bankPda, TOKEN_PROGRAM_ID, owner.tokens["PEPE"].mint],
        )
        
    });

    it("piggy bank is opened properly", async () => {

        await program.methods.initBank()
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
                owner_token_account: owner.tokens["PEPE"].account,
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

    it("close the piggy bank", async () => {
        await program.methods.closeBank()
            .accounts({
                bank: bankPda,
                vault: vaultPda,
                owner: owner.publicKey,
                tokenProgram: TOKEN_PROGRAM_ID,
            })
            .signers([owner])
            .rpc();

        const {value: {amount}} = await provider.connection.getTokenAccountBalance(vaultPda);
        assert.ok(+amount === 0);

        const {amount: newBalance} = await owner.balance("PEPE");
        assert.ok(newBalance === initBalance);
    });

});
