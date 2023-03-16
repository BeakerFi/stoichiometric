//import MainLayout from 'layout/MainLayout';

// render - pages
import Home from 'pages/Home';
import Swap from 'pages/Swap';
import Liquidity from 'pages/Liquidity';
import Feedback from 'pages/Feedback';
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
        path: '/feedback',
        element: <Feedback />
    }
];

export default MainRoutes;
