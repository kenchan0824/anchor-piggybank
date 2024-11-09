import * as anchor from "@coral-xyz/anchor";
import { web3, Program, BN } from "@coral-xyz/anchor";
import { getAssociatedTokenAddress, 
    TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID,  
} from '@solana/spl-token';
import { PiggyBank } from "../target/types/piggy_bank";
import { SimpleUser, findProgramAddress, u16 } from "@solardev/simple-web3";
const assert = require("assert"); 

describe("Anchor Counter Program", () => {

    const provider = anchor.AnchorProvider.env()
    anchor.setProvider(provider);

    const program = anchor.workspace.PiggyBank as Program<PiggyBank>;
    const owner = SimpleUser.generate(provider.connection)

    it("counter account is initialized properly", async () => {
        await owner.faucet();
        await owner.mint("PEPE").commit();

        const [bankPda, bump] = findProgramAddress(
            program.programId,
            ["bank", owner.publicKey, owner.tokens["PEPE"].mint],
        );

        const vaultAddr = await getAssociatedTokenAddress(
            owner.tokens["PEPE"].mint,
            bankPda,
            true
        );

        await program.methods.openBank()
            .accounts({
                bank: bankPda,
                vault: vaultAddr,
                owner: owner.publicKey,
                mint: owner.tokens["PEPE"].mint,
            })
            .signers([owner])
            .rpc();

        const bankAccount = await program.account.piggyBank.fetch(bankPda);
        assert.ok(bankAccount.owner.toBase58() === owner.publicKey.toBase58());
        assert.ok(bankAccount.mint.toBase58() === owner.tokens["PEPE"].mint.toBase58());
        assert.ok(bankAccount.balance.toNumber() === 0);

        const {value: {amount}} = await provider.connection.getTokenAccountBalance(vaultAddr);
        assert.ok(+amount === 0);
    });

});