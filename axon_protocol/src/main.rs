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

    // 1. INIZIALIZZAZIONE GOVERNANCE (Genesis Layer)
    // Impostiamo l'anzianità del creatore per il punteggio massimo
    let launch_date = Utc::now() - chrono::Duration::hours(24);
    let entry_date = Utc::now();
    let score_genesis = 100;

    println!("[GOVERNANCE] Inizializzazione Protocollo AXON...");
    // --- INSERISCI IL TUO NOME QUI SOTTO ---
    println!(" > Genesis Architect: [Giuseppe Tagliarini]"); 
    println!(" > Visionary Score: {}/100 (STATUS: ARCHITETTO)", score_genesis);
    println!(" > Security Layer: ROOT_VETO_ENABLED\n");

    // 2. REGISTRAZIONE PARTNER (Simulazione ingresso terzi)
    let partner_score = VisionaryScore::calculate(entry_date, launch_date);
    
    println!("[GOVERNANCE] Registrazione nuovo amministratore...");
    println!(" > Partner: Core_Provider_Alpha"); 
    println!(" > Visionary Score: {}/100 (STATUS: FONDATORE)\n", partner_score);

    // 3. SIMULAZIONE PROTEZIONE COPYRIGHT
    let my_friend_asset = ContentMetadata {
        source: SourceType::HumanOriginal,
        visionary_rank: 100,
    };
    println!("[REGISTRY] Verifica proprietà intellettuale...");
    println!(" > Risultato: {}\n", my_friend_asset.get_trust_level());

    // 4. PREDISPOSIZIONE DATA MINING (Analytics GDPR-Compliant)
    println!("[ANALYTICS] Configurazione metadati Insight Layer...");
    let mining_data = AxonTelemetry {
        provider: "Core_Provider_Alpha".to_string(),
        content_type: "Audio/Mastering".to_string(),
        region_code: "EU-IT".to_string(),
        anonymous_user_hash: "a3f89e2b10c...d4e5".to_string(), 
        integrity_score: 100,
    };

    // 5. SIMULAZIONE RILEVAMENTO MEDLEY
    println!("[ANALYZER] Scansione flusso multimediale sospetto...");
    
    let mut fake_stream = Vec::new();

    fake_stream.push(AxonEncoder::generate_signature(
        AiProvider::OpenAI, 
        "hash_alpha", 
        0, 
        Some(mining_data.clone())
    ));

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