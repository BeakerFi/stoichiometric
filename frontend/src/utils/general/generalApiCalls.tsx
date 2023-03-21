import {radix_api_url, router_address} from "./constants";
import {EntityDetailsRequest} from "@radixdlt/babylon-gateway-api-sdk";;

import {lender, pool, token} from "types";

async function getToken(address: string): Promise<token>{

    const obj: EntityDetailsRequest = {
        "address": address
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
    return {name: metadata[2]["value"], symb: metadata[3]["value"], address: address, icon_url: metadata[1]["value"]};
}

async function getOwnedTokens(account: string) {

    let ownedTokensList: any; /*TODO*/

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

async function getRawPoolsList() {
    const obj: EntityDetailsRequest = {
        "address": router_address
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
    return data["details"]["state"]["data_json"][1].map(row => {
        return {token: row[0], pool_address: row[1]}
    });
}

async function getTokensPoolsAndLenders() {

    let raw_list = await getRawPoolsList();

    const tokens: token[] = await Promise.all(raw_list.map((raw_pool: { token: string, pool_address: string; }) => {
        return getToken(raw_pool.token);
    }));

    const pools: pool[] = [];
    const lenders: lender[] = [];


    return { tokens, pools, lenders };
}

export { getToken, getOwnedTokens, getTokensPoolsAndLenders }