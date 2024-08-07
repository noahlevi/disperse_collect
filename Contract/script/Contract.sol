// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {Contract} from "../src/Contract.sol";

contract ContractScript is Script {
    Contract public contractInstance;

    function setUp() public {}

    function run() public {
        vm.startBroadcast();

        contractInstance = new Contract();

        vm.stopBroadcast();
    }
}