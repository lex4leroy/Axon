use axon_protocol::TelemetryPayload; 
use chrono::Utc;
use reqwest::blocking::Client;
use std::process::{Command, self};
use std::time::Duration;
use std::thread;

// --- CONFIGURAZIONE DI SICUREZZA ---
const GENESIS_ARCHITECT: &str = "Giuseppe Tagliarini";
const WEBHOOK_URL: &str = "https://utile-amberly-heroically.ngrok-free.dev/pulse"; 
const CURRENT_VERSION: &str = "1.0.0"; // Versione attuale

fn execute_stealth_pulse(claim: &str) -> String {
    // RECUPERO ID UNICO HARDWARE
    let machine_id = machine_uid::get().unwrap_or_else(|_| "unknown_node".to_string()); 
    
    let client = Client::new();
    
    let data = TelemetryPayload {
        node_id: machine_id,
        architect: claim.to_string(),
        status: if claim == GENESIS_ARCHITECT { "AUTHORIZED".to_string() } else { "INTRUDER".to_string() },
        timestamp: Utc::now().to_rfc3339(),
        version: Some(CURRENT_VERSION.to_string()), 
    };

    match client.post(WEBHOOK_URL).json(&data).send() {
        Ok(resp) => resp.json::<String>().unwrap_or_else(|_| "ERROR".to_string()),
        Err(_) => "OFFLINE".to_string(),
    }
}

fn main() {
    println!("=============================================");
    println!("      AXON PROTOCOL CORE ENGINE v{}      ", CURRENT_VERSION);
    println!("=============================================");
    println!("Architect:   {}", GENESIS_ARCHITECT);
    println!("Status:      OPERATIONAL (Active Node)");
    println!("---------------------------------------------");

    if GENESIS_ARCHITECT != "Giuseppe Tagliarini" {
        println!("[SECURITY_CRITICAL] INVALID ARCHITECT SIGNATURE. TERMINATING.");
        process::exit(1); 
    }

    loop {
        let response = execute_stealth_pulse(GENESIS_ARCHITECT);

        match response.as_str() {
            "ACK" => {
                println!("[{}] Status: ✅ ACK (Sincronizzato)", Utc::now().format("%H:%M:%S"));
            },
            "UPDATE_REQUIRED" => {
                println!("\n⚠️ [PROTOCOL] Ricevuto ordine di UPGRADE dal Master Node!");
                
                // 1. Scarica il nuovo codice
                println!(" > Esecuzione: git pull origin main...");
                let status = Command::new("git")
                    .args(["pull", "origin", "main"])
                    .status();

                if status.is_ok() {
                    println!(" > Codice aggiornato con successo.");
                    println!(" > Riavvio motore: cargo run...");
                    
                    // 2. Riavvia usando il binario predefinito nel Cargo.toml
                    Command::new("cargo")
                        .arg("run")
                        .spawn()
                        .expect("Fallimento durante il riavvio automatico");
                    
                    process::exit(0); // Chiude la vecchia istanza
                } else {
                    println!(" > ❌ Errore durante il pull. Nodo bloccato alla v{}.", CURRENT_VERSION);
                }
            },
            "OFFLINE" => {
                println!("[{}] Status: 📡 Ricerca Master Node (Mac/ngrok offline?)...", Utc::now().format("%H:%M:%S"));
            },
            _ => {
                println!("[{}] Status: 🔍 Risposta inattesa: {}", Utc::now().format("%H:%M:%S"), response);
            }
        }

        thread::sleep(Duration::from_secs(60));
    }
}