import React from 'react'
import {hot} from 'react-hot-loader'
import "./App.css"
import { useRoutes, Link } from 'react-router-dom'
import { AdminHub } from './components/AdminHub'
import { ClientPortal } from './components/ClientPortal'
import { EntryPoint } from './components/EntryPoint'


const App: React.FC = () => {
    
    const homeRoute = {
        path: '/',
        element: <EntryPoint />
    }
    
    const adminRoutes = {
        path: 'Network-Overview',
        element: <AdminHub />
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
