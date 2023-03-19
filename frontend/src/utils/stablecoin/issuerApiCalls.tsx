import {EntityDetailsRequest,} from "@radixdlt/babylon-gateway-api-sdk";
import {radix_api_url} from "../constants";

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

export { getLenderInformation }