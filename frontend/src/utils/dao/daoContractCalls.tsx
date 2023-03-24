import {position_address, router_address, stablecoin_address} from "../general/constants";
import {rdt} from "../connectToWallet";

import { step } from "types";

async function voterCardFromTokens(account: string,  stablecoin_amount: string) {
    const manifest = `
    CALL_METHOD
        ComponentAddress("${account}")
        "withdraw_by_amount"
        Decimal("${stablecoin_amount}")
        ResourceAddress("${stablecoin_address}");

    TAKE_FROM_WORKTOP_BY_AMOUNT
        Decimal("${stablecoin_amount}")
        ResourceAddress("${stablecoin_address}")
        Bucket("0");

    CALL_METHOD
        ComponentAddress("${component_address}")
        "lock_stablecoins"
        Bucket("0")
        None;

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