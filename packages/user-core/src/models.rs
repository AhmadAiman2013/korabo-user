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

#[derive(Serialize, Deserialize, Debug)]
pub struct UserProfile {
    pub user_id: String,
    pub email: String,
    pub name: Option<String>,
    pub interests: Vec<String>,
    pub study_preferences: Option<StudyPreferences>,
    pub privacy: PrivacySettings,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateProfileRequest {
    pub name: Option<String>,
    pub interest: Option<Vec<String>>,
    pub study_preferences: Option<StudyPreferences>,
}

#[derive(Debug, Deserialize)]
pub struct AddCourseRequest {
    pub course_id: String,
}

impl AddCourseRequest {
    pub fn normalized_course_id(&self) -> String {
        self.course_id.trim().to_uppercase().replace(" ","")
    }
}

#[derive(Debug, Serialize)]
pub struct CourseAddedEvent {
    pub course_id: String,
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseItem {
    pub course_id: String,
    pub added_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ProfileItem {
    #[serde(rename = "PK")]
    pub pk: String,
    #[serde(rename = "SK")]
    pub sk: String,
    #[serde(flatten)]
    pub profile: UserProfile,
}

#[derive(Debug, Serialize)]
pub struct CourseRecord {
    #[serde(rename = "PK")]
    pub pk: String,
    #[serde(rename = "SK")]
    pub sk: String,
    pub course_id: String,
    pub added_at: DateTime<Utc>,
}


