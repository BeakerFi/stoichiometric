import React, { useState, useEffect } from "react";

import { getTokensPoolsAndLenders } from "utils/general/generalApiCalls";

const TokensContext = React.createContext(null as any);

interface Props {
    children: any;
}

const TokensCtx: React.FC<Props> = (props) => {

    const [tokens, setTokens] = useState<any[]>([]);
    const [pools, setPools] = useState<any[]>([]);
    const [lenders, setLenders] = useState<any[]>([]);

    const [tokensLoading, setTokensLoading] = useState(false);

    useEffect(() => {
        async function setToks() {
            setTokensLoading(true);
            const x = await getTokensPoolsAndLenders();
            var l = [];
            for (var i = 0; i < x.lenders.length; ++i) l[x.lenders[i].token] = x.lenders[i].lender;
            setLenders(l);
            setTokens(x.tokens);
            setPools(x.pools);
            setTokensLoading(false);
        };
        setToks();
    }, [])

    return (
        <TokensContext.Provider value={{tokens, tokensLoading, pools, lenders}}>
            {props.children}
        </TokensContext.Provider>
    )

};

export {TokensContext, TokensCtx};