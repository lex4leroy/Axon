import os

# Definisci il percorso base (D:\Axon)
base_path = r"D:\Axon"

# Struttura delle cartelle e dei file
project_structure = {
    "axon_protocol": {
        "Cargo.toml": """[package]
name = "axon-protocol"
version = "0.1.0"
edition = "2021"
license = "GPL-3.0-or-later"

[dependencies]
blake3 = "1.5"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = "0.4"
rand = "0.8"
""",
        "src": {
            "lib.rs": """use serde::{Serialize, Deserialize};
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AiProvider {
    OpenAI, Grok, DeepSeek, Meta, Google, Anthropic,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AxonSignature {
    pub provider: AiProvider,
    pub original_content_hash: String,
    pub segment_index: u32,
    pub total_segments: u32,
    pub created_at: i64,
    pub is_hallucination: bool,
}

pub struct AxonEncoder;
impl AxonEncoder {
    pub fn generate_signature(provider: AiProvider, hash: &str, seg: u32, tot: u32, hall: bool) -> AxonSignature {
        AxonSignature {
            provider,
            original_content_hash: hash.to_string(),
            segment_index: seg,
            total_segments: tot,
            created_at: Utc::now().timestamp(),
            is_hallucination: hall,
        }
    }
}
""",
            "main.rs": """use axon_protocol::{AxonEncoder, AiProvider, AxonSignature};

fn main() {
    println!("AXON Protocol - Core Engine Initialized");
    let sig = AxonEncoder::generate_signature(AiProvider::OpenAI, "0x123", 0, 1, false);
    println!("Generated Signature: {:?}", sig);
}
"""
        }
    },
    "contracts": {
        "AxonGovernance.sol": """// SPDX-License-Identifier: GPL-3.0-or-later
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
"""
    },
    "README.md": """# AXON Protocol
## Global AI Integrity & Provenance Standard

### Vision
AXON is an open-source protocol (GPLv3) designed to embed permanent, blockchain-verifiable DNA into AI-generated media.

### Key Features
- **Temporal DNA:** Detects medleys and malicious edits.
- **Unanimous Governance:** Controlled by a board of top AI providers (OpenAI, Grok, DeepSeek, etc.).
- **Hallucination Audit:** Flags content diverted for copyright safety.

### License
GPLv3 - Created for global security and artist protection.
"""
}

def create_structure(parent_path, structure):
    for name, content in structure.items():
        path = os.path.join(parent_path, name)
        if isinstance(content, dict):
            os.makedirs(path, exist_ok=True)
            create_structure(path, content)
        else:
            with open(path, "w", encoding="utf-8") as f:
                f.write(content)
            print(f"Creato: {path}")

if __name__ == "__main__":
    if not os.path.exists(base_path):
        os.makedirs(base_path)
    create_structure(base_path, project_structure)
    print("\n--- Struttura completata con successo in D:\\Axon ---")