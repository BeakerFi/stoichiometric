import React, { useState, useContext, useEffect } from "react";

import { rdt, resetRdt } from "utils/connectToWallet";

import { SnackbarContext } from "./SnackbarContext";

import { getOwnedTokens } from "utils/general/generalApiCalls";

import { getOwnedPositions } from "utils/dex/routerApiCalls";

import {position} from "types";
import { getOwnedLoans, getAllLoansInformation } from "utils/stablecoin/issuerApiCalls";
import { TokensContext } from "./TokensContext";

const UserContext = React.createContext(null as any);


interface Props {
    children: any;
}

interface User {
    address: string | null;
    name: string | null;
};

const UserCtx: React.FC<Props> = (props) => {
    const { addAlert } = useContext(SnackbarContext);

    const { lenders } = useContext(TokensContext);

    const [user, setUser] = useState<User>({address: null, name: null})

    const [accountsList, setAccountsList] = useState<User[]>([]);

    const [connectionLoading, setConnectionLoading] = useState(false);

    const [positions, setPositions] = useState<position[]>([]);

    const [myLoans, setMyLoans] = useState<any[]>([]);

    const [achievements, setAchievements] = useState({
        ach_0: false,
        ach_1: false,
        ach_2: false,
        ach_3: false,
        ach_4: false,
        ach_5: false,
        ach_6: false,
        ach_7: false,
        ach_8: false,
        ach_9: false,
        ach_10: false,
        ach_11: false
    })

    const [tokensOwned, setTokensOwned] = useState<any[]>([]);

    async function setNbTokens(address?: string) {
        if (address == undefined) {
            if (user.address) {
                const result:any = await getOwnedTokens(user.address);
                if (result.length) setTokensOwned(result[0]);
            } else return
        }
        else {
            const result:any = await getOwnedTokens(address);
            if (result && result.length) setTokensOwned(result[0]);
        }
    } 

    async function setMyPositions(address?: string) {
        if (address == undefined) {
            if (user.address) {
                const result:any = await getOwnedPositions(user.address);
                const loans: any = await getOwnedLoans(user.address);
                setPositions(result);
                setMyLoans(await getAllLoansInformation(loans, lenders));
            } else return
        } else {
            const result:any = await getOwnedPositions(address);
            const loans: any = await getOwnedLoans(address);
            setPositions(result);
            setMyLoans(await getAllLoansInformation(loans, lenders));
        }
    }

    useEffect(() => {
        rdt.state$.subscribe(async state => { 
            setUser({address: state.accounts ? state.accounts[0].address : null, name: state.accounts ? state.accounts[0].label : null });
            setAccountsList(state.accounts ? state.accounts.map(x => { return {address: x.address, name: x.label}}) : [])         
        });
    }, []);

    useEffect(() => {
        async function setLoans() {
            if (user.address) {
                const loans: any = await getOwnedLoans(user.address);
                setMyLoans(await getAllLoansInformation(loans, lenders));
            } else return 
        }
        setLoans()
        
    }, [lenders])

    useEffect(() => {
        console.log("positions", positions);
    }, [positions])

    async function setUserValues(address:string) {
        setNbTokens(address);
        setMyPositions(address);
        addAlert("check", "Your account is connected");
    }

    useEffect(() => {
        if (user.address) {
            setUserValues(user.address);
        }
    }, [user])

    async function connectUser() {
        if(!connectionLoading) {
            addAlert("warning", "Please approve connexion on your Wallet");
            setConnectionLoading(true);

            const accounts: any = await rdt.requestData({
                accounts: { quantifier: 'atLeast', quantity: 1 },
            });
        }
    }

    async function logoutUser() {
        setConnectionLoading(false);
        resetRdt();

        setUser({address: null, name: null});
        setTokensOwned([]);
        addAlert("check", "Your are logged out");
    }

    return (
        <UserContext.Provider value={{user, accountsList, connectUser, logoutUser, connectionLoading, tokensOwned, positions, setNbTokens, achievements, setUser, myLoans}}>
            {props.children}
        </UserContext.Provider>
    )

};

export {UserContext, UserCtx};