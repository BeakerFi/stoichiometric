/** @jsxImportSource theme-ui */
import { useNavigate } from "react-router-dom";

import { useContext, useState } from "react";

import Dashboard from "components/Dashboard";

import { randomIntFromInterval } from "utils/general/generalMaths";

import Star from "components/Star";
import Snackbar from "components/Snackbar";
import ConnectWallet2 from "components/ConnectWalletLarge";

import { UserContext } from "contexts/UserContext";
import { ResponsiveContext } from "contexts/ResponsiveContext";

import styleFunction from "./style";

function Dao() {
    const [stars, setStars] = useState(Array.from({length: 10}, (_, i) => [randomIntFromInterval(0,1), randomIntFromInterval(10,90), randomIntFromInterval(10,90), randomIntFromInterval(0,1)]));

    const { user } = useContext(UserContext);
    const { device } = useContext(ResponsiveContext);

    const voteList = [{title: 'Vote for me!', subtitle: 'please', score: "55/89", address: 'test_address', finished:true, approved: true}, 
    {title: 'Vote arthur', subtitle: 'ent', score: "9/102", address: 'text_address', finished:false, approved: false},
    {title: 'Vote for me!', subtitle: 'please', score: "55/89", address: 'test_address', finished:false,  approved: false},
    {title: 'Vote for me!', subtitle: 'please', score: "55/89", address: 'test_address', finished:true,  approved: false},
    {title: 'Vote for me!', subtitle: 'please', score: "55/89", address: 'test_address', finished:true,  approved: false},
    {title: 'Vote for me!', subtitle: 'please', score: "55/89", address: 'test_address', finished: false,  approved: true}]

    const [addProposal, setAddProposal] = useState<boolean>(false);

    function toggleProposal() {
        setAddProposal(!addProposal);
    }
    
    const style = styleFunction(device);

    return (
        <Dashboard page='dao'>
            <Snackbar />

            {stars.map((x, index) => { return (
                <Star key={"star" + index} left={x[1].toString()} top={x[2].toString()} height={x[0] ? "15" : "20"} color={x[3] ? "text" : "text2"}/>
            )})}


            <div sx={style.main}>

                <div sx={style.votesContainer}>
                    <button sx={style.add} onClick={toggleProposal}>
                        {addProposal ? "See Proposals" : "Add A Proposal"}
                    </button>

                    { addProposal ? 

                        <div sx={style.addProposalZone}>
                            <label htmlFor="title">Title</label>
                            <input type="text" id="title"/>
                            <label htmlFor="subtitle">Description</label>
                            <textarea id="subtitle"/>
                            <div sx={style.property}>
                                Change<div sx={style.expand}/>
                            </div>
                            {user.address ? <button sx={style.send}>Submit</button> : <ConnectWallet2/>}
                        </div>

                        : null

                    }

                    { !addProposal ? voteList.map((x, index) => {
                        return (

                            <div key={"dao"+index} sx={style.voteContainer}>
                                <div sx={style.vote}>
                                    <div sx={style.column}>
                                        <h3>{x.title}</h3>
                                        <h4>{x.subtitle}</h4>
                                        <p sx={style.date}>{x.finished ? "Vote" : "Vote ending in"} {x.finished ? x.approved ? <span sx={style.approved}>approved</span> : <span sx={style.declined}>declined</span> : <span>3 days</span>}</p>
                                        <div sx={style.caracteristics}>
                                            <ul>
                                                <li>
                                                    Adding blabla
                                                </li>
                                                <li>
                                                    Removing blabla
                                                </li>
                                                <li>
                                                    Changing blabla
                                                </li>
                                                <li>
                                                    entent
                                                </li>
                                            </ul>
                                        </div>
                                    </div>
                                    <div sx={style.score}>
                                        <p>{x.score}</p>
                                    </div>
                                </div>

                                { x.finished ? null :
                                <div sx={style.voteButtons}>
                                    <button>+</button>
                                    <button>-</button>
                                </div>
                                }
                            </div>
                        )}) 
                        : null
                    }
                </div>
            </div>
        </Dashboard>
    )
}

export default Dao;