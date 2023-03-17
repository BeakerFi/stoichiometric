/** @jsxImportSource theme-ui */

import { useContext, useEffect, useState, useRef } from "react";
import { useSearchParams } from "react-router-dom";

import { ResponsiveContext } from "contexts/ResponsiveContext";
import { UserContext } from "contexts/UserContext";
import { SnackbarContext } from "contexts/SnackbarContext";

import { getPrice, getPositionInfos } from "utils/connectToBackend";

import Star from "components/Star";

import Dashboard from "components/Dashboard";
import ConnectWallet2 from "components/ConnectWallet2";

import Snackbar from "components/Snackbar";
import { TokensContext } from "contexts/TokensContext";

import { formatToString, formatToString2, nFormatter, randomIntFromInterval, twoDecimals } from "utils/maths";
import { createPosition, addToPosition, claimFees, removeLiquidity, closePosition } from "utils/connectToWallet";

function Liquidity() {

    let [searchParams, setSearchParams] = useSearchParams();

    const [stars, setStars] = useState(Array.from({length: 10}, (_, i) => [randomIntFromInterval(0,1), randomIntFromInterval(10,90), randomIntFromInterval(10,90), randomIntFromInterval(0,1)]));

    const { addAlert } = useContext(SnackbarContext);
    
    const { device, windowSize } = useContext(ResponsiveContext);

    const { tokens } = useContext(TokensContext);

    const { user, tokensOwned, setNbTokens, positions } = useContext(UserContext);

    const [category, setCategory] = useState("all");

    const [tokensList, setTokensList] = useState(tokens);

    const [token1Owned, setToken1Owned] = useState<"?" | number>("?");
    const [token2Owned, setToken2Owned] = useState<"?" | number>("?");

    const [price, setPrice] = useState<number>(0);

    const [sent, setSent] = useState<number>(0);
    const [get, setGet] = useState<number>(0);

    const [minPrice, setMinPrice] = useState<number>(2);
    const [maxPrice, setMaxPrice] = useState<number>(45);
    const [price1, setPrice1] = useState<number>(2.5);
    const [price2, setPrice2] = useState<number>(35);

    const [priceImpact, setPriceImpact] = useState("0");

    const [myPosition, setMyPosition] = useState(false);

    const [chosePosition, setChosePosition] = useState(false);

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
        return (token2InPool*n/token1InPool)
    }

    function calculateSent(n: number) {
        return (token1InPool*n/token2InPool)
    }

    function calculateMax1(x: number | string) {
        if (typeof(x) == "string") return "?"
        if (token1Owned == "?") return "?"
        if (token2Owned == "?") return "?"
        if (isNaN(x)) return "?"
        if (price == 0) return "?"
        else {
            var s = calculateSent(x)
            if (isNaN(s)) return "?"
            else {
                if (token1Owned < s) return formatToString(token1Owned);
                else return formatToString(s);
            }
        }
    }

    function calculateMax2(x: number | string) {
        if (typeof(x) == "string") return "?"
        if (token1Owned == "?") return "?"
        if (token2Owned == "?") return "?"
        if (isNaN(x)) return "?"
        if (price == 0) return "?"
        else {
            var s = calculateGet(x)
            if (isNaN(s)) return "?"
            else {
                if (token2Owned < s) return formatToString(token2Owned);
                else return formatToString(s);
            }
        }
    }

    const range1Change = (event: any) => {
        var s = event.target.value;
        if (!isNaN(s)) {
            if (s.length == 0) {
                setPriceMin(0);
                return
            }
            if (s.includes(".")) {
                setPriceMin(s);
            } else {
                setPriceMin(parseFloat(s));
            }
        }
    }

    const range2Change = (event: any) => {
        var s = event.target.value;
        if (!isNaN(s)) {
            if (s.length == 0) {
                setPriceMax(0);
                return
            }
            if (s.includes(".")) {
                setPriceMax(s);
            } else {
                setPriceMax(parseFloat(s));
            }
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

    const getChange = (event: any) => {
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
        const m = tokensOwned[token2.address];
        if (m == "undefined") setToken2Owned(0);
        else setToken2Owned(parseFloat(n));
    }, [tokensOwned, token1, token2])
    
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

    const [searchPosition, setSearchPosition] = useState("");

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

    function getSearchPosition(list: any[]) {
        if (!list) return []
        return list.filter(x => {
            var flag = (searchPosition.length == 0);
            for (const word of searchPosition.split(' ')) {
                if (word.length > 0) flag = flag || x.token_x.name.toLowerCase().includes(word) || x.token_x.symb.toLowerCase().includes(word) || x.token_y.name.toLowerCase().includes(word) || x.token_y.symb.toLowerCase().includes(word)
            }
            return flag
        })
    }

    const searchChange = (event: any) => {
        setSearch(event.target.value.toLowerCase());
    }

    const searchPositionChange = (event: any) => {
        setSearchPosition(event.target.value.toLowerCase());
    }

    useEffect(() => {
        setTokensList(getSearch(tokens));
    }, [tokens, search, category])

    useEffect(() => {
        setPositionsList(getSearchPosition(positions));
    }, [positions, searchPosition])

    const [swapLoading, setSwapLoading] = useState(false);

    const [positionsList, setPositionsList] = useState(positions);

    const [nftId, setNftId] = useState(null);
    const [positionInfos, setPositionInfos] = useState<any>({
        liquidity: 0,
        token_x: null,
        token_y: null,
        price_x: 0,
        price_y: 0,
        value_locked: 0,
        x_fees: 0,
        y_fees: 0,
    });

    async function getPosInfos(id: string, invert: boolean) {
        const result:any = await getPositionInfos(id);
        if (result) {
            if (!invert) setPositionInfos(result);
            else setPositionInfos({
                liquidity: result.liquidity,
                token_x: result.token_y,
                token_y: result.token_x,
                price_x: result.price_y,
                price_y: result.price_x,
                value_locked: result.value_locked,
                x_fees: result.y_fees,
                y_fees: result.x_fees,
            })
        }
    }

    useEffect(() => {
        if (nftId == null) {
            setPositionInfos({
                liquidity: 0,
                token_x: null,
                token_y: null,
                price_x: 0,
                price_y: 0,
                value_locked: 0,
                x_fees: 0,
                y_fees: 0,
            })
        }
        else {  
            setPositionInfos({
                liquidity: 0,
                token_x: null,
                token_y: null,
                price_x: 0,
                price_y: 0,
                value_locked: '?',
                x_fees: '?',
                y_fees: '?',
            })
            getPosInfos(nftId, invertPosition);
        }
    }, [nftId]);

    const [invertPosition, setInvertPosition] = useState(false);

    function findPosition(tk1Address: string, tk2Address: string) {
        if (!positions) return
        for (const position of positions) {
            if (position.token_x.address == tk1Address && position.token_y.address == tk2Address) { setInvertPosition(false); return position.id; }
            if (position.token_x.address == tk2Address && position.token_y.address == tk1Address) { setInvertPosition(true); return position.id; }
        }

        setInvertPosition(false);
        return null;
    }

    useEffect(() => {
        setNftId(findPosition(token1.address, token2.address));
    }, [token1, token2, positions])


    async function sendSwap() {
        setSwapLoading(true);
        /*var flag;
        if (!nftId) flag = await createPosition(user.address, token1.address, token2.address, sent.toString(), get.toString())
        else flag = await addToPosition(user.address, token1.address, token2.address, sent.toString(), get.toString(), nftId)
        setNbTokens();
        resetValues();
        if (flag) {
            addAlert("check", "You have provided liquidity!");
        } else {
            addAlert("error", "An error occured");
        }
        setSwapLoading(false);
        */
    }

    const [feesLoading, setFeesLoading] = useState(false);

    async function claimF() {
        setFeesLoading(true);
        /*var flag;
        if (nftId) {
            flag = await claimFees(user.address, nftId);
            if (flag) {
                addAlert("check", "Your fees have been claimed!");
            } else {
                addAlert("error", "An error occured");
            }
            setFeesLoading(false);
        }
        else {addAlert("error", "You don't have a position"); return false;}
        */
    }

    const [removeLoading, setRemoveLoading] = useState(false);

    async function removeL(liqu: string) {
        setRemoveLoading(true);
        /*var flag;
        if (nftId) {
            if(removePercentage < 100) flag = await removeLiquidity(user.address, nftId, liqu);
            else flag = await closePosition(user.address, nftId);
            if (flag) {
                addAlert("check", "Your liquidity has been removed!");
            } else {
                addAlert("error", "An error occured");
            }
            setRemoveLoading(false);
        }
        else {addAlert("error", "You don't have a position"); return false;}
         */
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
            width: `100%`,
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
        },


        inputBar2: {
            position: 'relative' as 'relative',
            width: '100%',
            marginBottom: '5px',

            '& input': {
                fontFamily: 'primary',
                fontSize: 2,
                width: 'calc(100% - 30px - 2px)',
                padding: '10px',
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
        },

        swapIcon: {
            height: '20px',
            marginBottom: '20px',
            cursor: 'pointer',
            aspectRatio: '1',
            opacity: '.3',
            transform: 'Rotate(90deg)',
            background: '',
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
                fontSize: 1,
                fontWeight: '500'
            },
            '&:hover div': {
                opacity: '1'
            }
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
            marginBottom: '20px',

            '& button': {
                background: 'primary',
                color: 'text',
                borderRadius: '5px',
                border: 'none',
                fontFamily: 'primary',
                fontSize: 1,
                cursor: 'pointer',
                marginBottom: '10px',
                width: '40%',
                marginLeft: '30%',

                '&:hover': {
                    opacity: '.8'
                }
            }
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
            width: '250px',
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
            }
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

        poolsList: {
            overflow: 'scroll',
            '-ms-overflow-style': 'none',
            'scrollbar-width': 'none',
            '::-webkit-scrollbar': {
                display: 'none'
            },
            width: '100%',
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
            left: `${chosePosition ? '0px' : '100%'}`,
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

        myPositionColumn: {
            position: 'relative' as 'relative',
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
        },


        rangeInput: {
            position: 'relative' as 'relative',
            width: '100%',
            marginBottom: '20px',
            marginTop: '10px',

            '& p': {
                margin: 0,
                marginBottom: '-35px',
                padding: 0,
                fontFamily: 'primary',
                fontSize: '1',
                fontWeight: '600',
                color: 'text'
            }
        },

        range2: {
            width: '100%',
            '-webkit-appearance': 'none',
            background: 'transparent',
            height: '100%',

            '&:hover::-webkit-slider-thumb': {
                background: 'primary',
                cursor: 'pointer',
                animate: '0.2s',
            },

            '&:focus': {
                outline: 'none'
            },

            '&::-webkit-slider-runnable-track': {
                height: '0',
                width: '100%',
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
              },
            
              '&:nth-of-type(1)': {
                transform: 'TranslateY(35px)',
              },

              '&:nth-of-type(2)': {
                transform: 'TranslateY(17px)',
              },

              '&:nth-of-type(3)::-webkit-slider-thumb': {
                display: 'none'
              }
        },

        rangeBar: {
            position: 'absolute' as 'absolute',
            left: '0',
            bottom: '0',
            transform: 'Translate(0, 1.5px)',
            width: '100%',
            height: '5px',
            color: 'shadow',
            boxShadow: 'none',
            background: 'background3',
            borderRadius: '50px',
            border: 'none',
            overflow: 'hidden' as 'hidden',

            '& div': {
                position: 'absolute' as 'absolute',
                left: `Calc(2.5px + ${100*(Math.min(price1, price2)-minPrice)/(maxPrice-minPrice)}%)`,
                width: `${100*Math.abs(price2-price1)/(maxPrice-minPrice)}%`,
                height: '100%',
                boxShadow: 'none',
                background: 'primary',
                border: 'none',
            }
        },

        rangeInputs: {
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'space-between',

            '& input': {
                width: '100px',
            }
        },

        ranges: {
            position: 'relative' as 'relative',
            width: '100%',
            marginBottom: '25px'
        }
    }  

    function setPriceMin(x: number) {
        console.log(x);
        if (isNaN(x)) {
            if (price1 <= price2) {setPrice1(minPrice);}
            else setPrice2(minPrice);
            return
        }
        if (x > Math.max(price1, price2)) {setPrice1(Math.max(Math.min(x, maxPrice), minPrice)); setPrice2(Math.max(Math.min(x, maxPrice), minPrice)); return}
        if (price1 <= price2) {setPrice1(Math.max(Math.min(x, maxPrice), minPrice));}
        else setPrice2(Math.max(Math.min(x, maxPrice), minPrice));
    }

    function setPriceMax(x: number) {
        if (isNaN(x)) {
            if (price1 > price2) {setPrice1(minPrice);}
            else setPrice2(minPrice);
            return
        }
        if (x < Math.min(price1, price2)) {setPrice1(Math.max(Math.min(x, maxPrice), minPrice)); setPrice2(Math.max(Math.min(x, maxPrice), minPrice)); return}
        if (price1 > price2) setPrice1(Math.max(Math.min(x, maxPrice), minPrice));
        else setPrice2(Math.max(Math.min(x, maxPrice), minPrice));
    }

    return (
        <Dashboard page="liquidity">
            <Snackbar />

            {stars.map(x => { return (
                <Star left={x[1].toString()} top={x[2].toString()} height={x[0] ? "15" : "20"} color={x[3] ? "text" : "text2"}/>
            )})}

            <div sx={style.main}>
                <div sx={style.top}>
                    <div sx={style.container}>
                        <div sx={style.buttons}>
                            <span sx={myPosition && user.address ? style.inactive : style.active} onClick={() => {setMyPosition(false); setChosePosition(false); }}>Provide Liquidity</span>
                            { user.address ?
                                <span sx={myPosition ? style.active : style.inactive} onClick={() => {setMyPosition(true); resetSelect(); }}>My Position</span>
                             : null}
                        </div>
                        { myPosition && user.address ? 
                            ( <div sx={style.myPositionColumn}>
                                    <div sx={style.chosePositionContainer}>
                                        <div sx={style.chosePositionZone}>
                                            <h2><div sx={style.close} onClick={() => setChosePosition(false)}/>Your positions</h2>
                                            <div sx={style.inputBar}>
                                                <input type="text" id="searchPosition" required={true} placeholder=" " autoComplete="off" onChange={searchPositionChange}/>
                                                <label htmlFor="searchPosition">Search for a position</label>
                                            </div>
                                            <div sx={style.poolsList}>
                                                {  positionsList.map((pool: any) => {
                                                    return (
                                                        <div sx={style.poolChoice} onClick={() => {
                                                            setChosePosition(false);
                                                            setInvertPosition(false);
                                                            setToken1(pool.token_x);
                                                            setToken2(pool.token_y);
                                                            setNftId(pool.id);
                                                        }}>
                                                            <img src={pool.token_x.icon_url}/>
                                                            <img src={pool.token_y.icon_url}/>
                                                            <p>{pool.token_x.symb} - {pool.token_y.symb}</p>
                                                        </div>
                                                    )
                                                })}
                                            </div>
                                        </div>
                                    </div>
                                    <div sx={style.chosePosition}  onClick={() => setChosePosition(true)}>
                                        <img src={token1.icon_url}/>
                                        <img src={token2.icon_url}/>
                                        <p>{token1.symb} - {token2.symb}</p>
                                        <div sx={style.expand2}/>
                                    </div>
                                    <div sx={style.swapZone}>
                                        <h1>🌻 My Fees</h1>
                                        <div sx={style.swapInfos}>
                                            <span sx={style.swapInfoMain}><span>Total Locked</span><div>{price > 0 ? formatToString(positionInfos.liquidity/Math.sqrt(price)) : '?'} {token1.symb} + {price > 0 ? formatToString(positionInfos.liquidity*Math.sqrt(price)) : '?'} {token2.symb}</div></span>
                                            <span sx={style.swapInfo}><span>Value</span>${formatToString(positionInfos.value_locked)}</span>
                                            <span sx={style.swapInfo}><span>Fees</span>{formatToString2(positionInfos.x_fees)} {token1.symb} + {formatToString2(positionInfos.y_fees)} {token2.symb}</span>                                            
                                            <span sx={style.swapInfo}><span>Current ROI</span>No Data</span>
                                        </div>
                                        <button sx={feesLoading ? {...style.swapButton, ...style.swapButtonLoading} : style.swapButton} onClick={() => feesLoading ? null : claimF()}>{feesLoading ? "" : "Claim Fees"}</button>
                                    </div>
                                    <div sx={style.swapZone}>
                                        <h1>🍂 Remove Liquidity</h1>
                                        <div sx={style.rangeInput}>
                                            <p>Price Range ({token2.symb + "/" + token1.symb})</p>
                                            <div sx={style.ranges}>
                                                <div sx={style.rangeBar}>
                                                    <div/>
                                                </div>
                                                <input type="range" sx={style.range2} min={minPrice} max={maxPrice} value={price1} step={(maxPrice-minPrice)/100} onChange={(e) => {setPrice1(twoDecimals(parseFloat(e.target.value)))}}/>
                                                <input type="range" sx={style.range2} min={minPrice} max={maxPrice} value={price2} step={(maxPrice-minPrice)/100} onChange={(e) => {setPrice2(twoDecimals(parseFloat(e.target.value)))}}/>
                                                <input type="range" sx={style.range2} min="0" max="1000" step="10"/>
                                            </div>
                                            <div sx={style.rangeInputs}>
                                                <div sx={style.inputBar2}>
                                                    <input type="text" id="range1" required={true} placeholder=" " autoComplete="off" onChange={(e) => {range1Change(e)}} value={Math.min(price1, price2)}/>
                                                    <label htmlFor="range1">Price min</label>
                                                </div>
                                                <div sx={style.inputBar2}>
                                                    <input type="text" id="range2" required={true} placeholder=" " autoComplete="off" onChange={(e) => {range2Change(e)}} value={Math.max(price1,price2)}/>
                                                    <label htmlFor="range2">Price max</label>
                                                </div>
                                            </div>
                                        </div>
                                        <div sx={style.swapInfos}>
                                            <button>Load Data</button>
                                            <span sx={style.swapInfoMain}><span>Removing</span><div>? {token1.symb} + ? {token2.symb}</div></span>
                                            <span sx={style.swapInfo}><span>Value</span>$?</span>
                                        </div>
                                        <button sx={removeLoading ? {...style.swapButton, ...style.swapButtonLoading} : style.swapButton} onClick={() => removeLoading ? null : removeL('1')}>{removeLoading ? "" : "Remove Liquidity"}</button>
                                    </div>
                                </div>
                            )
                            
                            : (
                            <div sx={style.swapZone}>
                                <h1>🌱 Provide Liquidity</h1>
                                <div sx={style.inputBar}>
                                    <input type="text" id="send" required={true} placeholder=" " autoComplete="off" onChange={sentChange} value={sent}/>
                                    <label htmlFor="send">{user.address ? `You can lock ${calculateMax1(token1Owned)} ${token1.symb}`: "You lock"}</label>
                                    <div sx={style.token} onClick={() => setToken1Select(true)}>
                                        <img src={token1.icon_url}/>
                                        <p>{token1.symb}</p>
                                        <div sx={style.expand}/>
                                    </div>
                                </div>
                                <span sx={style.tokenAddress}><span>Token Address</span>{token1.address.slice(0,5) + "..." + token1.address.slice(token1.address.length - 10, token1.address.length)}</span>
                                <div sx={style.swapIcon} onClick={invert}/>
                                <div sx={style.inputBar}>
                                    <input type="text" id="get" required={true} placeholder=" " autoComplete="off" onChange={getChange} value={get}/>
                                    <label htmlFor="get">{user.address ? `You can lock ${calculateMax2(token1Owned)} ${token2.symb}`: "You lock"}</label>
                                    <div sx={style.token} onClick={() => setToken2Select(true)}>
                                        <img src={token2.icon_url}/>
                                        <p>{token2.symb}</p>
                                        <div sx={style.expand}/>
                                    </div>
                                </div>
                                <span sx={style.tokenAddress}><span>Token Address</span>{token2.address.slice(0,5) + "..." + token2.address.slice(token2.address.length - 10, token2.address.length)}</span>
                                <div sx={style.rangeInput}>
                                    <p>Price Range ({token2.symb + "/" + token1.symb})</p>
                                    <div sx={style.ranges}>
                                        <div sx={style.rangeBar}>
                                            <div/>
                                        </div>
                                        <input type="range" sx={style.range2} min={minPrice} max={maxPrice} value={price1} step={(maxPrice-minPrice)/100} onChange={(e) => {setPrice1(twoDecimals(parseFloat(e.target.value)))}}/>
                                        <input type="range" sx={style.range2} min={minPrice} max={maxPrice} value={price2} step={(maxPrice-minPrice)/100} onChange={(e) => {setPrice2(twoDecimals(parseFloat(e.target.value)))}}/>
                                        <input type="range" sx={style.range2} min="0" max="1000" step="10"/>
                                    </div>
                                    <div sx={style.rangeInputs}>
                                        <div sx={style.inputBar2}>
                                            <input type="text" id="range1" required={true} placeholder=" " autoComplete="off" onChange={(e) => {range1Change(e)}} value={Math.min(price1, price2)}/>
                                            <label htmlFor="range1">Price min</label>
                                        </div>
                                        <div sx={style.inputBar2}>
                                            <input type="text" id="range2" required={true} placeholder=" " autoComplete="off" onChange={(e) => {range2Change(e)}} value={Math.max(price1,price2)}/>
                                            <label htmlFor="range2">Price max</label>
                                        </div>
                                    </div>
                                </div>
                                <div sx={style.swapInfos}>
                                    <span sx={style.swapInfoMain}><span>Providing</span><div>{typeof(sent) == "string" ? formatToString(parseFloat(sent)) : formatToString(sent)} {token1.symb} + {typeof(get) == "string" ? formatToString(parseFloat(get)) : formatToString(get)} {token2.symb}</div></span>
                                    <span sx={style.swapInfo}><span>Pool Rate</span>1 {token1.symb} = {price == 0 ? "?" : sent == 0 ? formatToString(price) : formatToString(get/sent)} {token2.symb}</span>
                                    <span sx={style.swapInfo}><span>Expected ROI</span>No Data</span>
                                    <span sx={style.swapInfo}><span>TVL</span>No Data</span>
                                    <span sx={style.swapInfo}><span>Share of Pool</span>No Data</span>
                                </div>

                                {
                                    user.address ? 
                                    <button sx={swapLoading ? {...style.swapButton, ...style.swapButtonLoading} : style.swapButton} onClick={() => swapLoading ? null : sendSwap()}>{swapLoading ? "" : "Provide Liquidity"}</button>
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
                        )}
                    </div>
                </div>
            </div>
        </Dashboard>
    )
}

export default Liquidity;