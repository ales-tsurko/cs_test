// SPDX-License-Identifier: Unlicense
pragma solidity >=0.4.0 <0.9.0;

contract CidStorage {
    string public cid;

    function set(string memory s) public {
        cid = s;
    }

    function get() public view returns (string memory) {
        return cid;
    }
}
