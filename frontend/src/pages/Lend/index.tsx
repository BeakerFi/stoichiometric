/** @jsxImportSource theme-ui */

import { useContext, useEffect, useState, useRef } from "react";
import { useSearchParams } from "react-router-dom";

import { ResponsiveContext } from "contexts/ResponsiveContext";
import { UserContext } from "contexts/UserContext";
import { SnackbarContext } from "contexts/SnackbarContext";

import Star from "components/Star";

import Dashboard from "components/Dashboard";
import ConnectWallet2 from "components/ConnectWalletLarge";

import Snackbar from "components/Snackbar";
import { TokensContext } from "contexts/TokensContext";

import { formatToString, randomIntFromInterval } from "utils/general/generalMaths";
import {swap_direct} from "../../utils/dex/routerContractCalls";

import { stable_coin as stable, token_default} from "utils/general/constants";

import { getLenderInformation } from "utils/stablecoin/issuerApiCalls";

import { takeLoan } from "utils/stablecoin/issuerContractCalls";
import styleFunction from "./style";

import {token, pool} from "types";

function Swap() {

    let [searchParams, setSearchParams] = useSearchParams();

    const [stars, setStars] = useState(Array.from({length: 10}, (_, i) => [randomIntFromInterval(0,1), randomIntFromInterval(10,90), randomIntFromInterval(10,90), randomIntFromInterval(0,1)]));

    const { addAlert } = useContext(SnackbarContext);
    
    const { device } = useContext(ResponsiveContext);

    const { tokens, lenders } = useContext(TokensContext);

    const { user, tokensOwned, setNbTokens } = useContext(UserContext);

    const [tokensList, setTokensList] = useState(tokens.filter((x:token) => x.address != stable.address));

    const [token1Owned, setToken1Owned] = useState<"?" | number>("?");

    const [price, setPrice] = useState<number>(0);

    const [sent, setSent] = useState<number>(0);
    const [get, setGet] = useState<number>(0);

    const [dir, setDir] = useState<number>(0);

    function resetValues() {
        setSent(0);
        if (!lock) setGet(0);
    }

    const [token1, setToken1] = useState({name: "", symb: "", address: "", icon_url: ""});

    useEffect(() => {
        var tk1 = searchParams.get('tk1');

        if (!tk1) {
            setSearchParams({tk1: token_default.symb})
        }
    }, [])

    useEffect(() => {
        if (tokens) {
            var tk1 = searchParams.get('tk1');

            if (tk1) {
                tk1 = tk1.toLowerCase();
            }

            if (tk1 && tokens.map((x:token) => x.symb.toLowerCase()).includes(tk1)) {
                var tok1 = tokens.filter((x:token) => x.symb.toLowerCase() == tk1)[0]
                setToken1({name:tok1!.name, symb: tok1!.symb, address: tok1!.address, icon_url: tok1!.icon_url});
                setSearchParams({tk1: tk1!.toUpperCase()})
            } else {
                setToken1(token_default);
                setSearchParams({tk1: token_default.symb})
            }
        }

    }, [tokens])

    const token1AddressRef = useRef(token1.address)
    token1AddressRef.current = token1.address

    function calculateGet(n: number) {
        return (n*price)
    }

    function calculateSent(n: number) {
        if (price > 0) return (n/price);
        return 0;
    }

    function calculateMax(x: number | string) {
        if (typeof(x) == "string") return "?"
        if (isNaN(x)) return "?"
        if (price == 0) return "?"
        else {
            var s = calculateGet(x)
            if (isNaN(s)) return "?"
            else return formatToString(s)
        }
    }

    const sentChange = (event: any) => {
        var s = event.target.value;
        if (!isNaN(s)) {
            if (s.length == 0) {
                resetValues();
                return
            }
            if (s.includes(".")) {
                setSent(s);
                if (!lock) {
                    var x = calculateGet(parseFloat(s));
                    if (isNaN(x)) setGet(0);
                    else setGet(x);
                }
            } else {
                setSent(parseFloat(s));
                if (!lock) {
                    var x = calculateGet(parseFloat(s));
                    if (isNaN(x)) setGet(0);
                    else setGet(x);
                }
            }
        }
    }

    const lentChange = (event: any) => {
        var s = event.target.value;
        if (!isNaN(s)) {
            if (s.length == 0) {
                resetValues();
                return
            }
            if (s.includes(".")) {
                setGet(s);
                var x = calculateSent(parseFloat(s));
                if (isNaN(x)) setSent(0);
                else setSent(x);
            } else {
                setGet(parseFloat(s));
                var x = calculateSent(parseFloat(s));
                if (isNaN(x)) setSent(0);
                else setSent(x);
            }
        }
    }

    useEffect(() => {
        async function getPoolInfos() {
            setPrice(0);
            const infos = await getLenderInformation(lenders[token1.address]);
            if (infos) setPrice(infos["price"] * infos["loan_to_value"]);
            if (infos) setDir(infos["daily_interest_rate"]);
        }
        getPoolInfos();
    }, [token1, tokensOwned])
    useEffect(() => {
        const n = tokensOwned[token1.address];
        if (n == "undefined") setToken1Owned(0);
        else setToken1Owned(parseFloat(n));
    }, [tokensOwned, token1])

    useEffect(() => {
        resetValues();
    }, [token1])

    const [token1Select, setToken1Select] = useState(false);

    function resetSelect() {
        setSearch('');
        setToken1Select(false);
    }

    function selectToken(token: token) {
        if (token1Select) {
            setToken1(token)
            setSearchParams({tk1: token.symb.toUpperCase()})
        }
        resetSelect();
        resetValues();
    }

    const [search, setSearch] = useState("");

    function getSearch(list: token[]) {
        return list.filter(token => {
            if (token.address == stable.address) return false;
            var flag = (search.length == 0);
            for (const word of search.split(' ')) {
                if (word.length > 0) flag = flag || token.name.toLowerCase().includes(word) || token.symb.toLowerCase().includes(word)
            }
            return flag
        })
    }

    const searchChange = (event: any) => {
        setSearch(event.target.value.toLowerCase());
    }

    useEffect(() => {
        setTokensList(getSearch(tokens));
    }, [tokens, search])

    const [swapLoading, setSwapLoading] = useState(false);

    async function sendSwap() {
        setSwapLoading(true);
        const flag = await swap_direct(user.address, token1.address, stable.address, sent.toString())
        setNbTokens();
        resetValues();
        if (flag) {
            addAlert("check", "Transaction submitted!");
        } else {
            addAlert("error", "An error occured");
        }
        setSwapLoading(false);
    }

    const [lock, setLock] = useState<boolean>(false);

    function toggleLock() {
        setLock(!lock);
    }

    const [myLoans, setMyLoans] = useState<boolean>(false);

    const [choseLend, setChoseLend] = useState<boolean>(false);

    const [removePercentage, setRemovePercentage] = useState<number>(0);

    async function sendTakeLoan(account: string, token: string, amount: string, borrow: string) {
        setSwapLoading(true);
        const flag = await takeLoan(account, token, amount, borrow);
        setNbTokens();
        resetValues();
        if (flag) {
            addAlert("check", "Transaction submitted!");
        } else {
            addAlert("error", "An error occured");
        }
        setSwapLoading(false);
    }  

    const lendsList = [{token:{icon_url:"", symb:"XRD", address: "", name:""}, id:""}]

    const style = styleFunction(device, swapLoading, token1Select, choseLend, lock);

    return (
        <Dashboard page="lend">
            <Snackbar />

            {stars.map(x => { return (
                <Star left={x[1].toString()} top={x[2].toString()} height={x[0] ? "15" : "20"} color={x[3] ? "text" : "text2"}/>
            )})}

            <div sx={style.main}>


                <div sx={style.top}>

                    <div sx={style.container}>
                        <div sx={style.buttons}>
                            <span sx={myLoans && user.address ? style.inactive : style.active} onClick={() => {setMyLoans(false)}}>Borrow SUSD</span>
                            { user.address ?
                                <span sx={myLoans ? style.active : style.inactive} onClick={() => {setMyLoans(true);}}>My Loans</span>
                                : null}
                        </div>

                        { myLoans && user.address ? 
                            (<div sx={style.myPositionColumn}>
                                <div sx={style.chosePositionContainer}>
                                    <div sx={style.chosePositionZone}>
                                        <h2><div sx={style.close} onClick={() => setChoseLend(false)}/>Your Loans</h2>
                                        <div sx={style.poolsList}>
                                            {  lendsList.map((pool: pool) => {
                                                return (
                                                    <div sx={style.poolChoice} onClick={() => {
                                                        setChoseLend(true)
                                                    }}>
                                                        <img src={pool.token.icon_url}/>
                                                        <p>{pool.token.symb}</p>
                                                    </div>
                                                )
                                            })}
                                        </div>
                                    </div>
                                </div>
                                <div sx={style.chosePosition}  onClick={() => setChoseLend(true)}>
                                    <img src={token1.icon_url}/>
                                    <p>{token1.symb}</p>
                                    <div sx={style.expand2}/>
                                </div>
                                <div sx={style.swapZone}>
                                    <h1>🐰 Repay the Loan</h1>
                                    <div sx={style.swapInfos}>
                                        <span sx={style.swapInfoMain}><span>Total Locked</span>342 XRD</span>
                                        <span sx={style.swapInfo}><span>Total Borrowed</span>3.4 SUSD</span>
                                        <span sx={style.swapInfo}><span>Interest</span>0.04 SUSD</span>                                           
                                    </div>
                                    <button sx={false ? {...style.swapButton, ...style.swapButtonLoading} : style.swapButton}>{false ? "" : "Repay"}</button>

                                </div>
                                <div sx={style.swapZone}>
                                    <h1>🐷 Add Collateral</h1>
                                    <div sx={style.inputBar}>
                                    <input type="text" id="get" required={true} placeholder=" " autoComplete="off" onChange={lentChange} value={get} disabled={lock}/>
                                        <label htmlFor="get">{user.address ? `Adding ${calculateMax(token1Owned)} ${stable.symb}`: "You add"}</label>
                                        <div sx={style.token2}>
                                            <img src={token1.icon_url}/>
                                            <p>{token1.symb}</p>
                                        </div>
                                    </div>
                                    <span sx={style.tokenAddress}><span>Token Address</span>{token1.address.slice(0,5) + "..." + token1.address.slice(token1.address.length - 10, token1.address.length)}</span>

                                    <button sx={false ? {...style.swapButton, ...style.swapButtonLoading} : style.swapButton}>{false ? "" : "Add"}</button>
                                </div>
                                <div sx={style.swapZone}>
                                    <h1>🦊 Remove Collateral</h1>
                                    <div sx={style.rangeRow}>
                                            <input sx={style.range} type="range" id="remove" name="remove"
                                                min="0" max="100" value={removePercentage} onChange={(e) => {setRemovePercentage(Math.floor(parseFloat(e.target.value)))}}/>
                                            <p>{removePercentage}%</p>
                                        </div>
                                    <div sx={style.swapInfos}>
                                        <span sx={style.swapInfoMain}><span>Removing</span><div>? {token1.symb}</div></span>
                                        <span sx={style.swapInfo}><span>Value</span>$?</span>
                                    </div>
                                    <button sx={false ? {...style.swapButton, ...style.swapButtonLoading} : style.swapButton}>{false ? "" : "Remove"}</button>

                                </div>
                            </div>
                        ) : 
                            <div sx={style.swapZone}>
                                <h1>📝 Borrow SUSD</h1>

                                { lock ? 
                                    <div sx={style.alert}>
                                        <p>The minimum collateral needed is {price > 0 ? formatToString(get/price) : "?"} {token1.symb}</p>
                                    </div> 
                                    : null 
                                }
                                                            
                                <div sx={style.check}>
                                    <input type="checkbox" id="lock" onChange={toggleLock}/>
                                    <label htmlFor="lock">Lock the borrowed value</label>
                                </div>

                                <div sx={style.inputBar}>
                                    <input type="text" id="get" required={true} placeholder=" " autoComplete="off" onChange={lentChange} value={get} disabled={lock}/>
                                    <label htmlFor="get">{user.address ? `You can borrow ${calculateMax(token1Owned)} ${stable.symb}`: "You borrow"}</label>
                                    <div sx={style.token2}>
                                        <img src={stable.icon_url}/>
                                        <p>{stable.symb}</p>
                                    </div>
                                </div>
                                <span sx={style.tokenAddress}><span>Token Address</span>{stable.address.slice(0,5) + "..." + stable.address.slice(stable.address.length - 10, stable.address.length)}</span>

                                <div sx={style.inputBar}>
                                    <input type="text" id="send" required={true} placeholder=" " autoComplete="off" onChange={sentChange} value={sent}/>
                                    <label htmlFor="send">{user.address ? `You have ${token1Owned == "?" ? "?" : isNaN (token1Owned) ? "?" : formatToString(token1Owned)} ${token1.symb}`: "You lock"}</label>
                                    <div sx={style.token} onClick={() => setToken1Select(true)}>
                                        <img src={token1.icon_url}/>
                                        <p>{token1.symb}</p>
                                        <div sx={style.expand}/>
                                    </div>
                                </div>
                                <span sx={style.tokenAddress}><span>Token Address</span>{token1.address.slice(0,5) + "..." + token1.address.slice(token1.address.length - 10, token1.address.length)}</span>
                                
                                <div sx={style.swapInfos}>
                                    <span sx={style.swapInfoMain}><span>Lend</span><div>{typeof(sent) == "string" ? formatToString(parseFloat(sent)) : formatToString(sent)} {token1.symb}<div/>{typeof(get) == "string" ? formatToString(parseFloat(get)) : formatToString(get)} {stable.symb}</div></span>
                                    <span sx={style.swapInfo}><span>LTV</span>1 {token1.symb} = {price == 0 ? "?" : sent == 0 ? formatToString(price) : formatToString(get/sent)} {stable.symb}</span>
                                    <span sx={style.swapInfo}><span>Daily Interest Rate</span>{dir}</span>
                                </div>

                                {
                                    user.address ? 
                                    <button sx={swapLoading ? {...style.swapButton, ...style.swapButtonLoading} : style.swapButton} onClick={() => swapLoading ? null : sendTakeLoan(user.address, token1.address, sent.toString(), get.toString())}>{swapLoading ? "" : "Lend"}</button>
                                    : 
                                    <ConnectWallet2 />
                                }


                                <div sx={style.selectToken}>
                                    <h2><div sx={style.close} onClick={resetSelect}/>Select Currency</h2>
                                    <div sx={style.inputBar}>
                                        <input type="text" id="search" required={true} placeholder=" " autoComplete="off" onChange={searchChange} value={search}/>
                                        <label htmlFor="search">Search for a token</label>
                                    </div>

                                    <div sx={style.tokensList}>
                                        {   tokensList.map((token: token) => {
                                            return (
                                                <div sx={style.tokenChoice} onClick={() => selectToken(token)}>
                                                    <img src={token.icon_url}/>
                                                    <p>{token.name}<span>{token.symb}</span></p>
                                                </div>
                                            )
                                        })}
                                    </div>
                                </div>
                            </div>
                        }

                    </div>
                </div>
            </div>
        </Dashboard>
    )
}

export default Swap;