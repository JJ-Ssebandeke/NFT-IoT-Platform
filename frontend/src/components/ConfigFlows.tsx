import React, {useState} from 'react'
import styled from 'styled-components';

export interface dataHandlerFunc {
    inputDataHandler?: <T>(deviceData: T) => void;

}

export type addressFlowObj = {
    EthAddress: string
}

export type metadataFlowObj = {
    deviceID: string,
    metadataUrl: string
}

const FlowForm = styled.div`
    display: flex;
    flex-direction: column;
    row-gap: 37.5px;
    margin-top: 57.5px;
    margin-left: 27.5px;
    
    input[type=text] {
       border: 1px solid rgba(128, 128, 128, 0.308);
       padding: 12px;
       width: 250px;
       border-radius: 3px;
       width: 50%;
    }
    .form-input-container{
        display: flex;
        flex-direction: column;
        row-gap: 7.5px;
    }
    .form-button-container{
        display: flex;
        column-gap: 1%;
    }
`
const Button = styled.button`

      border: 1px solid rgba(128, 128, 128, 0.068);
      color: rgb(240, 239, 239);
      border-radius: 2.5px;
      text-decoration: none;
      padding: 7.5px 0;
      cursor: pointer;
      background-color: #2849dd;
      width: 10%;
    
`

export const AddressFlow: React.FC<dataHandlerFunc> = ({inputDataHandler}) => {
    const [address,setAddress] = useState('')
    const handleChange: React.ChangeEventHandler<HTMLInputElement> = e => {
        setAddress(e.target.value.trim())
    }
    const collectData = () => {
        if(inputDataHandler){
            inputDataHandler<addressFlowObj>({ 'EthAddress': address })
        }

    } 
    return (
        <>  
          <FlowForm>
               <h2>Device Address</h2>
               <div className='form-input-container'>
                  <p>Input a corresponding ETH Address:</p>
                  <input type="text" name="EthAddress" placeholder= "Device Address" value={address} onChange={handleChange}></input> 
               </div>
               <div className='form-button-container'>
                  <Button> Previous </Button>
                  <Button onClick={collectData}>  Next </Button> 
               </div> 
          </FlowForm>
             
        </>
    )
}

export const MetadataFlow: React.FC<dataHandlerFunc> = ({inputDataHandler}) => {
    const [id,setId] = useState('')
    const [metadataUrl,setmetadataUrl] = useState('')
    const handleChange: React.ChangeEventHandler<HTMLInputElement> = e => {
        setId(e.target.value.trim())
    }
    const collectData = () => {
        if(inputDataHandler){
            inputDataHandler<metadataFlowObj>({'deviceID': id, 'metadataUrl': metadataUrl })
        }

    } 
    return (
        <>  
          <FlowForm>
             <h2> Token Metadata </h2>
             <div className='form-input-container'>
                 <p>Generate a unique device ID: </p>
                 <input type="text" name="deviceID" placeholder= "Device ID" value={id} onChange={handleChange}></input>
                 <p> Attach a corresponding IPFS URL:</p>
                 <input type="text" name="url" placeholder= "IPFS URL" value={metadataUrl} onChange={handleChange}></input>
             </div>
             <div className='form-button-container'>
                 <Button> Previous </Button>  
                 <Button onClick={collectData}> Next </Button>
           </div>
          </FlowForm>
          
        </>
    )
}

export const ConfrimationFlow: React.FC<dataHandlerFunc> = ({inputDataHandler}) => {
    return (
        <>
             <button onClick={() => {}}> Register Device </button>
        </>
    )
}