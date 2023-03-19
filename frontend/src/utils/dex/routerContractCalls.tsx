import {router_address, stablecoin_address} from "../general/constants";
import {rdt} from "../connectToWallet";

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

async function swap_indirect(account: string, token1Address: string, token2Address: string, amount: string) {
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
                      ResourceAddress("${stablecoin_address}");
                    
                    TAKE_FROM_WORKTOP
                      ResourceAddress(${stablecoin_address})
                      Bucket("1");
                    
                    CALL_METHOD
                      ComponentAddress("${router_address}")
                      "swap"
                      Bucket("1")
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

async function addLiquidity(account: string, other_token: string, stablecoin_amount: number, other_token_amount: number, steps: any[]) {

    let stablecoins_amount: number[];
    let other_tokens_amount: number[];
    let steps_id: number[];

    for (const step of steps) {
        
    }
}



export { swap_direct, swap_indirect }