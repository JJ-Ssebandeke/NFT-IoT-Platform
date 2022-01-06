// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

/**
 * Manage the creation and storage of devices.
 */

contract DeviceHub {

    // Device ownership lifecycle
    enum OwnershipState{STASHED, LOANED}

    address private admin;
   
    event DeviceRegistered(address _newDevice, string _ID);
    event DeviceLoaned(address _to,string _deviceID, uint date);
    event PaymentCollected(uint _value);
    event DeviceReturned(address _holder, string _Id);
    event PenaltyIncurred(uint _amount);

    uint256 private LATE_FEE;
  
    struct Device {
        string  deviceID;
        string  deviceURL;
        address deviceAddress;
        address deviceOwner;
        uint collateral;
        OwnershipState state;
        uint loanPeriod;    
    }

    // Device storage
    Device[] private deviceStash;

    constructor() payable {
        admin = msg.sender;
        LATE_FEE = 0.002 ether;

    }

    // Function to receive Ether. msg.data must be empty
    receive() external payable {}

    // Fallback function is called when msg.data is not empty
    fallback() external payable {}

    function registerDevice(string memory _devID, string memory _url, address _devAddress) external {
        uint eth = 0;
        OwnershipState status = OwnershipState.STASHED;
        Device memory newDevice = Device( _devID,_url,_devAddress,admin, eth,status,0);
        deviceStash.push(newDevice);
        emit DeviceRegistered(newDevice.deviceAddress,newDevice.deviceID);
    }

    function getDeviceCount() public view returns(uint count){
        return deviceStash.length;
    
    }
    function getStashedDevice(uint _index) public view returns(Device memory _registeredDevice){
        
        Device storage grabDevice = deviceStash[_index];
        return grabDevice;
    }
 
    // function to update the device properties for the devices stored in the contract.
    function requestDeviceLoan(uint amount, uint loanDuration) external {
          for(uint j = 0; j < getDeviceCount(); j++){
                Device memory _device = getStashedDevice(j);
                if(_device.state == OwnershipState.STASHED){

                    setDeviceSatus(_device, OwnershipState.LOANED ,amount ,msg.sender, loanDuration);
                    emit DeviceLoaned(msg.sender, _device.deviceID, block.timestamp);
                    break;
                }
          }
    }
    // function to update the device properties for the devices stored in the contract.
    function redeemDeviceLoan(address loaneeAddress) external {

          for(uint j = 0; j < getDeviceCount(); j++){
                Device memory _device = getStashedDevice(j);
                if(_device.deviceOwner == loaneeAddress && _device.loanPeriod >= block.timestamp){
                    
                    emit DeviceReturned(loaneeAddress,_device.deviceID);

                    (bool sent, bytes memory data) = loaneeAddress.call{value: _device.collateral}("");
                    require(sent, "Failed to send Ether");

                    setDeviceSatus(_device, OwnershipState.STASHED, 0, admin,0); 
                    emit DeviceReturned(loaneeAddress,_device.deviceID);     
                    break;
                }
                if(_device.deviceOwner == loaneeAddress && block.timestamp >= (_device.loanPeriod + 3600)){
                    uint penalty = _device.collateral - LATE_FEE;

                    emit PenaltyIncurred(penalty);

                    (bool sent, bytes memory data) = loaneeAddress.call{value: penalty}("");
                    require(sent, "Failed to send Ether");

                    setDeviceSatus(_device, OwnershipState.STASHED, 0, admin,0); 
                    emit DeviceReturned(loaneeAddress,_device.deviceID);     
                    break;
                }
            }
       
    }


    function setDeviceSatus(Device memory _d, OwnershipState isLoaned ,uint ethValue, address newOwner , uint duration)  internal pure {

         _d.state = isLoaned;
         _d.collateral = ethValue;
         _d.deviceOwner = newOwner;
         _d.loanPeriod = duration;
        
    }
  
  
}


