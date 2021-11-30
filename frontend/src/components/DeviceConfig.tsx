import React from 'react'
import styled from 'styled-components'


const FormContainer = styled.div`
    height: 85%;
    width: 55%;
    background-color: #fff;
    border-radius: 4px;
    box-shadow: 0px 0px 0px 1px rgb(6 10 13 / 3%), 0px 2px 4px rgb(6 10 13 / 4%);
    margin-left: 20%;
    
`

export const DeviceConfig: React.FC = () => {
    return (
        <FormContainer>
         <form className="add-device-form">
            
         </form>
            
        </FormContainer>
    )
}
