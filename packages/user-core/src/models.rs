use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct UserRegisteredEvent {
    pub user_id: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StudyPreferences {
    pub preferred_time: Option<String>,
    pub preferred_stye: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrivacySettings {
    pub show_courses: bool
}

impl Default for PrivacySettings {
    fn default() -> Self {
        Self { show_courses: true }
    }
}

pub struct UserProfile {
    pub user_id: String,
    pub email: String,
    pub name: Option<String>,
    pub interest: Vec<String>,
    pub study_preference: Option<StudyPreferences>,
    pub privacy: PrivacySettings,
    pub created_at: DateTime<Utc>
}


