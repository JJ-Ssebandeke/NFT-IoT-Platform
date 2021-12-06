import React from 'react'
import { Link } from 'react-router-dom'
import styled from 'styled-components'

const SideNavContainer = styled.div`
     background-color: #fff;
     grid-row: sideNav / span 3;
     align-self: stretch;
     border-right: 1px solid #e3e4e6;
     display: grid;
     grid-template: repeat(5, 1fr) / 100%;
     justify-items: center;
     
     .logo{
          align-self: start;
          margin-top: 25px;
     }
     
     
`
export const Sidenav = (): JSX.Element => {
    return (
        <>
          <SideNavContainer>
                  <p className='logo'> L </p>
                  <Link to="Network"> H </Link> 
                  <Link to="Registration"> B </Link>
                  <Link to="Analytics"> R </Link>
                  <Link to="/"> M </Link>
             </SideNavContainer>
            
        </>
    )
}
