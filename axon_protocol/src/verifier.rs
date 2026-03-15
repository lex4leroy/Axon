use crate::AxonEncoder;

pub struct MedleyAnalyzer;

impl MedleyAnalyzer {
    pub fn verify_integrity(stream: Vec<crate::AxonSignature>) {
        println!("--- Analisi Flusso Media in corso ---");
        let mut last_provider = None;

        for (i, sig) in stream.iter().enumerate() {
            if let Some(prev) = last_provider {
                if prev != sig.provider {
                    println!("🚨 ALLERTA MEDLEY: Salto di sorgente al segmento {}. Da {:?} a {:?}", i, prev, sig.provider);
                }
            }
            last_provider = Some(sig.provider.clone());
        }
        println!("--- Analisi completata ---\n");
    }
}