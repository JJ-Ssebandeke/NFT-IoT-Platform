import React from 'react'
import { Link } from 'react-router-dom'

export const EntryPoint = (): JSX.Element => {
    return (
        <>
          <Link to="Client-Portal"> Client-Portal </Link>
          <Link to="Network-Overview/Network"> Admin-Dashboard </Link>
            
        </>
    )
}
