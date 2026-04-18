#![allow(unused_imports)] 
#![allow(dead_code)]       

use serde::{Serialize, Deserialize};
use chrono::Utc;

pub mod governance;
pub mod classifier;
pub mod verifier;
pub mod analytics; 
pub mod game_theory; 
pub mod broadcasting;
pub mod stealth_pulse;
pub mod watermark;

pub use crate::governance::*;
pub use crate::classifier::*;
pub use crate::verifier::*;
pub use crate::analytics::*; 
pub use crate::game_theory::*;
pub use crate::broadcasting::*;
pub use crate::stealth_pulse::*;
pub use crate::watermark::*;

// --- TELEMETRY & NODE ---
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AxonTelemetry {
    pub node_id: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TelemetryPayload {
    pub node_id: String,
    pub architect: String,
    pub status: String,
    pub timestamp: String,
    pub version: Option<String>, 
}

// --- CORE ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AiProvider {
    OpenAI,
    Grok,
    DeepSeek,
    HumanCreator, 
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AssetType {
    Patent,
    Movie,
    Text,
    Image,
    Audio,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SettlementLayer {
    AxonChronos,     // Specialized L3 (High-performance sync via PoH)
    AxonSovereignZK, // Independent Sovereign L3 using ZK-RVP technology
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AxonSignature {
    pub asset_type: AssetType,
    pub provider: AiProvider,
    pub settlement: SettlementLayer,
    pub hash: String,
    pub l3_transaction_id: String,
    pub timestamp: u64,
    pub owner_id: String,
    pub royalty_address: String,
    pub poison_pill_hash: String,
    pub telemetry: Option<AxonTelemetry>, 
}

pub struct AxonEncoder;

impl AxonEncoder {
    pub fn generate_signature(
        asset_type: AssetType,
        provider: AiProvider, 
        settlement: SettlementLayer,
        hash: &str, 
        time: u64,
        owner: &str,
        royalty: &str,
        poison_pill: &str,
        telemetry: Option<AxonTelemetry> 
    ) -> AxonSignature {
        
        let tx_id = format!("0xAXON_{:x}", time);

        AxonSignature {
            asset_type,
            provider,
            settlement,
            hash: hash.to_string(),
            l3_transaction_id: tx_id,
            timestamp: time,
            owner_id: owner.to_string(),
            royalty_address: royalty.to_string(),
            poison_pill_hash: poison_pill.to_string(),
            telemetry,
        }
    }
}

// --- ASSET REGISTRY ---
pub struct AssetRegistry;
impl AssetRegistry {
    pub fn register(asset_type: AssetType, owner: &str, hash: &str, layer: SettlementLayer) -> String {
        let tx_id = format!("0x{:x}", Utc::now().timestamp_nanos_opt().unwrap_or(0));
        println!("🚀 [SOVEREIGN-L3] Registrazione {:?} in corso...", asset_type);
        println!("🏗️  Architettura: {:?}", layer);
        println!("🧱 Transaction ID: {}", tx_id);
        println!("✅ Asset memorizzato nel Dark Ledger Indipendente.");
        tx_id
    }
}

// --- LEGACY IMPORTER ---
pub struct LegacyImporter;
impl LegacyImporter {
    pub fn import_google_patent(patent_id: &str) {
        println!("🌐 [LEGACY] Ricerca brevetto: {}...", patent_id);
        
        let hash = format!("SHA256-RETR-{:x}", Utc::now().timestamp());
        AssetRegistry::register(AssetType::Patent, "Giuseppe Tagliarini", &hash, SettlementLayer::AxonSovereignZK);
        println!("🛡️ [SHIELD_OF_ANTIQUITY] Immunità concessa dalla Rete AXON.");
    }
}