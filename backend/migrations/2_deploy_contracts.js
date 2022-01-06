const DeviceHub = artifacts.require("./DeviceHub.sol");
const DeviceTag = artifacts.require("./DeviceTag.sol");

module.exports = function(deployer) {
    deployer.deploy(DeviceHub).then(function(){
        return deployer.deploy(DeviceTag, DeviceHub.address)
    })
}