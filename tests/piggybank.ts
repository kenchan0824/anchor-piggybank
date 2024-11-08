import * as anchor from "@coral-xyz/anchor";
import { web3, Program, BN } from "@coral-xyz/anchor";
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

        const [bankPda, bump] = findProgramAddress(
            program.programId,
            ["bank", owner.publicKey],
        );

        await program.methods.openBank()
            .accounts({
                bank: bankPda,
                owner: owner.publicKey,
            })
            .signers([owner])
            .rpc();

        const bankAccount = await program.account.piggyBank.fetch(bankPda);
        assert.ok(bankAccount.owner.toBase58() === owner.publicKey.toBase58());
        assert.ok(bankAccount.balance.toNumber() === 0);
    });

});