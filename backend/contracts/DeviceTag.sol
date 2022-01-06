// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
import "../node_modules/@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol";
import "../node_modules/@openzeppelin/contracts/utils/Counters.sol";
import "../contracts/DeviceHub.sol";

contract DeviceTag is ERC721Enumerable {

    using Counters for Counters.Counter;
    Counters.Counter private _tokenIds;
    uint256 private INSURANCE_FEE;
    uint256 public LoanCount;
    
    
    DeviceHub public devices;
    constructor(DeviceHub _devices) public ERC721("DeviceTag", "TAG") {
        devices = _devices;
        INSURANCE_FEE = 0.005 ether;
        LoanCount= 0;
    }

    function mintTag(uint _duration, uint _devicesRequested) public payable {

        require(LoanCount + _devicesRequested <= devices.getDeviceCount(),"Can't processes loan");
        require(msg.value == INSURANCE_FEE, "Collateral Not Met");

        address _to = msg.sender;
        _tokenIds.increment();
        
        for(uint i = 0; i < _devicesRequested; i++) {
            devices.requestDeviceLoan(msg.value, _duration);
        }
        
        
        uint256 id = _tokenIds.current();
        _mint(_to, id);
        LoanCount += _devicesRequested;
        
    }

    function burnTag(uint _devicesReturned) public {
        
         uint tagCount = balanceOf(msg.sender);
         require(tagCount > 0);
        
         for(uint i = 0; i < tagCount; i++) {
                devices.redeemDeviceLoan(msg.sender);
               _burn(tokenOfOwnerByIndex(msg.sender, i)); 
         }
         LoanCount -= _devicesReturned;

    }

}