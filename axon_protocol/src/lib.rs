use serde::{Serialize, Deserialize};
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AiProvider {
    OpenAI, Grok, DeepSeek, Meta, Google, Anthropic,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AxonSignature {
    pub provider: AiProvider,
    pub original_content_hash: String,
    pub segment_index: u32,
    pub total_segments: u32,
    pub created_at: i64,
    pub is_hallucination: bool,
}

pub struct AxonEncoder;
impl AxonEncoder {
    pub fn generate_signature(provider: AiProvider, hash: &str, seg: u32, tot: u32, hall: bool) -> AxonSignature {
        AxonSignature {
            provider,
            original_content_hash: hash.to_string(),
            segment_index: seg,
            total_segments: tot,
            created_at: Utc::now().timestamp(),
            is_hallucination: hall,
        }
    }
}
