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

function Dao() {
    const navigate = useNavigate();

    const [stars, setStars] = useState(Array.from({length: 10}, (_, i) => [randomIntFromInterval(0,1), randomIntFromInterval(10,90), randomIntFromInterval(10,90), randomIntFromInterval(0,1)]));

    const { user, achievements, logoutUser, accountsList, setUser } = useContext(UserContext);
    const { device } = useContext(ResponsiveContext);
    const { themeStyle, toggleTheme, setColor, color } = useContext(ThemeContext);
    const { addAlert } = useContext(SnackbarContext);

    const voteList = [{title: 'Vote for me!', subtitle: 'please', score: "55/89", address: 'test_address'}, {title: 'Vote arthur', subtitle: 'ent', score: "9/102", address: 'text_address'},
    {title: 'Vote for me!', subtitle: 'please', score: "55/89", address: 'test_address'},
    {title: 'Vote for me!', subtitle: 'please', score: "55/89", address: 'test_address'},
    {title: 'Vote for me!', subtitle: 'please', score: "55/89", address: 'test_address'},
    {title: 'Vote for me!', subtitle: 'please', score: "55/89", address: 'test_address'}]

    const [addProposal, setAddProposal] = useState<boolean>(false);

    function toggleProposal()
        {
            setAddProposal(!addProposal);
        }
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

        voteContainer: {
            width: '700px',
            display: 'flex',
            justifyContent: 'space-between',
            marginBottom: '20px',
        },

        vote: {
            width: '90%',
            marginRight: '20px',
            padding: '0 20px',
            background: 'background2',
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
            position: 'relative' as 'relative',
        },

        column: {
            display: 'flex',
            flexDirection: 'column' as 'column',
            marginTop: '20px',

            '& *': {
                color: 'text',
                fontFamily: 'primary',
                margin: '0',
                padding: '0'
            },

            '& h4': {
                fontsize: 1,
                opacity: '.7'
            }
        },

        caracteristics: {
            display: 'flex',
            flexDirection: 'column' as 'column',

            marginTop: '10px',
            marginBottom: '20px',
            marginLeft: '20px',

            '& li': {
                color: 'text',
                fontFamily: 'primary',
                fontSize: 1
            }
        },

        score: {
            '& p': {
                color: 'text',
                fontSize: 3,
                fontFamily: 'primary',
                fontWeight: '600'
           }
        },

        voteButtons: {
            display: 'flex',
            flexDirection: 'column' as 'column',
            justifyContent: 'space-between',
            padding: '20px 0',
            
            '& button': {
                background: 'primary',
                color: 'white',
                fontFamily: 'primary',
                fontSize: 4,
                fontweight: '600',
                border: 'none',
                borderRadius: '5px',
                width: '35px',
                cursor: 'pointer',

                '&:hover': {
                    opacity: '.8'
                }
            }
        },

        add: {
            border: 'none',
            background: 'primary',
            borderRadius: '5px',
            padding: '10px 20px',
            color: 'white',
            fontSize: 1,
            fontFamily: 'primary',
            cursor: 'pointer',
            marginBottom: '20px',

            '&:hover': {
                opacity: '.8',
            }
        },

        votesContainer: {
            height: 'calc(100vh - 250px)',
            overflow: 'scroll'
        },

        addProposalZone: {
            background: 'background2',
            width: 'calc(100% - 40px)',
            height: '50px',
            marginBottom: '20px',
            borderRadius: '5px',
            padding: '20px',
        }
    }

    return (
        <Dashboard page='dao'>
            <Snackbar />

            {stars.map(x => { return (
                <Star left={x[1].toString()} top={x[2].toString()} height={x[0] ? "15" : "20"} color={x[3] ? "text" : "text2"}/>
            )})}
            <div sx={style.main}>
                <div sx={style.votesContainer}>
                    <button sx={style.add} onClick={toggleProposal}>
                        {addProposal ? "See Proposals" : "Add A Proposal"}
                    </button>
                    {
                        addProposal ? 
                        <div sx={style.addProposalZone}>
                        </div>
                        : null
                    }
                    { !addProposal ? voteList.map(x => {
                        return (
                            <div sx={style.voteContainer}>
                                <div sx={style.vote}>
                                    <div sx={style.column}>
                                        <h3>{x.title}</h3>
                                        <h4>{x.subtitle}</h4>
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
                                <div sx={style.voteButtons}>
                                    <button>+</button>
                                    <button>-</button>
                                </div>
                            </div>
                        )
                    }) : null}
                </div>
            </div>
        </Dashboard>
    )
}

export default Dao;