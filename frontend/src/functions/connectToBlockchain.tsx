import {
    RadixDappToolkit,
  } from '@radixdlt/radix-dapp-toolkit'

const dAppId = 'account_tdx_b_1pza2s6mry0jqz9agu5td8msu9c4yqnvq8acngylytgjqj84unu'

var rdt = RadixDappToolkit(
    { dAppDefinitionAddress: dAppId, dAppName: 'Beaker.fi' },
    (x) => null,
    { networkId: 11 }
);

function resetRdt() {
    localStorage.clear();
}

const bench_address = "component_tdx_b_1qt7c7ws0a4f3wd3mwtcj4acvn87w4as9zyvkx3wwq8lskwe5zm";
const position_address = "resource_tdx_b_1qq02xf63eauv5xfg0ruggmmzruk80jl5tle4shh8x7uq6j8d4d";

async function sendSwapTransaction(account: string, token1Address: string, token2Address: string, amount: string) {
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
        ComponentAddress("${bench_address}")
        "swap"
        Bucket("0")
        ResourceAddress("${token2Address}");
        
    CALL_METHOD
        ComponentAddress("${account}")
        "deposit_batch"
        Expression("ENTIRE_WORKTOP");`
  
    const result = await rdt.sendTransaction({
      transactionManifest: manifest,
      version: 1,
    });

    if (result.isErr()) {
        return false
    } else return true
}

async function createPosition(account: string, token1Address: string, token2Address: string, token1Amount: string, token2Amount: string) {
    const manifest = `
    CALL_METHOD
        ComponentAddress("${account}")
        "withdraw_by_amount"
        Decimal("${token1Amount}")
        ResourceAddress("${token1Address}");

    TAKE_FROM_WORKTOP_BY_AMOUNT
        Decimal("${token1Amount}")
        ResourceAddress("${token1Address}")
        Bucket("bucket_a");

    CALL_METHOD
        ComponentAddress("${account}")
        "withdraw_by_amount"
        Decimal("${token2Amount}")
        ResourceAddress("${token2Address}");

    TAKE_FROM_WORKTOP_BY_AMOUNT
        Decimal("${token2Amount}")
        ResourceAddress("${token2Address}")
        Bucket("bucket_b");

    CALL_METHOD
        ComponentAddress("${bench_address}")
        "add_liquidity"
        Bucket("bucket_a")
        Bucket("bucket_b")
        None;
        
    CALL_METHOD
        ComponentAddress("${account}")
        "deposit_batch"
        Expression("ENTIRE_WORKTOP");`
  
    const result = await rdt.sendTransaction({
      transactionManifest: manifest,
      version: 1,
    });

    if (result.isErr()) {
        return false
    } else return true
}

async function addToPosition(account: string, token1Address: string, token2Address: string, token1Amount: string, token2Amount: string, positionId: string) {
    const manifest = `
        CALL_METHOD
            ComponentAddress("${account}")
            "withdraw_by_amount"
            Decimal("${token1Amount}")
            ResourceAddress("${token1Address}");

        TAKE_FROM_WORKTOP_BY_AMOUNT
            Decimal("${token1Amount}")
            ResourceAddress("${token1Address}")
            Bucket("0");

        CALL_METHOD
            ComponentAddress("${account}")
            "withdraw_by_amount"
            Decimal("${token2Amount}")
            ResourceAddress("${token2Address}");

        TAKE_FROM_WORKTOP_BY_AMOUNT
            Decimal("${token2Amount}")
            ResourceAddress("${token2Address}")
            Bucket("1");

        CALL_METHOD
            ComponentAddress("${account}")
            "create_proof_by_ids"
            Array<NonFungibleLocalId>(NonFungibleLocalId("${positionId}"))
            ResourceAddress("${position_address}");

        CREATE_PROOF_FROM_AUTH_ZONE_BY_IDS
            Array<NonFungibleLocalId>(NonFungibleLocalId("${positionId}"))
            ResourceAddress("${position_address}")
            Proof("0");

        CALL_METHOD
            ComponentAddress("${bench_address}")
            "add_liquidity"
            Bucket("0")
            Bucket("1")
            Some(Proof("0"));
            
        CALL_METHOD
            ComponentAddress("${account}")
            "deposit_batch"
            Expression("ENTIRE_WORKTOP");`
    
    const result = await rdt.sendTransaction({
        transactionManifest: manifest,
        version: 1,
    });

    if (result.isErr()) {
        return false
    } else return true
}

async function claimFees(account: string, positionId: string) {
    const manifest = `
        CALL_METHOD
            ComponentAddress("${account}")
            "create_proof_by_ids"
            Array<NonFungibleLocalId>(NonFungibleLocalId("${positionId}"))
            ResourceAddress("${position_address}");

        CREATE_PROOF_FROM_AUTH_ZONE_BY_IDS
            Array<NonFungibleLocalId>(NonFungibleLocalId("${positionId}"))
            ResourceAddress("${position_address}")
            Proof("0");

        CALL_METHOD
            ComponentAddress("${bench_address}")
            "claim_fees"
            Proof("0");
            
        CALL_METHOD
            ComponentAddress("${account}")
            "deposit_batch"
            Expression("ENTIRE_WORKTOP");`

    const result = await rdt.sendTransaction({
        transactionManifest: manifest,
        version: 1,
    });

    if (result.isErr()) {
        return false
    } else return true
}

async function removeLiquidity(account: string, positionId: string, toRemove: string) {
    const manifest = `
        CALL_METHOD
            ComponentAddress("${account}")
            "create_proof_by_ids"
            Array<NonFungibleLocalId>(NonFungibleLocalId("${positionId}"))
            ResourceAddress("${position_address}");

        CREATE_PROOF_FROM_AUTH_ZONE_BY_IDS
            Array<NonFungibleLocalId>(NonFungibleLocalId("${positionId}"))
            ResourceAddress("${position_address}")
            Proof("0");

        CALL_METHOD
            ComponentAddress("${bench_address}")
            "remove_liquidity"
            Decimal("${toRemove}")
            Proof("0");

        CALL_METHOD
            ComponentAddress("${account}")
            "deposit_batch"
            Expression("ENTIRE_WORKTOP");`

        
    console.log(manifest);
  
    const result = await rdt.sendTransaction({
        transactionManifest: manifest,
        version: 1,
    });

    if (result.isErr()) {
        return false
    } else return true
}

async function closePosition(account: string, positionId: string) {
    const manifest = `
        CALL_METHOD
            ComponentAddress("${account}")
            "withdraw_by_ids"
            Array<NonFungibleLocalId>(NonFungibleLocalId("${positionId}"))
            ResourceAddress("${position_address}");

        TAKE_FROM_WORKTOP_BY_IDS
            Array<NonFungibleLocalId>(NonFungibleLocalId("${positionId}"))
            ResourceAddress("${position_address}")
            Bucket("0");

        CALL_METHOD
            ComponentAddress("${bench_address}")
            "close_positions"
            Bucket("0");

        CALL_METHOD
            ComponentAddress("${account}")
            "deposit_batch"
            Expression("ENTIRE_WORKTOP"); 
        `

    const result = await rdt.sendTransaction({
        transactionManifest: manifest,
        version: 1,
    });

    if (result.isErr()) {
        return false
    } else return true

}
export { sendSwapTransaction, createPosition, rdt, resetRdt, addToPosition, claimFees, removeLiquidity, closePosition };