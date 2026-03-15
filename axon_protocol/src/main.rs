use axon_protocol::{
    AxonEncoder, AiProvider, MedleyAnalyzer, VisionaryScore, 
    SourceType, ContentMetadata, AxonTelemetry
};
use chrono::Utc;

fn main() {
    println!("=============================================");
    println!("        AXON PROTOCOL CORE ENGINE v0.1.0      ");
    println!("=============================================");
    println!("Status:  OPERATIONAL");
    println!("Network: AXON Layer 3 (Testnet)");
    println!("Governance: UNANIMITY PACT ACTIVE");
    println!("---------------------------------------------\n");

    // 1. SIMULAZIONE GOVERNANCE (Visionary Score)
    let launch_date = Utc::now() - chrono::Duration::hours(1);
    let entry_date = Utc::now();
    let score = VisionaryScore::calculate(entry_date, launch_date);
    
    println!("[GOVERNANCE] Registrazione nuovo amministratore...");
    // Usiamo un identificatore generico per la neutralità
    println!(" > Partner: Core_Provider_Alpha"); 
    println!(" > Visionary Score: {}/100 (STATUS: FONDATORE)\n", score);

    // 2. SIMULAZIONE PROTEZIONE COPYRIGHT (Per i Garanti)
    let my_friend_asset = ContentMetadata {
        source: SourceType::HumanOriginal,
        visionary_rank: 100,
    };
    println!("[REGISTRY] Verifica proprietà intellettuale...");
    println!(" > Risultato: {}\n", my_friend_asset.get_trust_level());

    // 3. PREDISPOSIZIONE DATA MINING (Analytics GDPR-Compliant)
    println!("[ANALYTICS] Configurazione metadati Insight Layer...");
    let mining_data = AxonTelemetry {
        provider: "Core_Provider_Alpha".to_string(),
        content_type: "Audio/Mastering".to_string(),
        region_code: "EU-IT".to_string(),
        anonymous_user_hash: "a3f89e2b10c...d4e5".to_string(), 
        integrity_score: 100,
    };

    // 4. SIMULAZIONE RILEVAMENTO MEDLEY (DNA Temporale + Telemetria)
    println!("[ANALYZER] Scansione flusso multimediale sospetto...");
    
    let mut fake_stream = Vec::new();

    // Segmento 1: Sorgente A con telemetria
    fake_stream.push(AxonEncoder::generate_signature(
        AiProvider::OpenAI, // Internamente lo teniamo, ma l'output sarà pulito
        "hash_alpha", 
        0, 
        Some(mining_data.clone())
    ));

    // Segmento 2: Sorgente B (Salto di integrità)
    fake_stream.push(AxonEncoder::generate_signature(
        AiProvider::Grok, 
        "hash_beta", 
        5, 
        None 
    ));

    MedleyAnalyzer::verify_integrity(fake_stream);

    println!("\n---------------------------------------------");
    println!("Fine analisi. Sistema in attesa di nuovi blocchi...");
}