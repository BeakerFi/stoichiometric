/** @jsxImportSource theme-ui */
import { useNavigate } from "react-router-dom";

import { useContext, useState, useEffect } from "react";

import Dashboard from "components/Dashboard";

import { randomIntFromInterval } from "utils/maths";

import Star from "components/Star";

import Snackbar from "components/Snackbar";

import ConnectWallet2 from "components/ConnectWallet2";

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

    const voteList = [{title: 'Vote for me!', subtitle: 'please', score: "55/89", address: 'test_address', finished:true, approved: true}, 
    {title: 'Vote arthur', subtitle: 'ent', score: "9/102", address: 'text_address', finished:false, approved: false},
    {title: 'Vote for me!', subtitle: 'please', score: "55/89", address: 'test_address', finished:false,  approved: false},
    {title: 'Vote for me!', subtitle: 'please', score: "55/89", address: 'test_address', finished:true,  approved: false},
    {title: 'Vote for me!', subtitle: 'please', score: "55/89", address: 'test_address', finished:true,  approved: false},
    {title: 'Vote for me!', subtitle: 'please', score: "55/89", address: 'test_address', finished: false,  approved: true}]

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
            width: '80%',
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
            marginBottom: '20px',
            borderRadius: '5px',
            padding: '20px',

            display: 'flex',
            flexDirection: 'column' as 'column',
            alignItems: 'center',
            width: '660px',

            '& *': {
                color: 'text',
                fontFamily: 'primary',
            },

            '& label': {
                fontSize: 2,
                fontWeight: '600',
                marginBottom: '10px',
                width: '100%',
                cursor: 'pointer',
            },

            '& input, textarea': {
                background: 'transparent',
                width: 'calc(100% - 20px)',
                border: 'solid 1px',

                borderColor: 'background3',
                borderRadius: '5px',
                fontSize: 1,
                marginBottom: '20px',
                padding: '5px 10px',
            },

            '& textarea': {
                height: '200px',
                resize: 'none' as 'none'
            },

        },

        send: {
            background: 'primary',
            border: 'none',
            borderRadius: '5px',
            fontSize: '1',
            padding: '10px 20px',
            cursor: 'pointer',

            '&:hover': {
                opacity: '.8'
            }
        },

        property: {
            fontSize: 1,
            color: 'text',
            fontFamily: 'primary',
            border: 'solid 1px',
            borderColor: 'background3',
            borderRadius: '5px',
            padding: '5px 20px',
            position: 'relative' as 'relative',
            paddingRight: '40px',
            marginBottom: '20px',
            cursor: 'pointer',

            '&:hover': {
                borderColor: 'text',

                '& div': {
                    opacity: '1'
                }
            }
        },

        expand: {
            height: '12px',
            aspectRatio: '1',
            background: 'text',
            opacity: '.3',
            "-webkit-mask":`url('/icons/expand.svg') center/contain no-repeat`,
                        mask:`url('/icons/expand.svg') center/contain no-repeat`,
            transform: 'TranslateY(-50%) Rotate(90deg)',
            position: 'absolute' as 'absolute',
            right: '10px',
            top: '50%',
        },

        date: {
            position: 'absolute' as 'absolute',
            right: '20px',
            top: '20px',
            fontFamily: 'primary',
            fontSize: 1,
            color: 'text2',
            
            '& span': {
                fontWeight: '600',
                color: 'text'
            }
        },

        approved: {
            color: 'green !important'
        },

        declined: {
            color: 'red !important'
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
                    { !addProposal ? voteList.map(x => {
                        return (
                            <div sx={style.voteContainer}>
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
                        )
                    }) : null}
                </div>
            </div>
        </Dashboard>
    )
}

export default Dao;