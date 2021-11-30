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
    width: 52.5%;
    margin-left: 25px;
    margin-right: 5px;
    background-color: #fff;
    border-radius: 4px;
    box-shadow: 0px 0px 0px 1px rgb(6 10 13 / 3%), 0px 2px 4px rgb(6 10 13 / 4%);
    
`
const BlockInfoContainer = styled.div`
    height: 85%;
    width: 92.5%;
    background-color: #fff;
    border-radius: 4px;
    box-shadow: 0px 0px 0px 1px rgb(6 10 13 / 3%), 0px 2px 4px rgb(6 10 13 / 4%);
    margin-right: 2.5%;
    
`

export const BlockExplorer: React.FC = () => {
    return (
        <>
            <TransactionContainer>
                <Card>

                </Card>
                <Card>

                </Card>
                <Card>

                </Card>
                <LatestTransaction>
                    
                </LatestTransaction>
                <Card>

                </Card>
            </TransactionContainer>
            <BlockInfoContainer>

            </BlockInfoContainer>
        </>
    )
}
