function styleFunction(device: string) {
    return {
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

        lend: {
            width: '250px',
            height: '150px',
            padding: '20px',
            marginBottom: '20px',
            background: 'background2',
            display: 'flex',
            flexDirection: 'column' as 'column',
            position: 'relative' as 'relative',
            borderRadius: '5px',

            '& button': {
                position: 'absolute' as 'absolute',
                left: '50%',
                bottom: '20px',
                transform: 'TranslateX(-50%)',
                background: 'primary',
                color: 'white',
                fontFamily: 'primary',
                fontSize: 0,
                border: 'none',
                borderRadius: '5px',
                padding: '5px 10px',
                cursor: 'pointer',
                width: '100px',

                '&:hover': {
                    opacity: '.8'
                }
            },

            '& p': {
                color: 'text2',
                fontFamily: 'primary',
                fontSize: 1,
                padding: '0',
                margin: '0',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'space-between',
                width: '100%',
                marginTop: '10px',

                '&:nth-of-type(1)': {
                    marginTop: '15px',
                }
            },

            '& span': {
                display: 'flex',
                alignItems: 'center',
                color: 'text',
                fontWeight: '600'
            },

            '& img': {
                width: '25px',
                height: '25px',
                borderRadius: '1000px',
                marginLeft: '10px'
            }
        },

        check: {
            display: 'flex',
            alignItems: 'center',
            color: 'text',
            fontFamily: 'primary',
            fontSize: 0,
            width: '1300px',
            marginBottom: '20px',

            '& input': {
                marginRight: '10px'
            },

            '& *': {
                cursor: 'pointer'
            }
        },

        lendColumn: {
            display: 'flex',
            justifyContent: 'space-between',
            flexDirection: 'column' as 'column',
            alignItems: 'center',
            width: '1300px'
        },

        lendRow: {
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
            width: '100%',
            
        },

        lendContainer: {
            overflow: 'scroll',
            height: 'calc(100vh - 250px)',
        },

        barContainer: {
            width: 'calc(100% - 40px)',
            position: 'absolute' as 'absolute',
            left: '50%',
            bottom: '30px',
            transform: 'translateX(-50%)',


            '&:hover div': {
                display: 'block'
            }
        },


        infos: {
            background: 'background3',
            position: 'absolute' as 'absolute',
            left: '50%',
            bottom: '20px',
            transform: 'TranslateX(-50%)',
            display: 'none',
            width: '100%',
            padding: '5px 10px',
            borderRadius: '5px',

            '& p': {
                fontSize: 0,
                margin: '0 !important'
            }
        },

        bar: {
            width: '100%',
            height: '10px',
            background: 'background3',
            borderRadius: '1000px',
            position: 'relative' as 'relative',
            overflow: 'hidden',

            '& div': {
                position: 'absolute' as 'absolute',
                width: '100%',
                height: '100%',
                top: '50%',
                transform: 'translateY(-50%)',
            },
        },
    }
}

export default styleFunction;