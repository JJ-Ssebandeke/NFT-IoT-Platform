// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
import "../node_modules/@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "../node_modules/@openzeppelin/contracts/utils/Counters.sol";
import "../contracts/DeviceHub.sol";

contract cryptoTagDispensary is ERC721 {

    using Counters for Counters.Counter;
    Counters.Counter private _tokenIds;
    uint256 private INSURANCE_FEE;
    
    DeviceHub public devices;
    constructor(DeviceHub _devices) public ERC721("CryptoTag", "CTAG") {
        devices = _devices;

    }

    function mintTag(address _to, uint _duration, uint _devicesRequested) public payable returns (uint256){

        require(_to == msg.sender);
        require(balanceOf(_to) < devices.getDeviceCount(), "All devices have been loaned");
        _tokenIds.increment();
        
        for(uint8 i = 0; i < _devicesRequested; i++) {
            uint256 tag = _tokenIds.current();
            devices.requestDeviceLoan(msg.value, tag, _duration);
        }

        uint256 newItemId = _tokenIds.current();
        _mint(_to, newItemId);
        return newItemId;
    }

    function burnTag(address payable _tagHolder) public {
         require(_tagHolder == msg.sender);
         uint tagCount = balanceOf(_tagHolder);
         for(uint8 i = 0; i < tagCount; i++) {
               uint id = devices.redeemDeviceLoan(_tagHolder);
               _burn(id); 
         }

        


    }

}