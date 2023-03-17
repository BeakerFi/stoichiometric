/** @jsxImportSource theme-ui */

import { useContext, useEffect, useState, useRef } from "react";
import { useSearchParams } from "react-router-dom";

import { ResponsiveContext } from "contexts/ResponsiveContext";
import { UserContext } from "contexts/UserContext";
import { SnackbarContext } from "contexts/SnackbarContext";

import { getPrice } from "utils/connectToBackend";

import Star from "components/Star";

import Dashboard from "components/Dashboard";
import ConnectWallet2 from "components/ConnectWallet2";

import Snackbar from "components/Snackbar";
import { TokensContext } from "contexts/TokensContext";

import { formatToString, formatToString2, randomIntFromInterval } from "utils/maths";
import {swap_direct} from "../utils/routerCalls";

const stable = {name: "Stoichiometric USD", symb: "SUSD", address: "resource_tdx_b_arthurjetebaisegrosfdp111111fdpputeputeshitcoin", icon_url: "https://upload.wikimedia.org/wikipedia/commons/thumb/4/46/Bitcoin.svg/1024px-Bitcoin.svg.png"};

function Swap() {

    let [searchParams, setSearchParams] = useSearchParams();

    const [stars, setStars] = useState(Array.from({length: 10}, (_, i) => [randomIntFromInterval(0,1), randomIntFromInterval(10,90), randomIntFromInterval(10,90), randomIntFromInterval(0,1)]));


    const { addAlert } = useContext(SnackbarContext);
    
    const { device, windowSize } = useContext(ResponsiveContext);

    const { tokens } = useContext(TokensContext);

    const { user, tokensOwned, setNbTokens } = useContext(UserContext);

    const [category, setCategory] = useState("all");

    const [tokensList, setTokensList] = useState(tokens);

    const [token1Owned, setToken1Owned] = useState<"?" | number>("?");

    const [price, setPrice] = useState<number>(0);

    const [sent, setSent] = useState<number>(0);
    const [get, setGet] = useState<number>(0);

    const [priceImpact, setPriceImpact] = useState("0");

    function resetValues() {
        setSent(0);
        setGet(0);
        setPriceImpact("0");
    }

    const [token1, setToken1] = useState({name: "", symb: "", address: "", icon_url: ""});
    const [token2, setToken2] = useState({name: "", symb: "", address: "", icon_url: ""});

    useEffect(() => {
        var tk1 = searchParams.get('tk1');
        var tk2 = searchParams.get('tk2');

        if (!tk1 || !tk2) {
            setSearchParams({tk1: 'XRD', tk2: 'WBTC'})
        }
    }, [])

    useEffect(() => {
        if (tokens) {
            var tk1 = searchParams.get('tk1');
            var tk2 = searchParams.get('tk2');

            if (tk1 && tk2) {
                tk1 = tk1.toLowerCase();
                tk2 = tk2.toLowerCase();
            }

            if (tk1 != tk2 && tokens.map((x:any) => x.symb.toLowerCase()).includes(tk1) && tokens.map((x:any) => x.symb.toLowerCase()).includes(tk2)) {
                var tok1 = tokens.filter((x:any) => x.symb.toLowerCase() == tk1)[0]
                var tok2 = tokens.filter((x:any) => x.symb.toLowerCase() == tk2)[0]
                setToken1({name:tok1!.name, symb: tok1!.symb, address: tok1!.address, icon_url: tok1!.icon_url});
                setToken2({name:tok2!.name, symb: tok2!.symb, address: tok2!.address, icon_url: tok2!.icon_url});
                setSearchParams({tk1: tk1!.toUpperCase(), tk2: tk2!.toUpperCase()})
            } else {
                setToken1({name: "Radix", symb: "XRD", address: "resource_tdx_b_1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq8z96qp", icon_url: "https://switchere.com/i/currencies/XRD.svg"});
                setToken2({name: "Wrapped Bitcoin", symb: "WBTC", address: "resource_tdx_b_1qre9sv98scqut4k9g3j6kxuvscczv0lzumefwgwhuf6qdu4c3r", icon_url: "https://upload.wikimedia.org/wikipedia/commons/thumb/4/46/Bitcoin.svg/1024px-Bitcoin.svg.png"});
                setSearchParams({tk1: 'XRD', tk2: 'WBTC'})
            }
        }

    }, [tokens])

    const token1AddressRef = useRef(token1.address)
    token1AddressRef.current = token1.address

    const token2AddressRef = useRef(token2.address)
    token2AddressRef.current = token2.address

    const [token1InPool, setToken1InPool] = useState(0);
    const [token2InPool, setToken2InPool] = useState(0);

    function calculateGet(n: number) {
        return (token2InPool*n/(n + token1InPool))
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
                var x = calculateGet(parseFloat(s));
                if (isNaN(x)) setGet(0);
                else setGet(x);
            } else {
                setSent(parseFloat(s));
                var x = calculateGet(parseFloat(s));
                if (isNaN(x)) setGet(0);
                else setGet(x);
            }
        }
    }

    useEffect(() => {
        const n = 100*Math.abs(
            (token2InPool/(token2InPool - get))*(sent + token1InPool)/token1InPool-1
        )
        if (isNaN(n)) setPriceImpact("?")
        else setPriceImpact(formatToString2(n))
    }, [sent, get])

    useEffect(() => {
        async function getPoolInfos() {
            setPrice(0);
            if(token1.address && token2.address) {
                const infos = await getPrice(token1.address, token2.address);
                if (infos && infos!['token1_address'] == token1AddressRef.current && infos!['token2_address'] == token2AddressRef.current) {
                    if (infos!['token1_amount'] > 0) {
                        setToken1InPool(parseFloat(infos!["token1_amount"]));
                        setToken2InPool(parseFloat(infos!["token2_amount"]));
                        setPrice(infos!["token2_amount"]/infos!["token1_amount"]);
                    }
                }
            }
        }
        getPoolInfos();
    }, [token1, token2, tokensOwned])
    useEffect(() => {
        const n = tokensOwned[token1.address];
        if (n == "undefined") setToken1Owned(0);
        else setToken1Owned(parseFloat(n));
    }, [tokensOwned, token1])
    
    function invert() {
        const temp = {...token2};
        setToken2(token1);
        setToken1(temp);
        var tk1 = searchParams.get('tk1')!.toUpperCase();
        var tk2 = searchParams.get('tk2')!.toUpperCase();
        setSearchParams({tk1: tk2, tk2: tk1})
    }

    useEffect(() => {
        resetValues();
    }, [token1, token2])

    const [token1Select, setToken1Select] = useState(false);
    const [token2Select, setToken2Select] = useState(false);

    function resetSelect() {
        setSearch('');
        setToken1Select(false);
        setToken2Select(false);
    }

    function selectToken(token: any) {
        if (token1Select) {
            if (token.address == token2.address) {
                invert()
            }
            else { 
                setToken1(token)
                var tk2 = searchParams.get('tk2')!.toUpperCase();
                setSearchParams({tk1: token.symb.toUpperCase(), tk2: tk2})
            }
        }
        if (token2Select) {
            if (token.address == token1.address) {
                invert()
            }
            else { 
                setToken2(token);
                var tk1 = searchParams.get('tk1')!.toUpperCase();
                setSearchParams({tk1: tk1, tk2: token.symb.toUpperCase()})
            }
        }
        resetSelect();
        resetValues();
    }

    const [search, setSearch] = useState("");

    function getSearch(list: any[]) {
        return list.filter(x => {
            if (category != "all" && x.category != category) return false;
            var flag = (search.length == 0);
            for (const word of search.split(' ')) {
                if (word.length > 0) flag = flag || x.name.toLowerCase().includes(word) || x.symb.toLowerCase().includes(word)
            }
            return flag
        })
    }

    const searchChange = (event: any) => {
        setSearch(event.target.value.toLowerCase());
    }

    useEffect(() => {
        setTokensList(getSearch(tokens));
    }, [tokens, search, category])

    const [swapLoading, setSwapLoading] = useState(false);

    async function sendSwap() {
        setSwapLoading(true);
        const flag = await swap_direct(user.address, token1.address, token2.address, sent.toString())
        setNbTokens();
        resetValues();
        if (flag) {
            addAlert("check", "Transaction submitted!");
        } else {
            addAlert("error", "An error occured");
        }
        setSwapLoading(false);
    }

    const style = {
        main: {
            display: 'flex',
            justifyContent: 'space-between',
            position: 'absolute' as 'absolute',
            left: `${device == "mobile" ? "10px" : device == "tablet" ? "115px" : '170px'}`,
            top: `${device == "mobile" ? "95px" : "30px"}`,
            height: `${device == "mobile" ? "calc(100% - 105px)" : "calc(100% - 60px)"}`,
            width: `${device == "mobile" ? "calc(100% - 20px)" : device == "tablet" ? "calc(100% - 135px)" : 'calc(100% - 190px)'}`,
        },

        top: {
            height: '100%',
            width: '100%',
            position: 'relative' as 'relative',
            overflow: 'scroll',
            '-ms-overflow-style': 'none',
            'scrollbar-width': 'none',
            '::-webkit-scrollbar': {
                display: 'none'
            }
        },

        container: {
            width: `${device == "mobile" ? '100%' : '510px'}`,
            position: 'absolute' as 'absolute',
            left: '50%',
            top: `${device == "mobile" ? '0px' : '50%'}`,
            transform: `${device == "mobile" ? 'TranslateX(-50%)' : 'Translate(-50%, -50%)'}`,
            display: 'flex',
            flexDirection: 'column' as 'column'
        },

        swapZone: {
            padding: '20px',
            height: 'auto',
            width: 'calc(100% - 40px)',
            background: 'background2',
            color: 'shadow',
            boxShadow: '0px 1px 4px',
            borderRadius: '5px',
            display: 'flex',
            flexDirection: 'column' as 'column',
            alignItems: 'center',
            overflow: "hidden",
            position: 'relative' as 'relative',
            marginBottom: '20px',

            '& h1': {
                margin: '0',
                padding: '0',
                width: '100%',
                color: 'text',
                fontSize: 4,
                fontWeight: '600',
                marginBottom: '20px'
            }
        },

        inputBar: {
            position: 'relative' as 'relative',
            width: '100%',
            marginBottom: '5px',

            '& input': {
                fontFamily: 'primary',
                fontSize: 2,
                width: 'calc(100% - 140px - 2px)',
                padding: '10px',
                paddingRight: '130px',
                margin: '0',
                border: 'solid 1px',
                borderColor: 'background3',
                background: 'background2',
                borderRadius: '5px',
                color: 'text',
    
                '&:focus': {
                    outline: 'none',
                    borderColor: 'text',
    
                    '& ~ label': {
                        left: '10px',
                        top: '0',
                        fontSize: 1,
                        transform: 'TranslateY(-50%)',
                        color: 'text'
                    }
                },
    
                '&:not(:placeholder-shown) ~ label': {
                    left: '10px',
                    top: '0',
                    fontSize: 1,
                    transform: 'TranslateY(-50%)',
                }
            },
    
            '& label': {
                position: 'absolute' as 'absolute',
                left: '20px',
                top: '50%',
                transform: 'TranslateY(-50%)',
                fontFamily: 'primary',
                fontSize: 2,
                zIndex: '10',
                padding: '0 5px',
                transition: '.1s',
                transitionProperty: 'left, top',
                cursor: 'text',
                background: 'background2',
                color: 'text2',
            }, 

            '& #get ~ label, #get': {
                cursor: 'not-allowed'
            }
        },

        swapIcon: {
            height: '20px',
            marginBottom: '20px',
            cursor: 'pointer',
            aspectRatio: '1',
            opacity: '.3',
            transform: 'Rotate(90deg)',
            background: 'text',
            "-webkit-mask":`url('/icons/swap.svg') center/contain no-repeat`,
                        mask:`url('/icons/swap.svg') center/contain no-repeat`,
            '&:hover': {
                opacity: '1'
            }
        },

        swapIcon2: {
            height: '20px',
            marginRight: '20px',
            cursor: 'pointer',
            aspectRatio: '1',
            opacity: '.3',
            transform: 'Rotate(90deg)',
            background: 'text',
            "-webkit-mask":`url('/icons/swap.svg') center/contain no-repeat`,
                        mask:`url('/icons/swap.svg') center/contain no-repeat`,
            '&:hover': {
                opacity: '1'
            }
        },

        token: {
            width: '110px',
            height: '100%',
            position: 'absolute' as 'absolute',
            right: '10px',
            top: '0',
            cursor: 'pointer',

            display: 'flex',
            alignItems: 'center',

            '& img': {
                height: '50%',
                aspectRatio: '1',
                borderRadius: '1000px',
                objectFit: 'contain' as 'contain',
                marginRight: '10px'
            },

            '& p': {
                margin: '0',
                padding: '0',
                color: 'text',
                fontFamily: 'primary',
                fontSize: 2,
                fontWeight: '500'
            },
            '&:hover div': {
                opacity: '1'
            }
        },

        token2: {
            width: '110px',
            height: '100%',
            position: 'absolute' as 'absolute',
            right: '10px',
            top: '0',
            cursor: 'not-allowed',

            display: 'flex',
            alignItems: 'center',

            '& img': {
                height: '50%',
                aspectRatio: '1',
                borderRadius: '1000px',
                objectFit: 'contain' as 'contain',
                marginRight: '10px'
            },

            '& p': {
                margin: '0',
                padding: '0',
                color: 'text',
                fontFamily: 'primary',
                fontSize: 2,
                fontWeight: '500'
            },
        },

        expand: {
            height: '12px',
            aspectRatio: '1',
            background: 'text',
            opacity: '.3',
            "-webkit-mask":`url('/icons/expand.svg') center/contain no-repeat`,
                        mask:`url('/icons/expand.svg') center/contain no-repeat`,
            transform: 'TranslateY(-50%)',
            position: 'absolute' as 'absolute',
            right: '0',
            top: '50%',
        },

        close: {
            height: '17px',
            aspectRatio: '1',
            background: 'text',
            opacity: '.3',
            "-webkit-mask":`url('/icons/expand.svg') center/contain no-repeat`,
                        mask:`url('/icons/expand.svg') center/contain no-repeat`,
            transform: 'TranslateY(-50%) Rotate(180deg)',
            zIndex: '25',
            cursor: 'pointer',
            position: 'absolute' as 'absolute',
            left: '0',
            top: '50%',
            '&:hover': {
                opacity: '1'
            }
        },

        tokenAddress: {
            color: 'text2',
            fontFamily: 'primary',
            marginBottom: '10px',
            fontSize: 0,
            width: 'calc(100% - 10px)',
            display: 'flex',
            justifyContent: 'space-between',
            overflow: 'hidden',


            '& span': {
                color: 'text2',
                fontWeight: '500',
                whiteSpace: 'nowrap' as 'nowrap',
                marginRight: '20px',
            }
        },

        swapInfos: {
            width: '100%',
            display: 'flex',
            flexDirection: 'column' as 'column',
            marginTop: '10px',
            marginBottom: '20px'
        },

        swapInfoMain: {
            color: 'text',
            fontFamily: 'primary',
            width: '100%',
            fontSize: 1,
            fontWeight: '600',
            display: 'flex',
            justifyContent: 'space-between',

            '& div': {
                display: 'flex',
                alignItems: 'center'
            },

            '& div div': {
                height: '15px',
                transform: 'Rotate(90deg)',
                margin: '0 5px',
                aspectRatio: '1',
                background: 'text',"-webkit-mask":`url('/icons/smallArrow.svg') center/contain no-repeat`,
                mask:`url('/icons/smallArrow.svg') center/contain no-repeat`,
            }
        },

        swapInfo: {
            color: 'text2',
            fontFamily: 'primary',
            width: '100%',
            fontSize: 0,
            fontWeight: '400',
            display: 'flex',
            justifyContent: 'space-between',
            marginTop: '5px',
        },
        
        swapButton: {
            background: 'primary',
            border: 'none',
            color: 'white',
            borderRadius: '10px',
            width: '150px',
            height: '50px',
            fontFamily: 'primary',
            fontSize: 2,
            cursor: 'pointer', 
            '&:hover': {
                opacity: `${swapLoading ? '1' : '.8'}`
            }
        },

        swapButtonLoading: {
            background: 'primary',
            cursor: 'default !important',
            display: 'flex',
            justifyContent: 'center',
            alignItems: 'center',
            '&::after': {
                content: '""',
                border: 'solid 3px',
                borderLeftColor: 'primary',
                borderRadius: '1000px',
                height: '30% !important',
                aspectRatio: '1',
                background: 'transparent',
                animation: 'rotating 1s linear infinite',
            },
        },

        selectToken: {
            height: 'calc(100% - 40px)',
            width: 'calc(100% - 40px)',
            padding: '20px',
            background: 'background2',
            position: 'absolute' as 'absolute',
            left: `${token1Select || token2Select ? '0' : '100%'}`,
            transition: '.2s',
            top: '0',
            zIndex: '20',
            display: 'flex',
            flexDirection: 'column' as 'column',

            '& h2': {
                margin: '0',
                padding: '0',
                color: 'text',
                fontFamily: 'primary',
                fontSize: 2,
                fontWeight: '600',
                width: '100%',
                textAlign: 'center' as 'center',
                position: 'relative' as 'relative',
                marginBottom: '20px'
            }
        },

        tokensList: {
            overflow: 'scroll',
            '-ms-overflow-style': 'none',
            'scrollbar-width': 'none',
            '::-webkit-scrollbar': {
                display: 'none'
            }
        },

        tokenChoice: {
            display: 'flex',
            width: 'calc(100% - 40px)',
            padding: '15px 20px',
            alignItems: 'center',
            cursor: 'pointer',
            '&: hover': {
                background: 'background3'
            },
            '& img': {
                height: '20px',
                aspectRatio: '1',
                objectFit: 'contain' as 'contain',
                borderRadius: '1000px',
                background: 'almostTransparent',
                marginRight: '15px'
            },
            '& p': {
                margin: '0',
                padding: '0',
                color: 'text',
                fontFamily: 'primary',
                fontWeight: '500',
                fontSize: 1,

                '& span': {
                    opacity: '.3',
                    marginLeft: '15px'
                }
            }
        },

        categories: {
            width: '100%',
            display: 'flex',
            alignItems: 'center',
            marginBottom: '10px',
            marginTop: '5px',

            '& span': {
                borderRadius: '5px',
                marginRight: '10px',
                padding: '5px 15px',
                fontFamily: 'primary',
                fontSize: 1,
                cursor: 'pointer',
    
                '&:hover': {
                    color: 'text'
                }
            }
        },

        activeCategory: {
            background: 'background3',
            color: 'text',
            cursor: 'default !important'
        },
    
        inactiveCategory: {
            background: 'background2',
            color: 'text2',
        },

        stableBarContainer: {
            width: '100%',
            display: 'flex',
            alignItems: 'center',
            marginTop: '20px',
            marginBottom: '20px',
        },

        stableBar: {
            width: '100%'
        }
    }  

    return (
        <Dashboard page="swap">
            <Snackbar />

            {stars.map(x => { return (
                <Star left={x[1].toString()} top={x[2].toString()} height={x[0] ? "15" : "20"} color={x[3] ? "text" : "text2"}/>
            )})}

            <div sx={style.main}>
                <div sx={style.top}>
                    <div sx={style.container}>
                        <div sx={style.swapZone}>
                            <h1>🏛 Swap Tokens</h1>
                            <div sx={style.inputBar}>
                                <input type="text" id="send" required={true} placeholder=" " autoComplete="off" onChange={sentChange} value={sent}/>
                                <label htmlFor="send">{user.address ? `You have ${token1Owned == "?" ? "?" : isNaN (token1Owned) ? "?" : formatToString(token1Owned)} ${token1.symb}`: "You send"}</label>
                                <div sx={style.token} onClick={() => setToken1Select(true)}>
                                    <img src={token1.icon_url}/>
                                    <p>{token1.symb}</p>
                                    <div sx={style.expand}/>
                                </div>
                            </div>
                            <span sx={style.tokenAddress}><span>Token Address</span>{token1.address.slice(0,5) + "..." + token1.address.slice(token1.address.length - 10, token1.address.length)}</span>
                            {(token1.address!=stable.address && token2.address!=stable.address) ?
                                <div sx={style.stableBarContainer}>
                                    <div sx={style.swapIcon2} onClick={invert}/>
                                    <div sx={style.stableBar}>
                                        <div sx={style.inputBar}>
                                            <input type="text" id="get" required={true} placeholder=" " autoComplete="off" disabled value={get}/>
                                            <label htmlFor="get">{user.address ? `Intermediate transaction`: "You get"}</label>
                                            <div sx={style.token2}>
                                                <img src={stable.icon_url}/>
                                                <p>{stable.symb}</p>
                                            </div>
                                        </div>
                                        <span sx={style.tokenAddress}><span>Token Address</span>{stable.address.slice(0,5) + "..." + stable.address.slice(stable.address.length - 10, stable.address.length)}</span>
                                    </div>
                                </div>
                                : <div sx={style.swapIcon} onClick={invert}/>

                            }
                            <div sx={style.inputBar}>
                                <input type="text" id="get" required={true} placeholder=" " autoComplete="off" disabled value={get}/>
                                <label htmlFor="get">{user.address ? `You can buy ${calculateMax(token1Owned)} ${token2.symb}`: "You get"}</label>
                                <div sx={style.token} onClick={() => setToken2Select(true)}>
                                    <img src={token2.icon_url}/>
                                    <p>{token2.symb}</p>
                                    <div sx={style.expand}/>
                                </div>
                            </div>
                            <span sx={style.tokenAddress}><span>Token Address</span>{token2.address.slice(0,5) + "..." + token2.address.slice(token2.address.length - 10, token2.address.length)}</span>
                            <div sx={style.swapInfos}>
                                <span sx={style.swapInfoMain}><span>Purchase</span><div>{typeof(sent) == "string" ? formatToString(parseFloat(sent)) : formatToString(sent)} {token1.symb}<div/>{typeof(get) == "string" ? formatToString(parseFloat(get)) : formatToString(get)} {token2.symb}</div></span>
                                <span sx={style.swapInfo}><span>Price</span>1 {token1.symb} = {price == 0 ? "?" : sent == 0 ? formatToString(price) : formatToString(get/sent)} {token2.symb}</span>
                                <span sx={style.swapInfo}><span>Price Impact</span>{priceImpact}%</span>
                                <span sx={style.swapInfo}><span>Pool Fees</span>0.3%</span>
                            </div>

                            {
                                user.address ? 
                                <button sx={swapLoading ? {...style.swapButton, ...style.swapButtonLoading} : style.swapButton} onClick={() => swapLoading ? null : sendSwap()}>{swapLoading ? "" : "Swap"}</button>
                                : 
                                <ConnectWallet2 />
                            }


                            <div sx={style.selectToken}>
                                <h2><div sx={style.close} onClick={resetSelect}/>Select Currency</h2>
                                <div sx={style.inputBar}>
                                    <input type="text" id="search" required={true} placeholder=" " autoComplete="off" onChange={searchChange} value={search}/>
                                    <label htmlFor="search">Search for a token</label>
                                </div>
                                <div sx={style.categories}>
                                    <span sx={category == "all" ? style.activeCategory : style.inactiveCategory} onClick={() => setCategory("all")}>All</span>
                                    <span sx={category == "Token" ? style.activeCategory : style.inactiveCategory} onClick={() => setCategory("Token")}>Tokens</span>
                                    <span sx={category == "Stable Coin" ? style.activeCategory : style.inactiveCategory} onClick={() => setCategory("Stable Coin")}>StableCoins</span>
                                </div>
                                <div sx={style.tokensList}>
                                    {   tokensList.map((token: any) => {
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
                    </div>
                </div>
            </div>
        </Dashboard>
    )
}

export default Swap;