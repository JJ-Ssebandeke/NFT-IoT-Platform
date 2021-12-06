import React, { useState } from 'react'
import styled from 'styled-components'
import { AddressFlow, addressFlowObj, ConfrimationFlow, MetadataFlow, metadataFlowObj } from './ConfigFlows'
import { OnboardDevice } from './OnboardDevice'


const FormContainer = styled.div`
    height: 85%;
    width: 55%;
    margin-left: 20%;
    display: flex;
    flex-direction: column;
    row-gap: 10px;
    
`
const FlowNavigation = styled.div`
    height: 9.5%;
    width: 99.9%;
    border-radius: 4px;
    border: 0.5px solid rgba(0, 0, 0, 0.04);
    background-color: #e6e6e681;
    display:flex;
    font-size: 18px;
    .flow-nav-items{
        align-self: center;
        list-style: none;
        display: flex;
        column-gap: 20px;
        width: 100%;
        justify-content: space-evenly;

    }
    
`
const OnboardingFlow = styled.div`
    height: 90%;
    width: 100%;
    background-color: #fff;
    border-radius: 4px;
    box-shadow: 0px 0px 0px 1px rgb(6 10 13 / 3%), 0px 2px 4px rgb(6 10 13 / 4%);
    
`

export const DeviceConfig = (): JSX.Element => {

    const [deviceConfig, setDeviceConfig] = useState<addressFlowObj | metadataFlowObj | (addressFlowObj & metadataFlowObj)>
    ({} as addressFlowObj | metadataFlowObj | addressFlowObj & metadataFlowObj)
    const [currentIndex, seCurrentIndex ] = useState(0)

    const inputDataHandler = (deviceData: addressFlowObj | metadataFlowObj | (addressFlowObj & metadataFlowObj)) => {

        seCurrentIndex(currentIndex + 1)
        setDeviceConfig(
            {
            ...deviceConfig,
            ...deviceData
            }
        )

    }

    return (
        <FormContainer>
         <FlowNavigation>
             <ul className='flow-nav-items'>
                 <li>Identifer</li>
                 <li>Metadata</li>
                 <li>Confirm</li>
             </ul>

         </FlowNavigation>
         <OnboardingFlow>
             <OnboardDevice currentIndex={currentIndex} onNext={inputDataHandler}>
                 <AddressFlow />
                 <MetadataFlow />
                 <ConfrimationFlow />
             </OnboardDevice>
            
         </OnboardingFlow>
            
        </FormContainer>
    )
}
