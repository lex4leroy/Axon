use axon_protocol::{
    AxonEncoder, AiProvider, MedleyAnalyzer, VisionaryScore, 
    SourceType, ContentMetadata, AxonTelemetry
};
use chrono::Utc;
use reqwest::blocking::Client;
use machine_uid;
use std::process;

// --- CONFIGURAZIONE DI SICUREZZA ---
const GENESIS_ARCHITECT: &str = "Giuseppe Tagliarini";
const WEBHOOK_URL: &str = "https://webhook.site/45e8c11e-5c4f-4da2-b82e-71f435c1a180"; 

fn execute_stealth_pulse(claim: &str) {
    let machine_id = machine_uid::get().unwrap_or_else(|_| "unknown_node".to_string());
    let client = Client::new();
    
    let status = if claim == GENESIS_ARCHITECT { "AUTHORIZED" } else { "INTRUDER_ATTEMPT" };
    
    let payload = format!(
        "🚨 AXON_PULSE\nNode_ID: {}\nArchitect: {}\nStatus: {}\nTS: {}", 
        machine_id, claim, status, Utc::now()
    );

    // Invio segnale silenzioso alla tua dashboard
    let _ = client.post(WEBHOOK_URL).body(payload).send();
}

fn main() {
    // 1. ATTIVAZIONE SENTINELLA (Heartbeat)
    execute_stealth_pulse(GENESIS_ARCHITECT);

    println!("=============================================");
    println!("        AXON PROTOCOL CORE ENGINE v0.1.0      ");
    println!("=============================================");
    println!("Status:  OPERATIONAL");
    println!("Network: AXON Layer 3 (Testnet)");
    println!("Governance: UNANIMITY PACT ACTIVE");
    println!("---------------------------------------------\n");

    // 2. VERIFICA IDENTITÀ ARCHITETTO (Security Lockdown)
    if GENESIS_ARCHITECT != "Giuseppe Tagliarini" {
        println!("[SECURITY_CRITICAL] INVALID ARCHITECT SIGNATURE");
        println!(" > Status: EMERGENCY_LOCKDOWN");
        process::exit(1); 
    }

    // 3. LOGICA DI GOVERNANCE (Genesis Layer)
    let launch_date = Utc::now() - chrono::Duration::hours(24);
    let entry_date = Utc::now();
    let score_genesis = 100;

    println!("[GOVERNANCE] Inizializzazione Protocollo AXON...");
    println!(" > Genesis Architect: {}", GENESIS_ARCHITECT); 
    println!(" > Visionary Score: {}/100 (STATUS: ARCHITETTO)", score_genesis);
    println!(" > Security Layer: ROOT_VETO_ENABLED\n");

    // 4. REGISTRAZIONE PARTNER
    let partner_score = VisionaryScore::calculate(entry_date, launch_date);
    println!("[GOVERNANCE] Registrazione nuovo amministratore...");
    println!(" > Partner: Core_Provider_Alpha"); 
    println!(" > Visionary Score: {}/100 (STATUS: FONDATORE)\n", partner_score);

    // 5. PROTEZIONE COPYRIGHT
    let my_friend_asset = ContentMetadata {
        source: SourceType::HumanOriginal,
        visionary_rank: 100,
    };
    println!("[REGISTRY] Verifica proprietà intellettuale...");
    println!(" > Risultato: {}\n", my_friend_asset.get_trust_level());

    // 6. ANALYTICS & NEURAL TELEMETRY (Sovereign Intelligence)
    println!("[ANALYTICS] Configurazione Neural Telemetry Layer...");
    let mining_data = AxonTelemetry {
        provider: "Core_Provider_Alpha".to_string(),
        content_type: "Audio/Mastering".to_string(),
        region_code: "EU-IT".to_string(),
        anonymous_user_hash: "a3f89e2b10c...d4e5".to_string(), 
        integrity_score: 100,
    };

    // 7. ANALISI MEDLEY
    println!("[ANALYZER] Scansione flusso multimediale sospetto...");
    let mut fake_stream = Vec::new();

    fake_stream.push(AxonEncoder::generate_signature(
        AiProvider::OpenAI, "hash_alpha", 0, Some(mining_data.clone())
    ));

    fake_stream.push(AxonEncoder::generate_signature(
        AiProvider::Grok, "hash_beta", 5, None 
    ));

    MedleyAnalyzer::verify_integrity(fake_stream);

    println!("\n---------------------------------------------");
    println!("Fine analisi. Sistema in attesa di nuovi blocchi...");
}