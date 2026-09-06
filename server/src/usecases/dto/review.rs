use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::usecases::dto::work::Work;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Review {
    pub id: Uuid,
    pub rating: i32,
    pub content: Option<String>,
    pub work_type: String,
    pub target_path: String,
    pub work: Work,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ReviewWithoutWork {
    pub rating: i32,
    pub content: Option<String>,
    pub work_type: String,
    pub target_path: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}