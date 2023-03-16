const api_url = 'https://beaker.fi:8888'

async function getTokens() {
  const request = new Request( api_url + '/marketv2', {
    method: 'GET',
    headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8',})
  });

  let tokens_list: any[] = [{name: 'Radix', symb: 'XRD', address: 'resource_tdx_b_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8z96qp', icon_url: "https://switchere.com/i/currencies/XRD.svg"},
  {name: "Wrapped Bitcoin", symb: "WBTC", address: "resource_tdx_b_1qre9sv98scqut4k9g3j6kxuvscczv0lzumefwgwhuf6qdu4c3r", icon_url: "https://upload.wikimedia.org/wikipedia/commons/thumb/4/46/Bitcoin.svg/1024px-Bitcoin.svg.png"},
  {name: "Stoichiometric USD", symb: "SUSD", address: "resource_tdx_b_arthurjetebaisegrosfdp111111fdpputeputeshitcoin", icon_url: "https://upload.wikimedia.org/wikipedia/commons/thumb/4/46/Bitcoin.svg/1024px-Bitcoin.svg.png"}];

  return tokens_list;
}

async function getNbTokens(account: string){
  const params = new URLSearchParams();
  params.append('wallet', account);

  const request = new Request( `${api_url}/getTokensOfWallet?${params}`, {
  method: 'GET',
  headers: new Headers({ 'Content-Type': 'application/json; charset=UTF-8',})
  });

  let tokens
  await fetch(request)
  .then(data => data.json())
  .then(res => tokens = res)
  .catch( e => console.log(e) )
  return tokens
}

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