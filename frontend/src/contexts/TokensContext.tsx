import React, { useState, useEffect } from "react";

import { getTokensAndPools } from "utils/connectToBackend";

const TokensContext = React.createContext(null as any);

interface Props {
    children: any;
}

const TokensCtx: React.FC<Props> = (props) => {

    const [tokens, setTokens] = useState<any[]>([]);
    const [pools, setPools] = useState<any[]>([]);

    const [tokensLoading, setTokensLoading] = useState(false);

    useEffect(() => {
        async function setToks() {
            setTokensLoading(true);
            const x = await getTokensAndPools();
            setTokens(x.tokens);
            setPools(x.pools);
            console.log(x.pools[0]);
            setTokensLoading(false);
        };
        setToks();
    }, [])

    return (
        <TokensContext.Provider value={{tokens, tokensLoading, pools}}>
            {props.children}
        </TokensContext.Provider>
    )

};

export {TokensContext, TokensCtx};