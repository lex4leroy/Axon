use axon_protocol::TelemetryPayload; // Corretto con la 'o'
use axum::{routing::post, Router, Json};
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

#[tokio::main]
async fn main() {
    // Definiamo la rotta per ricevere i segnali
    let app = Router::new().route("/pulse", post(receive_pulse));

    // Ci mettiamo in ascolto sulla porta 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🍎 AXON COMMAND CENTER (Mac) - ONLINE");
    println!("> In ascolto sulla porta 3000...");
    
    axum::serve(listener, app).await.unwrap();
}

// Funzione corretta per gestire il segnale in arrivo
async fn receive_pulse(Json(payload): Json<TelemetryPayload>) {
    println!("🚨 PULSE RICEVUTO dal nodo: {}", payload.node_id);
    
    // Il Mac ti avvisa a voce!
    let _ = Command::new("say")
        .arg(format!("New signal from {}", payload.node_id))
        .spawn();

    // Scrive il segnale nel tuo registro privato (Blockchain)
    let log_entry = serde_json::to_string(&payload).unwrap();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("axon_private_ledger.json")
        .unwrap();

    if let Err(e) = writeln!(file, "{}", log_entry) {
        eprintln!("Errore di scrittura: {}", e);
    }
}