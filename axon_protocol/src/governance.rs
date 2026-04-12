use chrono::{DateTime, Utc, Duration};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminProfile {
    pub name: String,
    pub visionary_score: u32,
    pub entry_timestamp: DateTime<Utc>,
}

pub struct VisionaryScore;

impl VisionaryScore {
    pub fn calculate(entry_date: DateTime<Utc>, launch_date: DateTime<Utc>) -> u32 {
        let duration = entry_date.signed_duration_since(launch_date);
        
        if duration <= Duration::hours(48) {
            100 // Visionario Assoluto (Status Fondatore)
        } else if duration <= Duration::days(30) {
            75  // Leader Strategico
        } else {
            10  // Late Follower
        }
    }
}

// --- AXON BOARD (v2026.1) ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BoardMember {
    NVIDIA,
    Google,
    Meta,
    TeslaXAI,
    OpenAI,
    GenesisArchitect(String),
}

pub struct AdministrativeBoard {
    pub members: Vec<BoardMember>,
    pub consensus_required: f64, // 1.0 for 100% unanimity
}

impl AdministrativeBoard {
    pub fn is_unanimous(&self, votes: usize) -> bool {
        votes == self.members.len()
    }

    pub fn get_genesis_board() -> Self {
        AdministrativeBoard {
            members: vec![
                BoardMember::NVIDIA,
                BoardMember::Google,
                BoardMember::Meta,
                BoardMember::TeslaXAI,
                BoardMember::OpenAI,
                BoardMember::GenesisArchitect("Giuseppe Tagliarini".to_string()),
            ],
            consensus_required: 1.0,
        }
    }
}

// --- SOLANA L3 ANCHOR ---
pub const SOLANA_L3_GENESIS_BLOCK: &str = "AXON_SOL_L3_0x2026_01";
pub const CHRONOS_STANDARD: &str = "PoH_UTC_SYNC";