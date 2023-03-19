import {radix_api_url, stablecoin_address} from "./constants";
import {EntityDetailsRequest} from "@radixdlt/babylon-gateway-api-sdk";
import {getPoolInfo, getPoolsList} from "../dex/routerApiCalls";
import {getLendersList} from "../stablecoin/issuerApiCalls";

async function getTokens() {

    let tokens_list: any[] = [{name: "Stoichiometric USD", symb: "SUSD", address: stablecoin_address, icon_url: "https://cdn-icons-png.flaticon.com/512/3215/3215346.png"}];

    let pools = await getPoolsList();

    for (const {token, } of pools) {

        const obj: EntityDetailsRequest = {
            "address": token
        };

        let data;
        await fetch( radix_api_url + `/entity/details`, {
            method: 'POST',
            body: JSON.stringify(obj),
            headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8',})
        })
            .then( (response) => response.json() )
            .then( (tmp_data) => data = tmp_data )
            .catch(console.error);

        // @ts-ignore
        const metadata = data["metadata"]["items"];
        tokens_list.push( {name: metadata[1]["value"], symb: metadata[2]["value"], address: token, icon_url: metadata[0]["value"]});
    }

    return tokens_list;
}

async function getOwnedTokens(account: string) {
    let ownedTokensList: any[] = [];

    const obj: EntityDetailsRequest = {
        "address": account
    };

    let data;
    await fetch( radix_api_url + `/entity/resources`, {
        method: 'POST',
        body: JSON.stringify(obj),
        headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8',})
    })
        .then( (response) => response.json() )
        .then( (tmp_data) => data = tmp_data )
        .catch(console.error);

    // @ts-ignore
    const fungible = data["fungible_resources"]["items"];

    for (var i = 0; i < fungible.length; ++i) {
        ownedTokensList[fungible[i]["address"]] = parseFloat(fungible[i]["amount"]["value"])
    }

    return [ownedTokensList, account];
}


async function getTokensPoolsAndLenders() {
    const tokens = await getTokens();
    const pools = await getPoolsList();
    const lenders = await getLendersList();
    return { tokens, pools, lenders };
}

export { getTokens, getOwnedTokens, getTokensPoolsAndLenders }