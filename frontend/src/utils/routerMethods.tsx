import {router_address} from "./constants";
import {rdt} from "./connectToWallet";

async function swap_direct(account: string, token1Address: string, token2Address: string, amount: string) {
    const manifest = `
                    CALL_METHOD
                      ComponentAddress("${account}")
                      "withdraw_by_amount"
                      Decimal("${amount}")
                      ResourceAddress("${token1Address}");
                    
                    TAKE_FROM_WORKTOP_BY_AMOUNT
                      Decimal("${amount}")
                      ResourceAddress("${token1Address}")
                      Bucket("0");
                    
                    CALL_METHOD
                      ComponentAddress("${router_address}")
                      "swap"
                      Bucket("0")
                      ResourceAddress("${token2Address}");
                    
                    CALL_METHOD
                      ComponentAddress("${account}")
                      "deposit_batch"
                      Expression("ENTIRE_WORKTOP");
    `;

    const result = await rdt.sendTransaction({
        transactionManifest: manifest,
        version: 1,
    });

    return !result.isErr();
}

export { swap_direct }