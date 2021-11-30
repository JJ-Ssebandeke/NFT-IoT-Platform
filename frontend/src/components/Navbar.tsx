import React from 'react'
import styled from 'styled-components'
import { useLocation } from 'react-router-dom'

const NavContainerTop = styled.div`
     background-color: #fff;
     border-bottom: 1px solid #e3e4e6;
     height: 55.5px;
     z-index: 3;
     grid-row: nav / 2;
     grid-column: 2 / 3;
     align-self: stretch;
     display: flex;
     justify-content: space-between;

     .nav-items {
        list-style: none;
        display: flex;
        justify-content: flex-end;
        margin: 15px 30px;
        column-gap: 20px;
    }

    .page-location{
        font-size: 20px;
        margin: 12.3px 45px;
    }
`

export const Navbar: React.FC = () => {
    const location = useLocation();

    return (
        <>
          <NavContainerTop>
              <p className='page-location'>{location.pathname.substring(18)}</p>
              <ul className= "nav-items">
                 <li> Network Peers: 8 </li>
                 <li> Latest Block: 10 </li>
                 <li> Time stamp: 20 </li>
              </ul>
         </NavContainerTop>    
            
        </>
    )
}
