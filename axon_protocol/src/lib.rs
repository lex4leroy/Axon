#![allow(unused_imports)] // <--- Zittisce i warning globali per gli import non usati
#![allow(dead_code)]       // <--- Zittisce i warning per le funzioni/struct non usate nel binario attuale

use serde::{Serialize, Deserialize};

pub mod governance;
pub mod classifier;
pub mod verifier;
pub mod analytics; 

pub use crate::governance::*;
pub use crate::classifier::*;
pub use crate::verifier::*;
pub use crate::analytics::*; 

// --- NUOVE DEFINIZIONI PER TELEMETRIA E MASTER NODE ---

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AxonTelemetry {
    pub node_id: String,
    pub status: String,
}

impl AxonTelemetry {
    pub fn log_event(&self) {
        println!("📡 [TELEMETRY] Segnale inviato dal nodo: {}", self.node_id);
    }
}

// Struttura dati per il tuo Command Center sul Mac
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TelemetryPayload {
    pub node_id: String,
    pub architect: String,
    pub status: String,
    pub timestamp: String,
    pub version: Option<String>, 
}

// --- CORE DEL PROTOCOLLO ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AiProvider {
    OpenAI,
    Grok,
    DeepSeek,
    HumanCreator, 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AxonSignature {
    pub provider: AiProvider,
    pub hash: String,
    pub timestamp: u64,
    pub telemetry: Option<AxonTelemetry>, 
}

pub struct AxonEncoder;

impl AxonEncoder {
    pub fn generate_signature(
        provider: AiProvider, 
        hash: &str, 
        time: u64, 
        telemetry: Option<AxonTelemetry> 
    ) -> AxonSignature {
        
        if let Some(ref t) = telemetry {
            t.log_event();
        }

        AxonSignature {
            provider,
            hash: hash.to_string(),
            timestamp: time,
            telemetry,
        }
    }
}