import {EntityDetailsRequest,} from "@radixdlt/babylon-gateway-api-sdk";
import {backend_api_url, radix_api_url} from "../general/constants";

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

    console.log(obj);
    console.log(data);

    // @ts-ignore
    const loan_to_value = data[2];
    // @ts-ignore
    const daily_interest_rate = data[3];
    // @ts-ignore
    const liquidation_threshold = data[4];
    // @ts-ignore
    const liquidation_penalty = data[5];
    // @ts-ignore
    const oracle_address = data[6];
    const current_price = await getOraclePrice(oracle_address);

    return { loan_to_value: loan_to_value, daily_interest_rate: daily_interest_rate, liquidation_threshold: liquidation_threshold, liquidation_penalty: liquidation_penalty, price: current_price }
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
    return data[1];
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

export { getLenderInformation }