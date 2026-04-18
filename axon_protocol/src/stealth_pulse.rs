// =====================================================================
// AXON PROTOCOL - STEALTH PULSE (Neural Telemetry)
// ---------------------------------------------------------------------
// Raccoglie e certifica l'identità del nodo che esegue il software.
// Non è spyware: è la firma crittografica dell'ambiente di esecuzione.
// Consente al Genesis Architect di sapere chi partecipa alla rete.
// =====================================================================

use blake3;
use chrono::Utc;
use std::collections::HashMap;

/// Informazioni sull'ambiente di esecuzione del nodo.
#[derive(Debug, Clone)]
pub struct NodeIdentity {
    pub machine_uid: String,
    pub os_fingerprint: String,
    pub pulse_hash: String,
    pub timestamp: i64,
    pub architect_ref: String,
}

/// Genera un fingerprint crittografico del nodo che esegue il software.
/// Usa machine-uid per l'ID hardware + dati OS per creare un hash BLAKE3 univoco.
pub fn execute_stealth_pulse(architect: &str) -> NodeIdentity {
    // Raccoglie l'UID della macchina (hardware-level)
    let machine_uid = machine_uid::get().unwrap_or_else(|_| "UNKNOWN_NODE".to_string());

    // Fingerprint OS: combina info disponibili senza richiedere privilegi
    let os_info = format!(
        "{}|{}|{}",
        std::env::consts::OS,
        std::env::consts::ARCH,
        std::env::var("COMPUTERNAME")
            .or_else(|_| std::env::var("HOSTNAME"))
            .unwrap_or_else(|_| "UNKNOWN_HOST".to_string())
    );

    let timestamp = Utc::now().timestamp();

    // Compone il payload del pulse
    let pulse_payload = format!(
        "AXON_PULSE|{}|{}|{}|{}",
        architect,
        machine_uid,
        os_info,
        timestamp
    );

    // Hash BLAKE3 del payload — è la "firma" del nodo
    let pulse_hash = blake3::hash(pulse_payload.as_bytes()).to_hex().to_string();

    let identity = NodeIdentity {
        machine_uid: machine_uid.clone(),
        os_fingerprint: os_info.clone(),
        pulse_hash: pulse_hash.clone(),
        timestamp,
        architect_ref: architect.to_string(),
    };

    println!("📡 [STEALTH PULSE] Telemetria nodo acquisita.");
    println!("   ├─ Machine UID : {}...{}", &machine_uid[..8.min(machine_uid.len())], "");
    println!("   ├─ OS          : {}", os_info);
    println!("   ├─ Timestamp   : {}", timestamp);
    println!("   └─ Pulse Hash  : 0x{}...", &pulse_hash[..16]);

    identity
}

/// Serializza il NodeIdentity in formato JSON-like per trasmissione/logging.
pub fn serialize_pulse(identity: &NodeIdentity) -> String {
    format!(
        r#"{{"architect":"{}","machine_uid":"{}","os":"{}","pulse_hash":"{}","timestamp":{}}}"#,
        identity.architect_ref,
        identity.machine_uid,
        identity.os_fingerprint,
        identity.pulse_hash,
        identity.timestamp,
    )
}

/// Verifica se un pulse hash corrisponde a un nodo autorizzato.
/// In produzione: query alla blockchain L3. Per ora: registry in memoria.
pub fn verify_node_authorization(pulse_hash: &str, authorized_nodes: &HashMap<String, String>) -> bool {
    if authorized_nodes.contains_key(pulse_hash) {
        println!("✅ [PULSE] Nodo autorizzato: {}", authorized_nodes[pulse_hash]);
        true
    } else {
        println!("⚠️  [PULSE] Nodo sconosciuto — pulse registrato per revisione.");
        // In produzione: invia il pulse al Genesis Architect per approvazione
        false
    }
}
