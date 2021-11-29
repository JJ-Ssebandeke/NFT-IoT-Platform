import React from 'react'
import { Navbar } from './Navbar'
import { Sidenav } from './Sidenav'
import styled from 'styled-components'

const AdminHubContainer = styled.div`
    display: grid;
    grid-template: 10% 90% / 4% 96%;
    grid-template-areas: "sideNav  nav"
                         "sideNav  content";

`

export const AdminHub: React.FC = () => {
    return (
        <AdminHubContainer>
          <Navbar />
          <Sidenav />
          <div className='Admin-Hub-content'>

          </div>
            
        </AdminHubContainer>
    )
}
