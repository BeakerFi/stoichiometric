import {
    EntityDetailsRequest,
    EntityNonFungibleIdsRequest,
    NonFungibleDataRequest,
    NonFungibleIdsRequest,
    NonFungibleIdsResponse
} from "@radixdlt/babylon-gateway-api-sdk";
import {backend_api_url, issuer_address, loan_address, radix_api_url} from "../general/constants";
import {amountToLiquidate} from "./stablecoinMaths";

import {lender, loan, token} from "types";

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

async function getLenderInformation(collateral_token: token, lender_address: string): Promise<lender> {

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
    const loan_to_value = parseFloat(data[1]);
    // @ts-ignore
    const daily_interest_rate = parseFloat(data[2]);
    // @ts-ignore
    const liquidation_threshold = parseFloat(data[3]);
    // @ts-ignore
    const liquidation_penalty = parseFloat(data[4]);
    // @ts-ignore
    const oracle_address = data[5];

    const current_price = await getOraclePrice(oracle_address);

    return { collateral_token: collateral_token, collateral_price: current_price, oracle: oracle_address, loan_to_value: loan_to_value, interest_rate: daily_interest_rate, liquidation_threshold: liquidation_threshold, liquidation_penalty: liquidation_penalty };
}

async function getOwnedLoans(account: string, lenders: lender[]): Promise<loan[]> {

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


    let loan_ids: string[] = [];

    // @ts-ignore
    for (let i = 0; i < data.length; ++i) {
        // @ts-ignore
        loan_ids.push(data[i]["non_fungible_id"]);
    }

    return await Promise.all(
        loan_ids.map(
            async id => {

                getHex(id).then(
                    (hexes) => getLoanInformation(hexes.mutable_hex, hexes.immutable_hex, lenders)
                )
            }
        )
    );
}

async function getLoanInformation(loan_id: string, lenders: lender[]): Promise<loan> {

    const obj: NonFungibleDataRequest = {
        "address": loan_address,
        "non_fungible_id": loan_id
    };

    let data;
    await fetch(radix_api_url + `/non-fungible/data`, {
        method: 'POST',
        body: JSON.stringify(obj),
        headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8',})
    })
        .then((response) => response)
        .then((tmp_data) => data = tmp_data)
        .catch(console.error);


    console.log("data", data);

    // @ts-ignore
    let mutable_hex = data["mutable_data_hex"];
    // @ts-ignore
    let immutable_hex = data["immutable_data_hex"];

    const params = new URLSearchParams();
    params.append('mutable_data_hex', mutable_hex);
    params.append('immutable_data_hex', immutable_hex);

    const request = new Request( `${backend_api_url}/decode_loan?${params}`, {
        method: 'GET',
        headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8',})
    });

    const res = await fetch(request)
    const data = await res.json()

    const lender = lenders[data.collateral_token];
    let collateral_price = lender.price * data.collateral_amount;

    let amount_to_liquidate = await amountToLiquidate(data.collateral_amount, collateral_price, data.amount_lent, lender.liquidation_threshold, lender.liquidation_penalty, data.interest_rate, data.loan_time);

    let liquidation_price = 20000;

    return { collateral_token: data.collateral_token, collateral_amount: data.collateral_amount, amount_lent: data.amount_lent, liquidation_price: liquidation_price, amount_to_liquidate: amount_to_liquidate };
}

async function getOraclePrice(oracle_address: string): Promise<number> {

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
    return parseFloat(data[0]);
}

async function getAllLoansInformation(loan_ids: string[], lenders: lender[]) {
    const hexes = await Promise.all(loan_ids.map(async id => getHex(id)))
    return Promise.all(hexes.map( async hex => getLoanInformation(hex.mutable_hex, hex.immutable_hex, lenders)))
}

async function getAllCollection(): Promise<string[]> {

    try {
      let cursor: string | null | undefined = '';
      const ids:string[] = [];
  
      while (cursor !== undefined) {
        const obj: NonFungibleIdsRequest = {
            "address": loan_address,
            "cursor" : cursor
        };
        await fetch(radix_api_url + '/non-fungible/ids', {
          method: 'POST',
          body: JSON.stringify(obj),
          headers: { 'Content-Type': 'application/json; charset=UTF-8' },
        })
          .then((response) => response.json())
          .then((data) => {
            const response = data as NonFungibleIdsResponse;
            response.non_fungible_ids.items.forEach( item => {
              ids.push(item.non_fungible_id)
            });
            cursor = response.non_fungible_ids.next_cursor;
          })
          .catch(console.error);
      }
  
      return ids;
    } catch {
      throw new Error("error");
    }
  }

export { getLendersList, getLenderInformation, getOwnedLoans, getAllLoansInformation, getAllCollection}