import React from 'react'
import styled from 'styled-components'
import { Card } from './globalStyles'

const AnalyticsContainer = styled.div`
    display: grid;
    margin-left: 2.5%;
    grid-template:  100% / 20% 80%;
    height: 100%;
    width: 100%;
`
const DeviceInfoContainer = styled.div`
    height: 100%;
    width: 80%;
    margin-left: 25px;
    display: flex;
    flex-direction: column;
    row-gap: 27.5px;
    
`
const LoanHistoryContainer = styled.div`
    height: 85%;
    width: 47.5%;
    background-color: #fff;
    border-radius: 4px;
    box-shadow: 0px 0px 0px 1px rgb(6 10 13 / 3%), 0px 2px 4px rgb(6 10 13 / 4%);
`


export const DeviceAnalytics = (): JSX.Element => {
    return (
        <>
         <AnalyticsContainer>
             <DeviceInfoContainer>
                 <Card>
                   <p>Registered</p>
                 </Card>
                 <Card>
                    <p>Loaned</p>
                 </Card>
                 <Card>
                    <p>Overdue</p>
                 </Card>

             </DeviceInfoContainer>
             <LoanHistoryContainer>

             </LoanHistoryContainer>


         </AnalyticsContainer>
        
        </>
    
    )
}
