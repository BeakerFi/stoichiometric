//import MainLayout from 'layout/MainLayout';

// render - pages
import Home from 'pages/Home';
import Swap from 'pages/Swap';
import Liquidity from 'pages/Liquidity';
import Lend from 'pages/Lend';
// ==============================|| MAIN ROUTING ||============================== //

const MainRoutes = [
    {
        path: '/',
        element: <Home />
    },{
        path: '/swap',
        element: <Swap />
    },{
        path: '/liquidity',
        element: <Liquidity />
    },{
        path: '/lend',
        element: <Lend />
    }
];

export default MainRoutes;
