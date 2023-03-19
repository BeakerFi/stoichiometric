/** @jsxImportSource theme-ui */
import { useNavigate } from "react-router-dom";

import { useContext, useState, useEffect } from "react";

import Dashboard from "components/Dashboard";

import { randomIntFromInterval } from "utils/maths";

import Star from "components/Star";

import Snackbar from "components/Snackbar";

import { UserContext } from "contexts/UserContext";
import { SnackbarContext } from "contexts/SnackbarContext";
import { ResponsiveContext } from "contexts/ResponsiveContext";
import { ThemeContext } from 'contexts/ThemeContext';




const stable = {name: "Stoichiometric USD", symb: "SUSD", address: "resource_tdx_b_arthurjetebaisegrosfdp111111fdpputeputeshitcoin", icon_url: "https://upload.wikimedia.org/wikipedia/commons/thumb/4/46/Bitcoin.svg/1024px-Bitcoin.svg.png"};
function Liquidate() {
    const navigate = useNavigate();

    const [stars, setStars] = useState(Array.from({length: 10}, (_, i) => [randomIntFromInterval(0,1), randomIntFromInterval(10,90), randomIntFromInterval(10,90), randomIntFromInterval(0,1)]));


    const { user, achievements, logoutUser, accountsList, setUser } = useContext(UserContext);
    const { device } = useContext(ResponsiveContext);
    const { themeStyle, toggleTheme, setColor, color } = useContext(ThemeContext);
    const { addAlert } = useContext(SnackbarContext);

    const [accountSelect, setAccountSelect] = useState(false);

    const lends = [{
        user: "address_test",
        token: {name: "Radix", symb: "XRD", address: "address_test", icon_url: "https://switchere.com/i/currencies/XRD.svg"},
        quantity: 34.45,
        borrowed: 139.89,
        healthFactor: .5
    },
    {
        user: "account_tdx_b_1pr3kgqdw7gaz7v6zk65xjet79w9c72qyzpcuhgmv8rksqs2ugt",
        token: {name: "Radix", symb: "XRD", address: "address_test", icon_url: "https://switchere.com/i/currencies/XRD.svg"},
        quantity: 34.45,
        borrowed: 239.89,
        healthFactor: 4
    },
    {
        user: "address_test",
        token: {name: "Radix", symb: "XRD", address: "address_test", icon_url: "test"},
        quantity: 34.45,
        borrowed: 394.89,
        healthFactor: .7
    },
    {
        user: "address_test",
        token: {name: "Radix", symb: "XRD", address: "address_test", icon_url: "test"},
        quantity: 34.45,
        borrowed: 339.89,
        healthFactor: 1.4
    }];

    const [lendsList, setLendsList] = useState<any[]>([]);
    const [onlyMine, setOnlyMine] = useState<boolean>(false);

    function toggleOnlyMine() {
        setOnlyMine(!onlyMine);
    }

    useEffect(() => {
        setLendsList(user.address && onlyMine ? lends.filter(x => x.user != user.address) : lends);
    }, [onlyMine])

    const style = {
        main: {
            display: 'flex',
            flexDirection: 'column' as 'column',
            alignItems: 'center',
            position: 'absolute' as 'absolute',
            left: `${device == "mobile" ? "10px" : device == "tablet" ? "115px" : '170px'}`,
            top: `${device == "mobile" ? "95px" : "200px"}`,
            height: `${device == "mobile" ? "calc(100% - 105px)" : "calc(100% - 60px)"}`,
            width: `${device == "mobile" ? "calc(100% - 20px)" : device == "tablet" ? "calc(100% - 135px)" : 'calc(100% - 190px)'}`,
        },

        lend: {
            width: '250px',
            height: '150px',
            padding: '20px',
            marginBottom: '20px',
            background: 'background2',
            display: 'flex',
            flexDirection: 'column' as 'column',
            position: 'relative' as 'relative',
            borderRadius: '5px',

            '& button': {
                position: 'absolute' as 'absolute',
                left: '50%',
                bottom: '20px',
                transform: 'TranslateX(-50%)',
                background: 'primary',
                color: 'white',
                fontFamily: 'primary',
                fontSize: 0,
                border: 'none',
                borderRadius: '5px',
                padding: '5px 10px',
                cursor: 'pointer',
                width: '100px',

                '&:hover': {
                    opacity: '.8'
                }
            },

            '& p': {
                color: 'text2',
                fontFamily: 'primary',
                fontSize: 1,
                padding: '0',
                margin: '0',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'space-between',
                width: '100%',
                marginTop: '10px',

                '&:nth-of-type(1)': {
                    marginTop: '15px',
                }
            },

            '& span': {
                display: 'flex',
                alignItems: 'center',
                color: 'text',
                fontWeight: '600'
            },

            '& img': {
                width: '25px',
                height: '25px',
                borderRadius: '1000px',
                marginLeft: '10px'
            }
        },

        check: {
            display: 'flex',
            alignItems: 'center',
            color: 'text',
            fontFamily: 'primary',
            fontSize: 0,
            width: '1300px',
            marginBottom: '20px',

            '& input': {
                marginRight: '10px'
            },

            '& *': {
                cursor: 'pointer'
            }
        },

        lendColumn: {
            display: 'flex',
            justifyContent: 'space-between',
            flexDirection: 'column' as 'column',
            alignItems: 'center',
            width: '1300px'
        },

        lendRow: {
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
            width: '100%',
            
        },

        lendContainer: {
            overflow: 'scroll',
            height: 'calc(100vh - 250px)',
        },

        barContainer: {
            width: 'calc(100% - 40px)',
            position: 'absolute' as 'absolute',
            left: '50%',
            bottom: '30px',
            transform: 'translateX(-50%)',


            '&:hover div': {
                display: 'block'
            }
        },


        infos: {
            background: 'background3',
            position: 'absolute' as 'absolute',
            left: '50%',
            bottom: '20px',
            transform: 'TranslateX(-50%)',
            display: 'none',
            width: '100%',
            padding: '5px 10px',
            borderRadius: '5px',

            '& p': {
                fontSize: 0,
                margin: '0 !important'
            }
        },

        bar: {
            width: '100%',
            height: '10px',
            background: 'background3',
            borderRadius: '1000px',
            position: 'relative' as 'relative',
            overflow: 'hidden',

            '& div': {
                position: 'absolute' as 'absolute',
                width: '100%',
                height: '100%',
                top: '50%',
                transform: 'translateY(-50%)',
            },
        },
    }
    //<button sx={saveLoading ? {...style.saveButton, ...style.saveButtonLoading} : style.saveButton} onClick={sendSave}>{saveLoading ? "" : "Save"}</button>

    return (
        <Dashboard page='liquidate'>
            <Snackbar />

            {stars.map(x => { return (
                <Star left={x[1].toString()} top={x[2].toString()} height={x[0] ? "15" : "20"} color={x[3] ? "text" : "text2"}/>
            )})}
            <div sx={style.main}>
                {user.address ? <div sx={style.check}>
                    <input type="checkbox" onChange={toggleOnlyMine} id="myLends"/>
                    <label htmlFor="myLends">Hide my loans</label>
                </div> : null}
                <div sx={style.lendContainer}>
                    <div sx={style.lendColumn}>   
                        {[1,2,3,4,5].map(x => {return (
                            <div sx={style.lendRow}>
                                {lendsList.map(x => {
                                    return (
                                        <div sx={style.lend}>
                                                <p>Collateral <span>{x.quantity} {x.token.symb} <img src={x.token.icon_url}/></span></p>
                                                <p>Borrowed <span>{x.borrowed} {stable.symb} <img src={stable.icon_url}/></span></p>
                                                { x.healthFactor > 1 || !user.address ? 
                                                <div sx={style.barContainer}>
                                                    <div sx={style.bar}>
                                                        <div sx={{right: `Min(100%, ${1/x.healthFactor*100}%)`, background: `${Math.random() < .3 ? "green" : Math.random() > .5 ? "orange" : "red"}`}}/>
                                                    </div>
                                                    <div sx={style.infos}>
                                                        <p>
                                                            Current Price <span>$43.4</span>
                                                        </p>
                                                        <p>
                                                            Liquidation Price <span>$23.7</span>
                                                        </p>
                                                    </div>
                                                </div> 
                                                : 
                                                <button>Liquidate</button> }
                                        </div>
                                    )
                                })}
                            </div>
                        )})}
                    </div>
                </div>
            </div>
        </Dashboard>
    )
}

export default Liquidate;