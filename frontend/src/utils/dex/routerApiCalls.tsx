import {radix_api_url, position_address, router_address, token_default} from "../general/constants";
import {
    EntityDetailsRequest,
    EntityNonFungibleIdsRequest,
    NonFungibleDataRequest
} from "@radixdlt/babylon-gateway-api-sdk";

import { token, position, pool, step } from "types";


async function getOwnedPositions(account: string): Promise<position[]> {

    const obj: EntityNonFungibleIdsRequest = {
        "address": account,
        "resource_address": position_address
    };

    let data: any;
    await fetch(radix_api_url + `/entity/non-fungible/ids`, {
        method: 'POST',
        body: JSON.stringify(obj),
        headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8',})
    })
        .then((response) => response.json())
        .then((tmp_data) => data = tmp_data["non_fungible_ids"]["items"])
        .catch(console.error);

    if (!data) return [];

    const positions: position[] = [];
    // @ts-ignore
    for (let i = 0; i < data.length; ++i) {

        const nf_id = data[i]["non_fungible_id"];
        positions.push({...{nfIdValue: await getNFIDValue(nf_id)}, token: token_default, id: data[i]["non_fungible_id"], x_fees: 0, y_fees: 0, value_locked: 0, price_x: 1, liquidity: 0});

    }

    return positions;
}

async function getNFIDValue(id: string) {
    const obj: NonFungibleDataRequest = {
        "address": position_address,
        "non_fungible_id": id
    }

    let data: any;
    await fetch(radix_api_url + `/non-fungible/data`, {
        method: 'POST',
        body: JSON.stringify(obj),
        headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8',})
    })
        .then((response) => response.json())
        .then((tmp_data) => data = tmp_data)
        .catch(console.error);

    return data

}

async function getPoolInformation(token: token, pool_address: string): Promise<pool> {

    const obj: EntityDetailsRequest = {
        "address": pool_address
    }

    let data: any;
    await fetch(radix_api_url + '/entity/details', {
        method: 'POST',
        body: JSON.stringify(obj),
        headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8',})
    })
        .then((response) => response.json())
        .then((tmp_data) => data = tmp_data["details"]["state"]["data_json"])
        .catch(console.error);

    const pool_steps: step[] = await Promise.all(data[6].map(async (pool_step: string[]) => {
        await getPoolStep(parseFloat(pool_step[0]),pool_step[1])
    }));

    return {token: token, rate_step: parseFloat(data[0]), current_step: parseFloat(data[1]), min_rate: parseFloat(data[2]), max_rate: parseFloat(data[3]), steps: pool_steps};
}

async function getPoolStep(step_id: number, step_address: string): Promise<step> {

    const obj: EntityDetailsRequest = {
        "address": step_address
    }

    let data: any;
    await fetch(radix_api_url + '/entity/details', {
        method: 'POST',
        body: JSON.stringify(obj),
        headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8',})
    })
        .then((response) => response.json())
        .then((tmp_data) => data = tmp_data["details"]["state"]["data_json"])
        .catch(console.error);

    return {step_id: step_id, stablecoin_amount: parseFloat(data[0]), other_token_amount: data[1], rate: data[2]}
}


export {getPoolInformation, getOwnedPositions}