import {
    EntityDetailsRequest,
    EntityNonFungibleIdsRequest, NonFungibleDataRequest,
} from "@radixdlt/babylon-gateway-api-sdk";
import {backend_api_url, issuer_address, loan_address, radix_api_url} from "../general/constants";

async function getLendersList() {
    const obj: EntityDetailsRequest = {
        "address": issuer_address
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
        return {token: row[0], lender: row[1]}
    });
}

async function getLenderInformation(lender_address: string) {

    const obj: EntityDetailsRequest = {
        "address": lender_address
    };

    let data;
    await fetch(radix_api_url + '/entity/details', {
        method: 'POST',
        body: JSON.stringify(obj),
        headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8',})
    })
        .then((response) => response.json())
        .then((tmp_data) => data = tmp_data["details"]["state"]["data_json"] )
        .catch(console.error)

    console.log(data);

    if (!data) return undefined;

    // @ts-ignore
    const loan_to_value = data[1];
    // @ts-ignore
    const daily_interest_rate = data[2];
    // @ts-ignore
    const liquidation_threshold = data[3];
    // @ts-ignore
    const liquidation_penalty = data[4];
    // @ts-ignore
    const oracle_address = data[5];

    const current_price = await getOraclePrice(oracle_address);

    return { loan_to_value: loan_to_value, daily_interest_rate: daily_interest_rate, liquidation_threshold: liquidation_threshold, liquidation_penalty: liquidation_penalty, price: current_price }
}

async function getLoansOwnedBy(account: string) {

    const obj: EntityNonFungibleIdsRequest = {
        "address": account,
        "resource_address": loan_address
    };

    let data;
    await fetch(radix_api_url + `/entity/non-fungible/ids`, {
        method: 'POST',
        body: JSON.stringify(obj),
        headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8',})
    })
        .then((response) => response.json())
        .then((tmp_data) => data = tmp_data["non_fungible_ids"]["items"])
        .catch(console.error);

    let loan_ids: any[] = [];
    // @ts-ignore
    for (const id of data) {

        const loan_id = id["non_fungible_id"];
        loan_ids.push(loan_id);
    }

    return loan_ids
}

async function getHex(loan_id: string) {

    const obj: NonFungibleDataRequest = {
        "address": loan_address,
        "non_fungible_id": loan_id
    };

    let data;
    await fetch(radix_api_url + `/entity/non-fungible/data`, {
        method: 'POST',
        body: JSON.stringify(obj),
        headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8',})
    })
        .then((response) => response.json())
        .then((tmp_data) => data = tmp_data)
        .catch(console.error);

    // @ts-ignore
    let mutable_hex = data["mutable_data_hex"];
    // @ts-ignore
    let immutable_hex = data["immutable_data_hex"];

    return { mutable_hex: mutable_hex, immutable_hex: immutable_hex };
}

async function getOraclePrice(oracle_address: string) {

    const obj: EntityDetailsRequest = {
        "address": oracle_address
    };

    let data;
    await fetch(radix_api_url + '/entity/details', {
        method: 'POST',
        body: JSON.stringify(obj),
        headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8',})
    })
        .then((response) => response.json())
        .then((tmp_data) => data = tmp_data["details"]["state"]["data_json"] )
        .catch(console.error)

    // @ts-ignore
    return data[0];
}

async function decode_loan(mutable_data: string, immutable_data: string){


    const params = new URLSearchParams();
    params.append('mutable_data_hex', mutable_data);
    params.append('immutable_data_hex', immutable_data);

    const request = new Request( `${backend_api_url}/decode_loan?${params}`, {
        method: 'GET',
        headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8',})
    });

    const res = await fetch(request)
    const data = await res.json()
    return data
}

export { getLendersList, getLenderInformation, getLoansOwnedBy }