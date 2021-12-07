// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

/**
 * Manage the creation and storage of devices.
 */
contract DeviceHub {


/**
   * Device ownership lifecycle
*/
  enum OwnershipState{STASHED, LOANED}

    address private admin;
   
    event DeviceRegistered(address _newDevice, string _ID);
    event DevicesLoaned(address _to,string _deviceID, uint date);
    event PaymentCollected(uint _value);
    event LoanRedeemed(address _holder, uint _Id, uint amountSent);
    uint256 private  LATE_FEE;
  
    struct Device {
        string  deviceID;
        string  deviceURL;
        address deviceAddress;
        address deviceOwner;
        uint collateral;
        uint tagId;
        OwnershipState state;
        uint loanPeriod;    
    }

    /**
   * Device storage
   */

    Device[] private deviceStash;

    constructor() payable {
        admin = msg.sender;
        LATE_FEE = 0.002 ether;

    }

    // Function to receive Ether. msg.data must be empty
    receive() external payable {}

    // Fallback function is called when msg.data is not empty
    fallback() external payable {}

    function registerDevice(string memory _devID, string memory _url, address _devAddress) public {
        
        uint eth = 0;
        uint tagID = 0;
        OwnershipState status = OwnershipState.STASHED;
        Device memory newDevice = Device( _devID,_url,_devAddress,admin, eth, tagID, status,0);
        deviceStash.push(newDevice);
        emit DeviceRegistered(newDevice.deviceAddress,newDevice.deviceID);

    }
    

    function getDeviceCount() public view returns(uint count){
        return deviceStash.length;
    
    }
    function getStashedDevice(uint _index) internal view returns(Device storage _registeredDevice){
        
        Device storage grabDevice = deviceStash[_index];
        return grabDevice;
    }

     /**
   * Process a device Loan request. 
   *
   * This function will, update the device properties for the devices stored in the contract.
   *                            
   */
    function requestDeviceLoan(uint amount, uint _id, uint loanDuration) external {
          for(uint8 j = 0; j < getDeviceCount(); j++){
                Device storage _device = getStashedDevice(j);
                if(_device.state == OwnershipState.STASHED){
                    setDeviceSatus(_device, OwnershipState.LOANED ,amount ,msg.sender, _id, loanDuration);
                    emit DevicesLoaned(msg.sender, _device.deviceID, block.timestamp);
                    break;
                }
          }
    }
    function redeemDeviceLoan(address loaneeAddress) external returns(uint _tokenId) {
          uint _tag = 0;
          for(uint8 j = 0; j < getDeviceCount(); j++){
                Device storage _device = getStashedDevice(j);
                if(_device.deviceOwner == loaneeAddress && _device.loanPeriod >= block.timestamp){
                    
                    emit LoanRedeemed(loaneeAddress,_device.tagId, _device.collateral);
                    
                    if(_tag == 0){
                        _tag = _device.tagId;
                    }

                    (bool sent, bytes memory data) = loaneeAddress.call{value: _device.collateral}("");
                    require(sent, "Failed to send Ether");
                    setDeviceSatus(_device, OwnershipState.STASHED, 0, admin, 0,0);      
                    break;
                }
                if(_device.deviceOwner == loaneeAddress && block.timestamp >= (_device.loanPeriod + 3600)){
                    uint penalty = _device.collateral - LATE_FEE;
                    emit LoanRedeemed(loaneeAddress,_device.tagId, penalty);

                    if(_tag == 0){
                        _tag = _device.tagId;
                    }

                    (bool sent, bytes memory data) = loaneeAddress.call{value: penalty}("");
                    require(sent, "Failed to send Ether");
                    setDeviceSatus(_device, OwnershipState.STASHED, 0, admin, 0,0);      
                    break;

                }
            }
            return _tag;
    }


    function setDeviceSatus(Device storage _d, OwnershipState isLoaned ,uint ethValue, address newOwner , uint newId,  uint duration)  internal {

         _d.state = isLoaned;
         _d.collateral = ethValue;
         _d.deviceOwner = newOwner;
         _d.tagId = newId;
         _d.loanPeriod = duration;
        
    }


       
  
}


