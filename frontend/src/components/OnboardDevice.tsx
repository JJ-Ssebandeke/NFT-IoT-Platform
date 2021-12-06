import React, { useState } from 'react'
import { addressFlowObj, metadataFlowObj } from './ConfigFlows';

interface onboardDeviceProps {
    currentIndex: number;
    onNext: (deviceData: addressFlowObj | metadataFlowObj | (addressFlowObj & metadataFlowObj)) => void;
}


export const OnboardDevice: React.FC<onboardDeviceProps> = ({children, currentIndex, onNext}) => {
 
    const inputDataHandler = (stepData: addressFlowObj | metadataFlowObj | (addressFlowObj & metadataFlowObj)) => {
        onNext(stepData);

    }
    
    const currentChild = React.Children.toArray(children)[currentIndex]
    
    if(React.isValidElement(currentChild)){
        return React.cloneElement(currentChild, { inputDataHandler })

    }
    return (
        <>
         {currentChild}
        </>

    )
    
}
