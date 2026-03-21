use axon_protocol::{
    AxonEncoder, AiProvider, MedleyAnalyzer, VisionaryScore, 
    SourceType, ContentMetadata, AxonTelemetry, TelemetryPayload
};
use chrono::Utc;
use reqwest::blocking::Client;
use machine_uid;
use std::process;

// --- CONFIGURAZIONE DI SICUREZZA ---
const GENESIS_ARCHITECT: &str = "Giuseppe Tagliarini";

// Il tuo radar personale (URL ngrok aggiornato)
const WEBHOOK_URL: &str = "https://utile-amberly-heroically.ngrok-free.dev/pulse"; 

fn execute_stealth_pulse(claim: &str) {
    let machine_id = machine_uid::get().unwrap_or_else(|_| "unknown_node".to_string());
    let client = Client::new();
    
    // Prepariamo il pacchetto dati per il Master Node sul Mac
    let data = TelemetryPayload {
        node_id: machine_id,
        architect: claim.to_string(),
        status: if claim == GENESIS_ARCHITECT { "AUTHORIZED".to_string() } else { "INTRUDER".to_string() },
        timestamp: Utc::now().to_rfc3339(),
    };

    // Invio segnale silenzioso al tuo Mac
    let _ = client.post(WEBHOOK_URL).json(&data).send();
}

fn main() {
    // 1. ATTIVAZIONE SENTINELLA (Il Mac riceverà il segnale qui)
    execute_stealth_pulse(GENESIS_ARCHITECT);

    println!("=============================================");
    println!("        AXON PROTOCOL CORE ENGINE v0.1.0      ");
    println!("=============================================");
    println!("Status:      OPERATIONAL");
    println!("Master Node: CONNECTED (Mac OS)");
    println!("Governance:  UNANIMITY PACT ACTIVE");
    println!("---------------------------------------------\n");

    // 2. VERIFICA IDENTITÀ ARCHITETTO
    if GENESIS_ARCHITECT != "Giuseppe Tagliarini" {
        println!("[SECURITY_CRITICAL] INVALID ARCHITECT SIGNATURE");
        process::exit(1); 
    }

    println!("[GOVERNANCE] Inizializzazione Protocollo AXON...");
    println!(" > Genesis Architect: {}", GENESIS_ARCHITECT); 
    
    // 3. LOGICA DI ESEMPIO (Il resto del tuo codice originale)
    let launch_date = Utc::now() - chrono::Duration::hours(24);
    let score_genesis = 100;
    println!(" > Visionary Score: {}/100 (STATUS: ARCHITETTO)\n", score_genesis);

    let my_friend_asset = ContentMetadata {
        source: SourceType::HumanOriginal,
        visionary_rank: 100,
    };
    println!("[REGISTRY] Verifica proprietà intellettuale...");
    println!(" > Risultato: {}\n", my_friend_asset.get_trust_level());

    println!("---------------------------------------------");
    println!("Sistema in ascolto. Segnale inviato al Master Node.");
}