// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;
import "../node_modules/@openzeppelin/contracts/token/ERC721/extensions/ERC721Enumerable.sol";
import "../node_modules/@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "../node_modules/@openzeppelin/contracts/utils/Counters.sol";
import "../node_modules/@openzeppelin/contracts/utils/Strings.sol";
import "../node_modules/@openzeppelin/contracts/token/ERC1155/utils/ERC1155Holder.sol";
import "../contracts/DeviceManager.sol";

contract DeviceTag is ERC721Enumerable, ERC1155Holder {

    using Counters for Counters.Counter;
    Counters.Counter private _tokenIds;
    uint256 private ISSUANCE_FEE;
    uint256 private LATE_FEE;

    mapping(address => uint) private collateral;
    mapping(uint => uint[]) private devicesOnLoan;
    
    event PaymentCollected(uint _value);
    event LateDevice(uint _id);

    DeviceManager public manager;
    constructor(DeviceManager _manager) public ERC721("Tag", "NFT") {
        manager = _manager;
        ISSUANCE_FEE = 0.005 ether;
        LATE_FEE = 0.001 ether;
    }

    function supportsInterface(bytes4 interfaceId) public view virtual override(ERC721Enumerable, ERC1155Receiver) returns (bool) {
        return super.supportsInterface(interfaceId);
    }

    function mintTag(uint _duration, uint _devicesRequested, uint _id) public payable {

        require(manager.balanceOf(address(this), _id) + _devicesRequested <= manager.totalSupply(_id),"Can't processes loan");
        require(msg.value == ISSUANCE_FEE, "FEE Not Met");
        emit PaymentCollected(msg.value);

        collateral[msg.sender] = msg.value;
        
        address _to = msg.sender;
        uint256 tagID;

        for(uint i = 0; i < _devicesRequested; i++) {

            _tokenIds.increment();
            tagID = _tokenIds.current();

            manager.LoanDevice(_duration,_id);
            _safeMint(_to, tagID);
            devicesOnLoan[_id].push(tagID);
        }
        manager.safeTransferFrom(address(manager), address(this), _id,_devicesRequested, "");
        
    }

    function burnTag(uint _devicesReturned, uint _deviceID) public {
         require(balanceOf(msg.sender) > 0);

         address loanee = msg.sender;
         bool onTime;
         for(uint i = 0; i < _devicesReturned; i++) {
              for(uint j = 0; j < devicesOnLoan[_deviceID].length; j++){
                  if(ownerOf(devicesOnLoan[_deviceID][j]) == loanee){
                     _burn(devicesOnLoan[_deviceID][j]);
                     devicesOnLoan[_deviceID][j] = devicesOnLoan[_deviceID][devicesOnLoan[_deviceID].length-1];
                     devicesOnLoan[_deviceID].pop();
                     onTime = manager.returnDevice(loanee,_deviceID); 
                     break;
                  }
                }
             if(!onTime){
                 uint penalty =  collateral[msg.sender] - LATE_FEE;
                 collateral[msg.sender] = penalty;
                 emit LateDevice(_deviceID);

             }


         }

         if(manager.getLoanCount(loanee) == 0) {
              (bool sent, bytes memory data) = loanee.call{value: collateral[loanee]}("");
                     require(sent, "Failed to send Ether");
                
         }

        manager.safeTransferFrom(address(this), address(manager),_deviceID ,_devicesReturned, "");

    }

    function getTagIDs(uint _id) external view returns(uint[] memory) {
        require(msg.sender == manager.getAdmin());
        return devicesOnLoan[_id];
    }


}