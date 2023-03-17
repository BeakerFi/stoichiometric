import { getTokens, getNbTokens } from "./connectToApi";

const api_url = 'https://beaker.fi:8888'

async function getPositions(address: string) {

  const params = new URLSearchParams();
  params.append('account', address);

  const request = new Request( `${api_url}/myPositions?${params}`, {
    method: 'GET',
    headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8',})
  });

  let positions
  await fetch(request)
  .then(data => data.json())
  .then(data => positions = data)
  .catch( e => console.log(e) )

  return positions
}

async function getPositionInfos(id: string) {
  const params = new URLSearchParams();
  params.append('id', id);

  const request = new Request( `${api_url}/positionValue?${params}`, {
    method: 'GET',
    headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8',})
  });

  let positionData
  await fetch(request)
  .then(data => data.json())
  .then(data => positionData = data)
  .catch( e => console.log(e) )

  console.log(positionData);

  return positionData
}

async function getPools() {

  const api_url = 'https://beaker.fi:8888'

  const request = new Request( api_url + '/pools', {
    method: 'GET',
    headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8',})
  });

  let pools: any[] = [];
  await fetch(request)
  .then(data => data.json())
  .then(arr => pools = arr)
  .catch( e => console.log(e) )

  console.log("pools", pools)

  return pools
}

async function getTokensAndPools() {
  const tokens = await getTokens();
  const pools = await getPools();
  return { tokens, pools };
}

async function getPrice(token1: string,token2: string) {

  const params = new URLSearchParams();
  params.append('token_x', token1);
  params.append('token_y', token2);

  const request = new Request( `${api_url}/pool_info_price_v2?${params}`, {
    method: 'GET',
    headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8',})
  });

  console.log(request);

  let pool_info
  await fetch(request)
  .then(data => data.json())
  .then(data => pool_info = data)
  .catch( e => console.log(e) )
  return pool_info
}

export { getTokensAndPools, getNbTokens, getPositions, getPositionInfos, getPrice };