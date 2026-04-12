#[derive(Debug, PartialEq, Clone)]
pub enum SourceType {
    HumanOriginal,
    AiGenerated,
    HybridMedley,
    AudioOriginal,
    MusicRemix,
    VideoOriginal,     // <--- Nuovo: Ripresa Video Originale Certificata
    DeepfakeDetected,  // <--- Nuovo: Rilevato Deepfake o Manipolazione Visiva
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
            SourceType::AudioOriginal => "🎵 AUDIO VERIFICATO: Proprietà Intellettuale Sonora".to_string(),
            SourceType::MusicRemix => "🎚️ REMIX AUTORIZZATO: Tracciamento Campionamento Attivo".to_string(),
            SourceType::VideoOriginal => "📹 VIDEO ORIGINALE: Integrità dei Frame Certificata".to_string(),
            SourceType::DeepfakeDetected => "🚨 ALLERTA DEEPFAKE: Manipolazione Neurale Rilevata!".to_string(),
            SourceType::HybridMedley => "⚠️ ATTENZIONE: Rilevato Assemblaggio (Medley)".to_string(),
            _ => "❓ SORGENTE NON VERIFICATA".to_string(),
        }
    }
}

// --- NUOVO MODULO PER LA TUTELA DEL COPYRIGHT ---

pub struct CopyrightRegistry;

impl CopyrightRegistry {
    /// Verifica se un contenuto simile o identico è già stato registrato sul Ledger L3.
    pub fn verify_uniqueness(content_hash: &str) -> bool {
        // Simulazione di una query al registro L3 (AppChain)
        // In produzione, verificherebbe la presenza dell'hash (o p-hash simile)
        let simulated_registry = vec!["0xGIÀ_REGISTRATO"]; 

        if simulated_registry.contains(&content_hash) {
            println!("🚨 [COPYRIGHT ALERT] Hash {} già presente nel registro L3!", content_hash);
            println!("❌ Tentativo di registrazione respinto: Possibile furto di identità digitale.");
            return false;
        }

        println!("✅ [UNIQUE] Contenuto originale confermato (First-Born status).");
        true
    }
}