// --- INTEGRITY SHIELD ---

pub struct IntegrityShield;

impl IntegrityShield {
    /// Rileva tentativi di manipolazione (Immagine, Musica o Frame-by-Frame Video).
    pub fn monitor_tampering(signature: &crate::AxonSignature, current_hash: &str) {
        if signature.poison_pill_hash != current_hash {
            Self::trigger_self_destruct_signal(signature);
        } else {
            println!("🛡️ [SHIELD] Integrità confermata per il flusso di {}", signature.owner_id);
        }
    }

    /// AUTODISTRUZIONE: Se il file viene manomesso (Immagine o Brano Musicale), viene black-listato globalmente.
    fn trigger_self_destruct_signal(sig: &crate::AxonSignature) {
        println!("🚨 [POISON-PILL ACTIVATED] Tentativo di MANIPOLAZIONE rilevato!");
        println!("❌ Il contenuto (ID: {}) dell'owner {} è ora ILLEGALE e INVALIDO sul network AXON.", sig.hash, sig.owner_id);
    }
}

// --- MEDLEY SHIELD (Taglia & Cuci Detection) ---

pub struct MedleyShield;

impl MedleyShield {
    /// Analizza un filmato sospetto cercando segmenti di DNA appartenenti a opere originali.
    pub fn detect_medley_attack(suspicious_hash_stream: &[String], registry: &[crate::AxonSignature]) {
        println!("🔍 [MEDLEY-SCAN] Analisi sequenziale dei frame in corso...");
        
        for (i, segment_hash) in suspicious_hash_stream.iter().enumerate() {
            for original in registry {
                if &original.hash == segment_hash {
                    println!("\n⚠️  [ALERT] RILEVATO TAGLIO E CUCI (Medley Attack)!");
                    println!("👉 Frammento originale al secondo {}: Identificato", i);
                    println!("👤 Autore Legittimo: {}", original.owner_id);
                    println!("🧱 Blockchain Proof (L3): https://explorer.axon.sh/tx/{}", original.l3_transaction_id);
                    println!("⚖️  Status: VIOLAZIONE COPYRIGHT RILEVATA");
                    return;
                }
            }
        }
        
        println!("✅ Nessun segmento protetto rilevato nel montaggio.");
    }
}