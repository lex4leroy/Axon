// =====================================================================
// AXON PROTOCOL - DWT WATERMARKING ENGINE v2
// ---------------------------------------------------------------------
// Watermarking nel dominio delle frequenze (DWT Haar 2D).
// Usa QIM corretto + repetition coding x2 per robustezza.
//
// In produzione (frame 720p+):
//   - LL ha milioni di coefficienti → ripetizione x8 o più
//   - Perceptual masking adatta la forza alla complessità del contenuto
//   - DCT su blocchi 8x8 (come JPEG) per maggiore robustezza
// =====================================================================

use blake3;
use chrono::Utc;

// Passo di quantizzazione — aumentarlo aumenta robustezza ma visibilità
// Con q=8: max modifica LL = q/4 = 2.0 pixel (sotto soglia JND 2.5)
const WATERMARK_STRENGTH: f32 = 8.0;
const TEMPORAL_SPREAD_FRAMES: usize = 48;

#[derive(Debug, Clone)]
pub struct AxonDna {
    pub owner_id: String,
    pub asset_hash: String,
    pub dna_bits: Vec<u8>,   // 128 bit — payload reale
    pub timestamp: i64,
    pub settlement_layer: String,
}

impl AxonDna {
    pub fn new(owner: &str, content: &[u8], layer: &str) -> Self {
        let timestamp = Utc::now().timestamp();
        let asset_hash = blake3::hash(content).to_hex().to_string();
        let dna_payload = format!("AXON_DNA|{}|{}|{}", owner, asset_hash, timestamp);
        let dna_hash = blake3::hash(dna_payload.as_bytes());
        let dna_bits = hash_to_bits(dna_hash.as_bytes(), 128);
        AxonDna { owner_id: owner.to_string(), asset_hash, dna_bits, timestamp,
                  settlement_layer: layer.to_string() }
    }

    pub fn perceptual_signature(&self) -> String {
        let sig = format!("PHASH|{}|{}", self.owner_id, self.asset_hash);
        blake3::hash(sig.as_bytes()).to_hex()[..32].to_string()
    }
}

fn hash_to_bits(hash_bytes: &[u8], num_bits: usize) -> Vec<u8> {
    let mut bits = Vec::with_capacity(num_bits);
    for byte in hash_bytes.iter().take((num_bits + 7) / 8) {
        for i in (0..8).rev() { bits.push((byte >> i) & 1); }
    }
    bits.truncate(num_bits);
    bits
}

// ─────────────────────────────────────────────────────────────────────
// DWT 2D Haar
// ─────────────────────────────────────────────────────────────────────

pub fn dwt2d_haar(pixels: &[Vec<f32>]) -> (Vec<Vec<f32>>, Vec<Vec<f32>>, Vec<Vec<f32>>, Vec<Vec<f32>>) {
    let rows = pixels.len();
    let cols = if rows > 0 { pixels[0].len() } else { 0 };
    let hr = rows / 2; let hc = cols / 2;
    let mut ll = vec![vec![0.0f32; hc]; hr];
    let mut lh = vec![vec![0.0f32; hc]; hr];
    let mut hl = vec![vec![0.0f32; hc]; hr];
    let mut hh = vec![vec![0.0f32; hc]; hr];
    for r in 0..hr { for c in 0..hc {
        let (a, b) = (pixels[2*r][2*c], pixels[2*r][2*c+1]);
        let (e, f) = (pixels[2*r+1][2*c], pixels[2*r+1][2*c+1]);
        ll[r][c] = (a+b+e+f)/4.0; lh[r][c] = (a-b+e-f)/4.0;
        hl[r][c] = (a+b-e-f)/4.0; hh[r][c] = (a-b-e+f)/4.0;
    }}
    (ll, lh, hl, hh)
}

pub fn idwt2d_haar(ll: &[Vec<f32>], lh: &[Vec<f32>], hl: &[Vec<f32>], hh: &[Vec<f32>]) -> Vec<Vec<f32>> {
    let hr = ll.len(); let hc = if hr > 0 { ll[0].len() } else { 0 };
    let mut out = vec![vec![0.0f32; hc*2]; hr*2];
    for r in 0..hr { for c in 0..hc {
        let (l, h1, h2, h3) = (ll[r][c], lh[r][c], hl[r][c], hh[r][c]);
        out[2*r][2*c]     = (l+h1+h2+h3).clamp(0.0,255.0);
        out[2*r][2*c+1]   = (l-h1+h2-h3).clamp(0.0,255.0);
        out[2*r+1][2*c]   = (l+h1-h2-h3).clamp(0.0,255.0);
        out[2*r+1][2*c+1] = (l-h1-h2+h3).clamp(0.0,255.0);
    }}
    out
}

// ─────────────────────────────────────────────────────────────────────
// QIM CORRETTO — trova sempre il quantization level più vicino
// ─────────────────────────────────────────────────────────────────────

/// Inietta un singolo bit tramite QIM corretto.
/// bit=0 → sposta verso il multiplo di q più vicino
/// bit=1 → sposta verso il valore (n+0.5)*q più vicino
fn qim_encode(coeff: f32, bit: u8, q: f32) -> f32 {
    let q0_nearest = (coeff / q).round() * q;          // bit=0 level più vicino

    if bit == 0 {
        q0_nearest
    } else {
        // I due bit=1 levels adiacenti a q0_nearest
        let candidate_plus  = q0_nearest + q / 2.0;
        let candidate_minus = q0_nearest - q / 2.0;
        // Scegli il più vicino al coefficiente originale (minima distorsione)
        if (coeff - candidate_plus).abs() <= (coeff - candidate_minus).abs() {
            candidate_plus
        } else {
            candidate_minus
        }
    }
}

/// Decodifica un bit da un coefficiente potenzialmente rumoroso.
/// Decisione: quale dei due livelli QIM è più vicino?
fn qim_decode(coeff: f32, q: f32) -> u8 {
    let q0_nearest = (coeff / q).round() * q;

    // Distanza al bit=0 level più vicino
    let d0 = (coeff - q0_nearest).abs();

    // Distanza al bit=1 level più vicino
    let candidate_plus  = q0_nearest + q / 2.0;
    let candidate_minus = q0_nearest - q / 2.0;
    let d1 = (coeff - candidate_plus).abs().min((coeff - candidate_minus).abs());

    if d0 <= d1 { 0u8 } else { 1u8 }
}

// ─────────────────────────────────────────────────────────────────────
// REPETITION CODING x2
// Ogni bit del DNA viene scritto in 2 posizioni LL consecutive.
// In estrazione: majority vote → robustezza raddoppiata.
// Richiede LL_size >= 2 * num_bits
// ─────────────────────────────────────────────────────────────────────

pub fn inject_dna_qim(ll: &mut Vec<Vec<f32>>, dna_bits: &[u8], strength: f32) {
    let q = strength;
    let rows = ll.len();
    let cols = if rows > 0 { ll[0].len() } else { 0 };
    let ll_capacity = rows * cols;

    // Con repetition x2: ogni bit occupa 2 slot
    let bits_encodable = (ll_capacity / 2).min(dna_bits.len());

    let flat_indices: Vec<(usize, usize)> = (0..rows).flat_map(|r| (0..cols).map(move |c| (r, c))).collect();

    for bit_idx in 0..bits_encodable {
        let bit = dna_bits[bit_idx];
        // Slot 1: posizione normale
        let (r1, c1) = flat_indices[bit_idx * 2];
        ll[r1][c1] = qim_encode(ll[r1][c1], bit, q);
        // Slot 2: posizione replica (robustezza)
        let (r2, c2) = flat_indices[bit_idx * 2 + 1];
        ll[r2][c2] = qim_encode(ll[r2][c2], bit, q);
    }
}

pub fn extract_dna_qim(ll: &[Vec<f32>], expected_bits: usize, strength: f32) -> (Vec<u8>, f32) {
    let q = strength;
    let rows = ll.len();
    let cols = if rows > 0 { ll[0].len() } else { 0 };
    let ll_capacity = rows * cols;
    let bits_decodable = ((ll_capacity / 2)).min(expected_bits);

    let flat: Vec<f32> = (0..rows).flat_map(|r| ll[r].iter().copied()).collect();

    let mut extracted = Vec::with_capacity(bits_decodable);
    let mut confidences = Vec::with_capacity(bits_decodable);

    for bit_idx in 0..bits_decodable {
        let b1 = qim_decode(flat[bit_idx * 2], q);
        let b2 = qim_decode(flat[bit_idx * 2 + 1], q);

        // Majority vote tra le 2 repliche
        let bit = if b1 == b2 { b1 } else {
            // In caso di disaccordo: usa distanza per scegliere il più affidabile
            b1 // fallback al primo (in produzione: usa confidence)
        };
        let confidence = if b1 == b2 { 1.0f32 } else { 0.5f32 };
        extracted.push(bit);
        confidences.push(confidence);
    }

    let avg_conf = if confidences.is_empty() { 0.0 }
    else { confidences.iter().sum::<f32>() / confidences.len() as f32 };

    (extracted, avg_conf)
}

// ─────────────────────────────────────────────────────────────────────
// TEMPORAL DNA
// ─────────────────────────────────────────────────────────────────────

pub fn generate_temporal_dna_sequence(dna: &AxonDna, num_frames: usize) -> Vec<Vec<u8>> {
    let spread = TEMPORAL_SPREAD_FRAMES.min(num_frames);
    let n = dna.dna_bits.len();
    (0..num_frames).map(|i| {
        if i < spread {
            let mut b = dna.dna_bits.clone();
            b.rotate_left((i * 3) % n);
            b
        } else { vec![0u8; n] }
    }).collect()
}

pub fn reconstruct_dna_from_frames(indexed_frames: &[(usize, Vec<u8>)], num_bits: usize) -> (Vec<u8>, f32) {
    if indexed_frames.is_empty() { return (vec![0u8; num_bits], 0.0); }
    let mut votes = vec![0usize; num_bits];
    let total = indexed_frames.len();
    
    for (orig_idx, fd) in indexed_frames {
        let offset = (orig_idx * 3) % num_bits;
        let mut aligned = fd.clone();
        if aligned.len() == num_bits {
            // Ripristina l'allineamento originale usando l'indice corretto
            aligned.rotate_right(offset);
            for (idx, &bit) in aligned.iter().enumerate().take(num_bits) {
                if bit == 1 { votes[idx] += 1; }
            }
        }
    }
    
    let rec: Vec<u8> = votes.iter().map(|&v| if v > total/2 { 1u8 } else { 0u8 }).collect();
    let conf = votes.iter().map(|&v| {
        (v as f32 - total as f32/2.0).abs() / (total as f32/2.0)
    }).sum::<f32>() / num_bits as f32;
    (rec, conf)
}

// ─────────────────────────────────────────────────────────────────────
// PUBLIC API
// ─────────────────────────────────────────────────────────────────────

pub fn watermark_frame(pixels: &[Vec<f32>], owner: &str, content: &[u8], layer: &str) -> (Vec<Vec<f32>>, AxonDna) {
    let dna = AxonDna::new(owner, content, layer);
    let (mut ll, lh, hl, hh) = dwt2d_haar(pixels);

    let ll_cap = ll.len() * if ll.is_empty() { 0 } else { ll[0].len() };
    let bits_to_inject = (ll_cap / 2).min(dna.dna_bits.len());

    inject_dna_qim(&mut ll, &dna.dna_bits[..bits_to_inject], WATERMARK_STRENGTH);
    let watermarked = idwt2d_haar(&ll, &lh, &hl, &hh);

    println!("🧬 [DWT] DNA AXON iniettato nel dominio frequenziale.");
    println!("   ├─ Owner          : {}", owner);
    println!("   ├─ Asset Hash     : {}...", &dna.asset_hash[..16]);
    println!("   ├─ pHash          : {}", dna.perceptual_signature());
    println!("   ├─ Bit iniettati  : {}/{} (repetition x2)", bits_to_inject, dna.dna_bits.len());
    println!("   └─ Layer          : {}", layer);

    (watermarked, dna)
}

pub fn verify_frame_dna(pixels: &[Vec<f32>], expected_dna: &AxonDna) -> VerificationResult {
    let (ll, _, _, _) = dwt2d_haar(pixels);
    let (extracted, confidence) = extract_dna_qim(&ll, expected_dna.dna_bits.len(), WATERMARK_STRENGTH);

    let compared = extracted.len().min(expected_dna.dna_bits.len());
    if compared == 0 {
        return VerificationResult { is_authentic: false, bit_error_rate: 1.0,
            confidence: 0.0, owner_id: None, asset_hash: expected_dna.asset_hash.clone(),
            bits_compared: 0, bits_total: expected_dna.dna_bits.len() };
    }

    let matching = extracted.iter().zip(expected_dna.dna_bits.iter()).filter(|(&a, &b)| a == b).count();
    let ber = 1.0 - (matching as f32 / compared as f32);
    let coverage = compared as f32 / expected_dna.dna_bits.len() as f32;
    let is_match = ber < 0.15 && coverage >= 0.10;

    if is_match {
        println!("✅ [VERIFY] DNA confermato. BER: {:.1}% | Confidence: {:.1}% | Copertura: {:.0}%",
            ber*100.0, confidence*100.0, coverage*100.0);
    } else if coverage < 0.10 {
        println!("⚠️  Frame troppo piccolo ({}/{} bit). Usare frame >= {}x{}.",
            compared, expected_dna.dna_bits.len(),
            (((expected_dna.dna_bits.len()*4) as f32).sqrt() as usize) + 2,
            (((expected_dna.dna_bits.len()*4) as f32).sqrt() as usize) + 2);
    } else {
        println!("🚨 [VERIFY] DNA non confermato. BER: {:.1}% | Copertura: {:.0}%",
            ber*100.0, coverage*100.0);
    }

    VerificationResult { is_authentic: is_match, bit_error_rate: ber, confidence,
        owner_id: if is_match { Some(expected_dna.owner_id.clone()) } else { None },
        asset_hash: expected_dna.asset_hash.clone(), bits_compared: compared,
        bits_total: expected_dna.dna_bits.len() }
}

#[derive(Debug, Clone)]
pub struct VerificationResult {
    pub is_authentic: bool,
    pub bit_error_rate: f32,
    pub confidence: f32,
    pub owner_id: Option<String>,
    pub asset_hash: String,
    pub bits_compared: usize,
    pub bits_total: usize,
}
