// =====================================================================
// AXON PROTOCOL - GAME THEORY & ECONOMY MODULE
// ---------------------------------------------------------------------
// Questo modulo gestisce la Teoria dei Giochi del protocollo:
// 1. STAKING: Garanzia finanziaria per validare.
// 2. REPUTATION: "Truth Score" cumulativo basato sulla veridicità.
// 3. SLASHING: Penalità per comportamenti malevoli o "menzogne".
// =====================================================================

#[derive(Debug, Clone)]
pub struct ValidatorNode {
    pub node_id: String,
    pub staked_balance: u128,    // In AXON Gwei
    pub reputation_score: u32,   // Truth Score (0-100)
}

pub struct AxonEconomy;

impl AxonEconomy {
    
    /// PREMIO PER LA VERITÀ: Aumenta la reputazione se il contenuto viene convalidato.
    pub fn reward_truth(node: &mut ValidatorNode) {
        if node.reputation_score < 100 {
            node.reputation_score += 2;
        }
        println!(
            "✅ [REWARD] Integrità confermata per il Nodo {}. Nuova Reputazione: {}", 
            node.node_id, 
            node.reputation_score
        );
    }

    /// SLASHING - PENALIZZA CHI MENTE: Riduciamo stake e reputazione.
    pub fn slash_bad_actor(node: &mut ValidatorNode, factor: f64) {
        let penalty = (node.staked_balance as f64 * factor) as u128;
        node.staked_balance = node.staked_balance.saturating_sub(penalty);
        
        // Crollo della reputazione
        node.reputation_score = node.reputation_score.saturating_sub(25);
        
        println!(
            "🚨 [SLASHING] Rilevata frode nel Nodo {}. Penale: {} Gwei. Punteggio Verità: {}", 
            node.node_id, 
            penalty, 
            node.reputation_score
        );
    }

    /// ESCROW: Simula il blocco fondi durante un audit legale.
    pub fn lock_escrow(amount: u128) -> String {
        format!("🔒 [ESCROW] Fondi bloccati per auditing: {} Gwei", amount)
    }
}
