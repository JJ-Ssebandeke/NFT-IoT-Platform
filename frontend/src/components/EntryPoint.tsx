import React from 'react'
import { Link } from 'react-router-dom'

export const EntryPoint: React.FC  = () => {
    return (
        <>
          <Link to="Client-Portal"> Client-Portal </Link>
          <Link to="Network-Overview/Dashboard"> Admin-Dashboard </Link>
            
        </>
    )
}
