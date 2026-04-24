use axon_protocol::{
    SettlementLayer, AssetType, AssetRegistry, LegacyImporter,
    MedleyShield, AxonEncoder, AiProvider,
    execute_stealth_pulse, serialize_pulse,
    watermark_frame, verify_frame_dna,
    generate_temporal_dna_sequence, reconstruct_dna_from_frames,
};
use chrono::Utc;
use std::env;

const GENESIS_ARCHITECT: &str = "Giuseppe Tagliarini";
const CURRENT_VERSION: &str = "2026.1.6 (Sovereign L3 Edition)";

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("=============================================");
    println!("      AXON SOVEREIGN L3 NETWORK v{}     ", CURRENT_VERSION);
    println!("=============================================");

    // STEALTH PULSE
    let node_identity = execute_stealth_pulse(GENESIS_ARCHITECT);
    let pulse_json = serialize_pulse(&node_identity);
    println!("   Pulse: {}", &pulse_json[..80.min(pulse_json.len())]);
    println!();

    if args.len() > 1 {
        match args[1].as_str() {

            "REGISTER_ASSET" => {
                let owner = args.iter().position(|r| r == "--owner")
                    .and_then(|i| args.get(i+1)).map(|s| s.as_str()).unwrap_or("Anonymous");
                let hash = format!("BLAKE3-{}", Utc::now().timestamp_nanos_opt().unwrap_or(0));
                AssetRegistry::register(AssetType::Movie, owner, &hash, SettlementLayer::AxonChronos);
            }

            "VERIFY_MEDLEY" => {
                println!("Scanning Sovereign L3 Ledger for Splicing Breach (using pHash)...");
                let original_hash = "ORIGINAL_DNA_772";
                let original_phash = "PHASH_5a1b3c9";
                let original_sig = AxonEncoder::generate_signature(
                    AssetType::Movie, AiProvider::HumanCreator,
                    SettlementLayer::AxonChronos, original_hash, original_phash,
                    chrono::Utc::now().timestamp() as u64,
                    "Architect_Tagliarini", "0xRoyalty", "0xPoison", None
                );
                let registry = vec![original_sig];
                let fake_stream = vec!["PHASH_X".to_string(), original_phash.to_string(), "PHASH_Y".to_string()];
                MedleyShield::detect_medley_attack(&fake_stream, &registry);
            }

            "IMPORT_PATENT" => {
                let patent_id = args.iter().position(|r| r == "--id")
                    .and_then(|i| args.get(i+1)).map(|s| s.as_str()).expect("Missing --id");
                LegacyImporter::import_google_patent(patent_id);
            }

            "DEMO_WATERMARK" => {
                println!("=== DEMO DWT Watermarking ===\n");

                // Frame 32x32 — LL sarà 16x16 = 256 coefficienti, sufficiente per 128 bit DNA
                // In produzione: frame video reale (720p+) ha milioni di coefficienti
                let size = 32usize;
                let original_frame: Vec<Vec<f32>> = (0..size).map(|r| {
                    (0..size).map(|c| {
                        // Gradiente sinusoidale per simulare texture reale
                        let val = 128.0 + 64.0 * ((r as f32 * 0.3).sin() + (c as f32 * 0.2).cos());
                        val.clamp(0.0, 255.0)
                    }).collect()
                }).collect();

                println!("Frame {}x{} — sub-banda LL: {}x{} = {} coefficienti",
                    size, size, size/2, size/2, (size/2)*(size/2));

                // Watermarking
                let (watermarked, dna) = watermark_frame(
                    &original_frame, GENESIS_ARCHITECT,
                    b"axon_demo_video_content_v2026", "AxonChronos"
                );

                // Differenza massima pixel
                let max_diff = original_frame.iter().zip(watermarked.iter())
                    .flat_map(|(ro, rw)| ro.iter().zip(rw.iter()).map(|(o, w)| (o - w).abs()))
                    .fold(0.0f32, f32::max);
                println!("\nDiff massima pixel: {:.2} (soglia JND: 2.5)", max_diff);

                // Verifica sul frame watermarkato pulito
                println!("\n[Test 1] Verifica frame watermarkato (nessun attacco):");
                let r1 = verify_frame_dna(&watermarked, &dna);
                println!("   Bit confrontati: {}/{}", r1.bits_compared, r1.bits_total);

                // Simula recompressione leggera (rumore ±3)
                println!("\n[Test 2] Simulazione recompressione leggera (+/-3):");
                let compressed: Vec<Vec<f32>> = watermarked.iter().map(|row| {
                    row.iter().enumerate().map(|(c, &v)| {
                        (v + ((c as f32 * 1.7).sin() * 3.0)).clamp(0.0, 255.0)
                    }).collect()
                }).collect();
                let r2 = verify_frame_dna(&compressed, &dna);
                println!("   Bit confrontati: {}/{}", r2.bits_compared, r2.bits_total);

                // Simula screen recording aggressivo (rumore ±8)
                println!("\n[Test 3] Simulazione screen recording aggressivo (+/-8):");
                let screen_rec: Vec<Vec<f32>> = watermarked.iter().map(|row| {
                    row.iter().enumerate().map(|(c, &v)| {
                        (v + ((c as f32 * 2.7).sin() * 8.0)).clamp(0.0, 255.0)
                    }).collect()
                }).collect();
                let r3 = verify_frame_dna(&screen_rec, &dna);
                println!("   Bit confrontati: {}/{}", r3.bits_compared, r3.bits_total);

                // Temporal DNA
                println!("\n[Test 4] Temporal DNA — 40% frame loss:");
                let seq = generate_temporal_dna_sequence(&dna, 30);
                let surviving: Vec<(usize, Vec<u8>)> = seq.iter().enumerate()
                    .filter(|(i, _)| i % 5 != 0 && i % 7 != 0)
                    .map(|(i, f)| (i, f.clone())).collect();
                let (rec, conf) = reconstruct_dna_from_frames(&surviving, dna.dna_bits.len());
                let ok = rec.iter().zip(dna.dna_bits.iter()).filter(|(a,b)| a == b).count();
                println!("   Frame: {}/{} | Bit DNA: {}/{} corretti | Confidence: {:.1}%",
                    surviving.len(), seq.len(), ok, dna.dna_bits.len(), conf * 100.0);
            }

            _ => println!("Comandi: REGISTER_ASSET | VERIFY_MEDLEY | IMPORT_PATENT | DEMO_WATERMARK"),
        }
        return;
    }

    println!("Status:           OPERATIONAL");
    println!("Chronos PoH:      ACTIVE");
    println!("Sovereign ZK:     ACTIVE");
    println!("DWT Watermark:    ACTIVE");
    println!("Stealth Pulse:    ACTIVE");
}
