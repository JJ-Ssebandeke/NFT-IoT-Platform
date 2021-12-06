import React, { Children } from 'react'
import {hot} from 'react-hot-loader'
import "./App.css"
import { useRoutes, Link } from 'react-router-dom'
import { AdminHub } from './components/AdminHub'
import { ClientPortal } from './components/ClientPortal'
import { EntryPoint } from './components/EntryPoint'
import { BlockExplorer } from './components/BlockExplorer'
import { DeviceConfig } from './components/DeviceConfig'
import { DeviceAnalytics } from './components/DeviceAnalytics'


const App = (): JSX.Element => {
    
    const homeRoute = {
        path: '/',
        element: <EntryPoint />
    }
    
    const adminRoutes = {
        path: 'Network-Overview',
        element: <AdminHub />,
        children: [
            {path: 'Network', element: <BlockExplorer />},
            {path: 'Registration', element: <DeviceConfig /> },
            {path: 'Analytics', element: <DeviceAnalytics />}
        ]
    }
    
    const clientRoutes = {
        path: 'Client-Portal',
        element: <ClientPortal />
    }

    const routeElement = useRoutes([homeRoute, adminRoutes, clientRoutes])

    return (
        <div className='App'>
             
            {routeElement}

            
        </div>
    )
}

export default (hot)(module)(App)
