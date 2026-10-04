use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::usecases::dto::work::Work;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct CollectionWork {
    pub id: Uuid,
    pub target_path: String,
    pub work_type: String,
    pub work: Work,
    pub added_at: DateTime<Utc>,
}

pub(crate) struct CollectionWorkForReconstruct {
    pub id: Uuid,
    pub target_path: String,
    pub work_type: String,
    pub added_at: DateTime<Utc>
}
