use axon_protocol::TelemetryPayload;
use axum::{routing::post, Router, Json};
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/pulse", post(receive_pulse));

    // Ascolto sulla porta 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🍎 AXON COMMAND CENTER (INTEL WINDOWS) - ONLINE");
    println!("> In ascolto sulla porta 3000...");
    
    axum::serve(listener, app).await.unwrap();
}

async fn receive_pulse(Json(payload): Json<TelemetryPayload>) {
    println!("🚨 PULSE RICEVUTO dal nodo: {}", payload.node_id);
    
    // --- ADATTAMENTO PER WINDOWS (PowerShell Speech) ---
    let alert_msg = format!("New signal from {}", payload.node_id);
    let ps_script = format!(
        "Add-Type -AssemblyName System.Speech; (New-Object System.Speech.Synthesis.SpeechSynthesizer).Speak('{}')", 
        alert_msg
    );
    
    let _ = Command::new("powershell")
        .arg("-Command")
        .arg(ps_script)
        .spawn();

    // --- LOG NEL REGISTRO PRIVATO ---
    let log_entry = serde_json::to_string(&payload).unwrap();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("axon_private_ledger.json")
        .unwrap();

    if let Err(e) = writeln!(file, "{}", log_entry) {
        eprintln!("Errore di scrittura nel registro: {}", e);
    }
}