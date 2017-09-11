pragma solidity ^0.4.11;

interface PermissionInterface {
    /// 0: "None" - no pemission
    /// 1: "Create" - create contract
    /// 2: "Send" - send tx
    /// 3: "Both": both permission

    // grant the permission to a user
    function grantPermission(uint _permission, address _user) returns (bool); 
    // revoke the permission of a user
    function revokePermission(uint _permission, address _user) returns (bool);
    // query users of the permission
    function queryUsersOfPermission(uint _permission) constant returns (string);
    // query the user's permission 
    function queryPermission(address _user) returns (uint);
}
