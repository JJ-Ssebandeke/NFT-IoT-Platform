// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "../node_modules/@openzeppelin/contracts/token/ERC1155/extensions/ERC1155Supply.sol";
import "./DeviceDatabase.sol";

/**
 * Manage the creation and storage of devices.
 */

contract DeviceManager is ERC1155Supply {

    using DeviceDatabase for DeviceDatabase.Map;
    using DeviceDatabase for DeviceDatabase.DeviceData;

    event DeviceRegistered(address _newDevice, uint _ID);
    event DeviceLoaned(address _to,uint _deviceID, uint date);
    event DeviceReturned(address _holder, uint _Id);

    address private admin;
    uint256 private LATE_FEE;

    DeviceDatabase.Map private map;
   
    constructor() ERC1155("") payable {
        admin = msg.sender;

    }
   
    function registerDevice(uint deviceID, string memory url, address deviceAddress) external {

        DeviceDatabase.DeviceData memory _data = DeviceDatabase.DeviceData(deviceAddress, admin, 0, deviceID);
        bytes32 byteData = keccak256(abi.encodePacked(_data.deviceAddress,deviceID));
        map.setDeviceData(byteData,_data);

        _mint(admin, deviceID, 1,"");
        emit DeviceRegistered(_data.deviceAddress,deviceID);
    }

     function checkAvailability(bytes32 key) internal view returns (bool isAvailable){
        if(map.getDataEntry(key).currentOwner == admin){
            return true;
        }
        else{
            return false;
        } 
    }

    function getTotalSupply(uint _id) external view returns(uint){
        return totalSupply(_id); 

    }

    function getLoanCount(address loanee) external view returns (uint){
         
         uint count = 0;
         for (uint i = 0; i < map.HashKeys(); i++){
             bytes32 dataHash =  map.getKeyAtIndex(i);
             if(map.getDataEntry(dataHash).currentOwner == loanee){
                 count++;   
             }

         }
        return count;
    }

    // function to update the device properties for the devices stored in the contract.
    function onLoan(uint loanDuration, uint deviceID) external  {

           for(uint j = 0; j < totalSupply(deviceID); j++){
                for (uint i= 0; i < map.HashKeys(); i++){
                    bytes32 dataHash =  map.getKeyAtIndex(i);
                    if(map.getTokenID(dataHash) == deviceID && checkAvailability(dataHash)){
                        map.updateDeviceData(msg.sender,true, loanDuration, dataHash);
                        emit DeviceLoaned(msg.sender,deviceID, block.timestamp);
                    }
                }
               
            }
  
    }
    // function to update the device properties for the devices stored in the contract.
    function onReturn(address loaneeAddress, uint deviceId) external returns(bool onTime) {
         
            for (uint i= 0; i < map.HashKeys(); i++){
                bytes32 dataHash = map.getKeyAtIndex(i);
                if(map.getTokenID(dataHash) == deviceId){
                    if(map.getCurrentOwner(dataHash) == loaneeAddress && map.getLoanPeriod(dataHash) >= block.timestamp){

                        map.updateDeviceData(admin, false, 0, dataHash);
                        emit DeviceReturned(loaneeAddress, deviceId);
                        return true;
                    }
                    if(map.getCurrentOwner(dataHash) == loaneeAddress && (map.getLoanPeriod(dataHash) + 3600) <= block.timestamp){
                        map.updateDeviceData(admin, false, 0, dataHash);
                        return false;
                    }
                          
                }
            }
    }


}


