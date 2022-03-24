// SPDX-License-Identifier: MIT

pragma solidity ^0.8.3;

import "../node_modules/@openzeppelin/contracts/token/ERC1155/IERC1155.sol";

interface IDeviceManager is IERC1155 {

     function onLoan(uint loanDuration, uint deviceID) external ;
     function onReturn(address loaneeAddress, uint deviceId) external returns(bool);
     function getLoanCount(address loanee) external view returns (uint);
     function getTotalSupply(uint id) external view returns(uint);

}