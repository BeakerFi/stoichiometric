/** @jsxImportSource theme-ui */
import { useNavigate } from "react-router-dom";

import { useContext, useState, useEffect } from "react";

import Dashboard from "components/Dashboard";

import { randomIntFromInterval } from "utils/general/generalMaths";

import Star from "components/Star";

import Snackbar from "components/Snackbar";

import { UserContext } from "contexts/UserContext";
import { SnackbarContext } from "contexts/SnackbarContext";
import { ResponsiveContext } from "contexts/ResponsiveContext";
import { ThemeContext } from 'contexts/ThemeContext';
import styleFunction from "./style";




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

    const style = styleFunction(device);


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