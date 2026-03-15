#[derive(Debug, PartialEq, Clone)]
pub enum SourceType {
    HumanOriginal,
    AiGenerated,
    HybridMedley,
    Unverified,
}

pub struct ContentMetadata {
    pub source: SourceType,
    pub visionary_rank: u32, 
}

impl ContentMetadata {
    pub fn get_trust_level(&self) -> String {
        match self.source {
            SourceType::HumanOriginal => "🛡️ MASSIMA INTEGRITÀ: Opera Umana Protetta".to_string(),
            SourceType::AiGenerated if self.visionary_rank > 80 => "✅ IA CERTIFICATA: Partner Fondatore".to_string(),
            SourceType::HybridMedley => "⚠️ ATTENZIONE: Rilevato Assemblaggio (Medley)".to_string(),
            _ => "❓ SORGENTE NON VERIFICATA".to_string(),
        }
    }
}