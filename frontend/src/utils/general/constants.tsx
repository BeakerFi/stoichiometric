
/*
    General purpose constants
*/

const dAppId = "account_tdx_b_1pq8yjqm777eg2jhla6sdye5swlkt7wyj65tqjzgav44qw9m04l";

const radix_api_url = "https://betanet.radixdlt.com"

const backend_api_url = 'https://beaker.fi:8888'


/*
    Dex constants
 */

const router_address = "component_tdx_b_1q2ynsv6lj92vgr5n2kef79jjhrad3x766m72lvvgjteqq9c60n";

const position_address = "resource_tdx_b_1qq43nszmgqe7kr95dq8l6s2v6x2fhj9rrvw6swghu3vqz7k7xa";


/*
    Stablecoin constants
 */

const issuer_address = "component_tdx_b_1qfhyydhm77z6y82x36ywkjgwsaqaev5f2z3hszpfcy6q6lclud";

const stablecoin_address = "resource_tdx_b_1qpua4ltl0s8yh7yqvwysqnulvka7jkngyafaw57zd9dqatp3hj";

const loan_address = "resource_tdx_b_1qqwunyhu96jtwpwjwup3jndmws0xht3ul3ncqa2r23cqghtj6j";

const stable_coin = {name: "Stoichiometric USD", symb: "SUSD", address: stablecoin_address, icon_url: "https://cdn-icons-png.flaticon.com/512/3215/3215346.png"}

const token_default = {name: 'Wrapped Bitcoin', symb: 'WBTC', address: 'resource_tdx_b_1qre9sv98scqut4k9g3j6kxuvscczv0lzumefwgwhuf6qdu4c3r', icon_url: 'https://upload.wikimedia.org/wikipedia/commons/thumb/4/46/Bitcoin.svg/1200px-Bitcoin.svg.png'};





export {dAppId, radix_api_url, backend_api_url, router_address, position_address, issuer_address, stablecoin_address, loan_address, stable_coin, token_default}