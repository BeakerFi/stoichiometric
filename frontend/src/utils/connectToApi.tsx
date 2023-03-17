import {api_url, router_address} from "./constants";
import {EntityDetailsRequest, EntityDetailsResponse} from "@radixdlt/babylon-gateway-api-sdk";

async function getTokens() {

    let tokens_list: any[] = [{name: "Beta USD", symb: "BUSD", address: "resource_tdx_b_1qpev6f8v2su68ak5p2fswd6gqml3u7q0lkrtfx99c4ts3zxlah", icon_url: "https://pixlok.com/wp-content/uploads/2021/03/Dollar-Coins-PNG.jpg"}];

    const obj: EntityDetailsRequest = {
        "address": router_address
    };

    let data;
    await fetch( api_url + `/entity/details`, {
        method: 'POST',
        body: JSON.stringify(obj),
        headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8',})
    })
        .then( (response) => response.json() )
        .then( (tmp_data) => data = tmp_data )
        .catch(console.error);

    // @ts-ignore
    const pools = data["details"]["state"]["data_json"][1].map(row => {return {token: row[0], pool: row[1]}});

    for (const {token, pool} of pools) {

        const obj: EntityDetailsRequest = {
            "address": token
        };

        let data;
        await fetch( api_url + `/entity/details`, {
            method: 'POST',
            body: JSON.stringify(obj),
            headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8',})
        })
            .then( (response) => response.json() )
            .then( (tmp_data) => data = tmp_data )
            .catch(console.error);

        // @ts-ignore
        const metadata = data["metadata"]["items"];
        tokens_list.push( {name: metadata[2]["value"], symb: metadata[3]["value"], address: token, icon_url: metadata[1]["value"]});
    }

    return tokens_list;
}

async function getNbTokens(account: string) {
    let nbTokensList: any[] = [];

    const obj: EntityDetailsRequest = {
        "address": account
    };

    let data;
    await fetch( api_url + `/entity/resources`, {
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
        nbTokensList[fungible[i]["address"]] = parseFloat(fungible[i]["amount"]["value"])
    }

    return [nbTokensList, account];
}


export { getTokens, getNbTokens }