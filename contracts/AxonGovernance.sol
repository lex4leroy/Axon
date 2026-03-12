// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract AxonGovernance {
    address[] public admins;
    mapping(address => bool) public isAdmin;
    
    struct Proposal {
        bytes32 dataHash;
        uint256 voteCount;
        bool executed;
        mapping(address => bool) hasVoted;
    }

    Proposal[] public proposals;

    constructor(address[] memory _initialAdmins) {
        for(uint i=0; i < _initialAdmins.length; i++) {
            admins.push(_initialAdmins[i]);
            isAdmin[_initialAdmins[i]] = true;
        }
    }

    // Richiede l'unanimità per eseguire un'azione (es. revoca o nuovi membri)
    function executeProposal(uint _proposalId) public {
        Proposal storage p = proposals[_proposalId];
        require(p.voteCount == admins.length, "Richiesta UNANIMITA non raggiunta");
        require(!p.executed, "Gia eseguita");
        p.executed = true;
    }
}
