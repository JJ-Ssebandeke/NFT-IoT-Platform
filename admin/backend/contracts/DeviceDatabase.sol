// SPDX-License-Identifier: MIT
pragma solidity ^0.8.10;

library DeviceDatabase {

    struct DeviceData {
        address deviceAddress;
        address currentOwner;
        uint loanPeriod;
        uint ID;    
    }

    struct Map {
        bytes32[] keys;
        mapping(bytes32 => DeviceData) dataEntries;
        mapping(bytes32 => uint) indexOf;
        mapping(bytes32 => bool) inserted;
    }
    
    
    function getKeyAtIndex(Map storage map, uint index) internal view returns (bytes32) {
        return map.keys[index];
    }

    function HashKeys(Map storage map) public view returns (uint) {
        return map.keys.length;
    }


    function getDataEntry(Map storage map, bytes32 key) internal view returns (DeviceData memory) {
        return map.dataEntries[key];
    }
    

    function getTokenID(Map storage map, bytes32 key) internal view returns (uint){
        return getDataEntry(map,key).ID;
    }

    function getCurrentOwner(Map storage map, bytes32 key) internal view returns (address){
        return getDataEntry(map,key).currentOwner;
    }

    function getLoanPeriod(Map storage map, bytes32 key) internal view returns (uint){
        return getDataEntry(map,key).loanPeriod;
    }
    

    function updateDeviceData(Map storage map,address newOwner, bool status,uint duration, bytes32 key) internal view {

        if(status == true){
            getDataEntry(map, key).currentOwner = newOwner;
            getDataEntry(map,key).loanPeriod = duration;

        }
        if(status == false){
            getDataEntry(map, key).currentOwner = newOwner;
            getDataEntry(map,key).loanPeriod = 0;

        }

    }

    function setDeviceData(Map storage map,bytes32 key,DeviceData memory val) internal {

        if (map.inserted[key]) {
            map.dataEntries[key] = val;
        } else {
            map.inserted[key] = true;
            map.dataEntries[key] = val;
            map.indexOf[key] = map.keys.length;
            map.keys.push(key);
        }
    }


}