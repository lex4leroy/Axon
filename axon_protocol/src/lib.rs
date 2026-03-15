pub mod governance;
pub mod classifier;
pub mod verifier;
pub mod analytics; 

pub use crate::governance::*;
pub use crate::classifier::*;
pub use crate::verifier::*;
pub use crate::analytics::*; 

#[derive(Debug, Clone, PartialEq)]
pub enum AiProvider {
    OpenAI,
    Grok,
    DeepSeek,
    HumanCreator, // Per proteggere le opere del tuo amico
}

#[derive(Debug, Clone)]
pub struct AxonSignature {
    pub provider: AiProvider,
    pub hash: String,
    pub timestamp: u64,
    pub telemetry: Option<AxonTelemetry>, // Predisposizione per il Data Mining
}

pub struct AxonEncoder;

impl AxonEncoder {
    pub fn generate_signature(
        provider: AiProvider, 
        hash: &str, 
        time: u64, 
        telemetry: Option<AxonTelemetry> 
    ) -> AxonSignature {
        
        // Logica di Data Mining: se presente, l'evento viene registrato
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