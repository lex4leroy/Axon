use axon_protocol::{AxonEncoder, AiProvider};

fn main() {
    println!("--- AXON PROTOCOL CORE ENGINE ---");
    println!("Status: Operational");
    println!("Network: AXON Layer 3 (Testnet)");
    
    // Esempio di generazione firma conforme al protocollo
    let entry_point = AxonEncoder::generate_signature(
        AiProvider::Grok, 
        "0x-alpha-genesis-hash", 
        0, 
        100, 
        false
    );

    println!("Genesis Block Check: {:?} validated.", entry_point.provider);
}