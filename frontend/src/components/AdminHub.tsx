import React from 'react'
import { Navbar } from './Navbar'
import { Sidenav } from './Sidenav'
import styled from 'styled-components'
import { Outlet } from 'react-router-dom'

const AdminHubContainer = styled.div`
    display: grid;
    height: 100vh;
    grid-template: 10% 90% / 4% 96%;
    grid-template-areas: "sideNav  nav"
                         "sideNav  content";
    background-color: #f8f8f9;
    .Admin-Hub-content {
        grid-row: content;
        display: flex;

    }

`

export const AdminHub: React.FC = () => {
    return (
        <AdminHubContainer>
          <Navbar />
          <Sidenav />
          <div className='Admin-Hub-content'>
             <Outlet />
          </div>
            
        </AdminHubContainer>
    )
}
