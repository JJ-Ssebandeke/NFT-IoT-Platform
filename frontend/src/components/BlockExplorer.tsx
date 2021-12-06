import React from 'react'
import styled from 'styled-components'
import { Card } from './globalStyles'


const TransactionContainer = styled.div`
    height: 85%;
    width: 92.5%;
    margin-left: 25px;
    display: flex;
    flex-wrap: wrap;
    
`
const LatestTransaction = styled.div`
    height: 350px;
    width: 81.5%;
    margin-left: 25px;
    display: flex;
    flex-direction: column;
    row-gap: 0.35px;
    
`
const TransactionNavigation = styled.div`
    height: 9.5%;
    width: 100%;
    border-top-right-radius: 4px;
    border-top-left-radius: 4px;
    background-color: #fff;
    box-shadow: 0px 0px 0px 1px rgb(6 10 13 / 3%), 0px 2px 4px rgb(6 10 13 / 4%);
    display:flex;
    font-size: 15px;
    p{
        margin: 6.5px 10px; 

    }
`
const Transactions = styled.div`
    height: 91.5%;
    width: 100%;
    background-color: #fff;
    border-bottom-left-radius: 4px;
    border-bottom-right-radius: 4px;
    box-shadow: 0px 0px 0px 1px rgb(6 10 13 / 3%), 0px 2px 4px rgb(6 10 13 / 4%);  
    
`
const BlockInfoContainer = styled.div`
    height: 85%;
    width: 92.5%;
    margin-right: 2.5%;
    display: flex;
    flex-direction: column;
    row-gap: 10px;
`
const BlockNavigation = styled.div`
    height: 9.5%;
    width: 99.9%;
    border-radius: 4px;
    border: 0.5px solid rgba(0, 0, 0, 0.04);
    background-color: #e6e6e681;
    display:flex;
    font-size: 15px;
    .block-nav-items{
        align-self: center;
        list-style: none;
        display: flex;
        column-gap: 20px;
        width: 100%;
        justify-content: space-evenly;

    }
    
`
const BlockData = styled.div`
    height: 91.5%;
    width: 100%;
    background-color: #fff;
    border-radius: 4px;
    box-shadow: 0px 0px 0px 1px rgb(6 10 13 / 3%), 0px 2px 4px rgb(6 10 13 / 4%);  
`

export const BlockExplorer = (): JSX.Element => {
    return (
        <>
            <TransactionContainer>
                <Card>
                    <p>Admin ID</p>
                </Card>
                <Card>
                    <p>General Info</p>
                </Card>
                <Card>
                    <p> Details </p>
                </Card>
                <LatestTransaction>
                    <TransactionNavigation>
                      <p>Latest Transactions</p>
                    </TransactionNavigation>
                    <Transactions>

                    </Transactions>
                    
                </LatestTransaction>
            </TransactionContainer>
            <BlockInfoContainer>
                <BlockNavigation>
                    <ul className='block-nav-items'>
                        <li>Block Number</li>
                        <li>Hash</li>
                        <li>Transactions</li>
                        
                    </ul>

                </BlockNavigation>
                <BlockData>

                </BlockData>
              
            </BlockInfoContainer>
        </>
    )
}
