/** @jsxImportSource theme-ui */

import { useContext, useEffect, useState, useRef } from "react";
import { useSearchParams } from "react-router-dom";

import { ResponsiveContext } from "contexts/ResponsiveContext";
import { UserContext } from "contexts/UserContext";
import { SnackbarContext } from "contexts/SnackbarContext";

import Star from "components/Star";

import Dashboard from "components/Dashboard";
import ConnectWallet2 from "components/ConnectWallet2";

import Snackbar from "components/Snackbar";
import { TokensContext } from "contexts/TokensContext";

import { formatToString, formatToString2, randomIntFromInterval } from "utils/general/generalMaths";
import {swap_direct} from "../utils/dex/routerContractCalls";

import { stable_coin as stable, token_default} from "utils/general/constants";

import { getLenderInformation } from "utils/stablecoin/issuerApiCalls";

function Swap() {

    let [searchParams, setSearchParams] = useSearchParams();

    const [stars, setStars] = useState(Array.from({length: 10}, (_, i) => [randomIntFromInterval(0,1), randomIntFromInterval(10,90), randomIntFromInterval(10,90), randomIntFromInterval(0,1)]));

    const { addAlert } = useContext(SnackbarContext);
    
    const { device, windowSize } = useContext(ResponsiveContext);

    const { tokens } = useContext(TokensContext);

    const { user, tokensOwned, setNbTokens } = useContext(UserContext);

    const [category, setCategory] = useState("all");

    const [tokensList, setTokensList] = useState(tokens.filter((x:any) => x.address != stable.address));

    const [token1Owned, setToken1Owned] = useState<"?" | number>("?");

    const [price, setPrice] = useState<number>(0);

    const [sent, setSent] = useState<number>(0);
    const [get, setGet] = useState<number>(0);

    const [priceImpact, setPriceImpact] = useState("0");

    function resetValues() {
        setSent(0);
        if (!lock) setGet(0);
        setPriceImpact("0");
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

            if (tk1 && tokens.map((x:any) => x.symb.toLowerCase()).includes(tk1)) {
                var tok1 = tokens.filter((x:any) => x.symb.toLowerCase() == tk1)[0]
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

    const [token1InPool, setToken1InPool] = useState(0);
    const [stableInPool, setStableInPool] = useState(0);

    function calculateGet(n: number) {
        return (stableInPool*n/(n + token1InPool))
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
                var x = calculateGet(parseFloat(s));
                if (isNaN(x)) setSent(0);
                else setSent(x);
            } else {
                setGet(parseFloat(s));
                var x = calculateGet(parseFloat(s));
                if (isNaN(x)) setSent(0);
                else setSent(x);
            }
        }
    }

    useEffect(() => {
        const n = 100*Math.abs(
            (stableInPool/(stableInPool - get))*(sent + token1InPool)/token1InPool-1
        )
        if (isNaN(n)) setPriceImpact("?")
        else setPriceImpact(formatToString2(n))
    }, [sent, get])

    useEffect(() => {
        async function getPoolInfos() {
            setPrice(0);
            const infos = await getLenderInformation(token1.address);
            console.log(infos)
            setPrice(infos["price"]);
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

    function selectToken(token: any) {
        if (token1Select) {
            setToken1(token)
            setSearchParams({tk1: token.symb.toUpperCase()})
        }
        resetSelect();
        resetValues();
    }

    const [search, setSearch] = useState("");

    function getSearch(list: any[]) {
        return list.filter(x => {
            if (x.address == stable.address) return false;
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
                },
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
                cursor: `${lock ? 'not-allowed' : 'text'}`
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

            '&:nth-of-type(1)': {
                marginBottom: '25px'
            },

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
            left: `${token1Select? '0' : '100%'}`,
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
        },

        check: {
            display: 'flex',
            alignItems: 'center',
            color: 'text',
            fontFamily: 'primary',
            fontSize: 1,
            transform: 'TranslateY(-15px)',
            width: '100%',

            '& *': {
                cursor: 'pointer'
            }
        },

        alert :{
            width: '60%',
            border: 'solid 1px',
            borderColor: 'red',
            borderRadius: '5px',
            padding: '5px 15px',

            marginBottom: '20px',

            '& p': {
                margin: '0',
                padding: '0',
                fontSize: 0,
                fontFamily: 'primary',
                color: 'red',
                textAlign: 'center' as 'center'
            }
        },

        buttons: {
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
            width: '100%',
            marginBottom: '10px',

            '& span': {
                fontFamily: 'primary',
                fontSize: 1,
                borderRadius: '5px',
                padding: '5px 10px',
            }
        },

        active: {
            background: 'background2',
            color: 'text',
        },

        inactive: {
            color: 'text2',
            cursor: 'pointer',
            '&: hover': {
                background: 'background2',
            }
        },

        myPositionColumn: {
            position: 'relative' as 'relative',
        },

        chosePosition: {
            padding: '20px',
            height: 'auto',
            width: 'calc(100% - 40px)',
            background: 'background2',
            color: 'shadow',
            boxShadow: '0px 1px 4px',
            borderRadius: '5px',
            display: 'flex',
            alignItems: 'center',
            overflow: "hidden",
            position: 'relative' as 'relative',
            marginBottom: '20px',
            cursor: 'pointer',

            '& p': {
                margin: '0',
                padding: '0',
                color: 'text',
                fontFamily: 'primary',
                fontSize: 1,
                fontWeight: '500'
            },

            '& img': {
                height: '25px',
                aspectRatio: '1',
                borderRadius: '1000px',
                objectFit: 'contain' as 'contain',
                marginRight: '10px',
                '&:nth-child(1)': {
                    transform: 'Translate(0, -3px)'
                },
                '&:nth-child(2)': {
                    transform: 'Translate(-15px, 3px)',
                    marginRight: '0px',
                }
            },

            '&:hover div': {
                opacity: 1
            }
        },

        chosePositionZone: {
            position: 'absolute' as 'absolute',
            top: `0px`,
            left: `${choseLend ? '0px' : '100%'}`,
            height: 'calc(100% - 60px)',
            width: 'calc(100% - 40px)',
            padding: '20px',
            background: 'background2',
            color: 'shadow',
            borderRadius: '5px',
            display: 'flex',
            alignItems: 'center',
            cursor: 'default',
            zIndex: '1000',
            transition: '.2s',
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

        chosePositionContainer: {
            height: '100%',
            width: '100%',
            position: 'absolute' as 'absolute',
            left: 0,
            top: 0,
            overflow: 'hidden',
        },

        poolChoice: {
            display: 'flex',
            width: 'calc(100% - 40px)',
            padding: '15px 20px',
            alignItems: 'center',
            cursor: 'pointer',
            '&: hover': {
                background: 'background3'
            },
            '& p': {
                margin: '0',
                padding: '0',
                color: 'text',
                fontFamily: 'primary',
                fontSize: 1,
                fontWeight: '500'
            },

            '& img': {
                height: '25px',
                aspectRatio: '1',
                borderRadius: '1000px',
                objectFit: 'contain' as 'contain',
                marginRight: '10px',
                '&:nth-child(1)': {
                    transform: 'Translate(0, -3px)'
                },
                '&:nth-child(2)': {
                    transform: 'Translate(-15px, 3px)',
                    marginRight: '0px',
                }
            },
        },

        expand2: {
            height: '12px',
            aspectRatio: '1',
            background: 'text',
            opacity: '.3',
            "-webkit-mask":`url('/icons/expand.svg') center/contain no-repeat`,
                        mask:`url('/icons/expand.svg') center/contain no-repeat`,
            transform: 'TranslateY(-50%)',
            position: 'absolute' as 'absolute',
            right: '20px',
            top: '50%',
        },

        poolsList: {
            overflow: 'scroll',
            '-ms-overflow-style': 'none',
            'scrollbar-width': 'none',
            '::-webkit-scrollbar': {
                display: 'none'
            },
            width: '100%',
        },

        rangeRow: {
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'space-between',
            width: '100%',
            marginBottom: '20px',

            '& p': {
                margin: '0',
                padding: '0',
                color: 'text',
                fontFamily: 'primary',
                fontSize: 1,
            },
        },

        range: {
            width: '90%',
            '-webkit-appearance': 'none',
            background: 'transparent',
            height: '100%',

            '&:hover::-webkit-slider-thumb': {
                background: 'primary'
            },

            '&:focus': {
                outline: 'none'
            },

            '&::-webkit-slider-runnable-track': {
                width: '100%',
                height: '5px',
                cursor: 'pointer',
                animate: '0.2s',
                color: 'shadow',
                boxShadow: 'none',
                background: 'background3',
                borderRadius: '50px',
                border: 'none',
              },

            '&::-webkit-slider-thumb': {
                boxShadow: '0px 1px 4px',
                border: 'none',
                height: '20px',
                width: '20px',
                borderRadius: '50px',
                background: 'background3',
                cursor: 'pointer',
                '-webkit-appearance': 'none',
                marginTop: '-7.5px',
              }
        }

    }  

    const lendsList = [{token:{icon_url:"", symb:"XRD"}}]

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
                                            {  lendsList.map((pool: any) => {
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
                        )


                        :

                        <div sx={style.swapZone}>
                            <h1>📝 Borrow SUSD</h1>

                            { lock ? 
                                <div sx={style.alert}>
                                    <p>The minimum collateral needed is 457465.45 XRD</p>
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
                                <span sx={style.swapInfo}><span>Daily Interest Rate</span>0.3%</span>
                            </div>

                            {
                                user.address ? 
                                <button sx={swapLoading ? {...style.swapButton, ...style.swapButtonLoading} : style.swapButton} onClick={() => swapLoading ? null : sendSwap()}>{swapLoading ? "" : "Lend"}</button>
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
                        </div>}
                    </div>
                </div>
            </div>
        </Dashboard>
    )
}

export default Swap;