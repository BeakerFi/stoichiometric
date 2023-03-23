import React, { useState, useEffect } from "react";
import { lender } from "types";

import { getTokensPoolsAndLenders } from "utils/general/generalApiCalls";
import { getAllCollection, getAllLoansInformation } from "utils/stablecoin/issuerApiCalls";

const TokensContext = React.createContext(null as any);

interface Props {
    children: any;
}

const TokensCtx: React.FC<Props> = (props) => {

    const [tokens, setTokens] = useState<any[]>([]);
    const [pools, setPools] = useState<any[]>([]);
    const [lenders, setLenders] = useState<any[]>([]);
    const [loans, setLoans] = useState<any[]>([]);

    const [tokensLoading, setTokensLoading] = useState(false);

    useEffect(() => {
        async function setToks() {
            setTokensLoading(true);
            const x = await getTokensPoolsAndLenders();
            setTokens(x.tokens);

            var poolsList: any[] = [];
            for (var i = 0; i < x.pools.length; ++i) poolsList[x.pools[i].token.address] = x.pools[i];
            setPools(poolsList);

            var l: any = [];
            for (var i = 0; i < x.lenders.length; ++i) { 
                l[x.lenders[i].token] = x.lenders[i].lender;
                console.log(x.lenders[i].lender)
            }
            setLenders(l);
            setTokensLoading(false);


            const y = await getAllCollection();
            const z = await getAllLoansInformation(y, l);

            setLoans(z);
        };
        setToks();
    }, [])


    return (
        <TokensContext.Provider value={{tokens, tokensLoading, pools, lenders, loans}}>
            {props.children}
        </TokensContext.Provider>
    )

};

export {TokensContext, TokensCtx};