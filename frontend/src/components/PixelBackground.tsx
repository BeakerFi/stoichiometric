/** @jsxImportSource theme-ui */

import { useState } from "react";

import { randomIntFromInterval } from "functions/maths";

const style = {
    main: {
        height: '100%',
        width: '100%,',
        position: 'absolute' as 'absolute',
        left: '-3px',
        top: '-6px',
        display: 'flex',
        zIndex: '900'
    },

    column: {
        height: '100%',
        width: '80px',
        display: 'flex',
        flexDirection: 'column' as 'column',
        overflow: 'hidden'
    },

    tile: {
        width: '100%',
        aspectRatio: '1',
        minHeight: '80px',
        opacity: '0',
        background: 'text',
        transition: '1s cubic-bezier(1,0,1,0)'
    },

    tile1: {
        '&:hover': {
            transition: 'none',
            opacity: '.015'
        }
    },

    tile2: {
        '&:hover': {
            transition: 'none',
            opacity: '.01'
        }
    },

    tile3: {
        '&:hover': {
            transition: 'none',
            opacity: '.005'
        }
    }
}

function PixelBackground() {

    const [A, setA] = useState(Array.from({length: 25}, (_, i) => randomIntFromInterval(0,1)));
    const [B, setB] = useState(Array.from({length: 20}, (_, i) => randomIntFromInterval(0,1)));

    return (
        <div sx={style.main}>
            {A.map(x => { return (
                <div sx={style.column}>
                    {B.map(y => {
                        const tileStyle = x+y - (x*y) ? y ? {...style.tile, ...style.tile1} : {...style.tile, ...style.tile2} : {...style.tile, ...style.tile3};
                        return (
                        <div sx={tileStyle}/>
                        )
                    })}
                </div>  
            )})}
        </div>
    )
}

export default PixelBackground;