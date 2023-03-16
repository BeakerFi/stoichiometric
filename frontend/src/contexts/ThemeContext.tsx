import React, { useState } from "react";

const ThemeContext = React.createContext(null as any);

interface Props {
    children: any;
}

const ThemeCtx: React.FC<Props> = (props) => {

    const [themeStyle, setThemeStyle] = useState('dark')
    const [color, setColor] = useState('#fa464b');
    const toggleTheme = () => {
        setThemeStyle(themeStyle === 'light' ? 'dark' : 'light')
    }   

    return (
        <ThemeContext.Provider value={{themeStyle, toggleTheme, color, setColor, setThemeStyle}}>
            {props.children}
        </ThemeContext.Provider>
    )

};

export {ThemeContext, ThemeCtx};