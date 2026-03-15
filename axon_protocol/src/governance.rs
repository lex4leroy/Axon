use chrono::{DateTime, Utc, Duration};

#[derive(Debug, Clone)]
pub struct AdminProfile {
    pub name: String,
    pub visionary_score: u32,
    pub entry_timestamp: DateTime<Utc>,
}

pub struct VisionaryScore;

impl VisionaryScore {
    pub fn calculate(entry_date: DateTime<Utc>, launch_date: DateTime<Utc>) -> u32 {
        let duration = entry_date.signed_duration_since(launch_date);
        
        if duration <= Duration::hours(48) {
            100 // Visionario Assoluto (Status Fondatore)
        } else if duration <= Duration::days(30) {
            75  // Leader Strategico
        } else {
            10  // Late Follower
        }
    }
}