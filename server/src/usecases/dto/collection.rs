use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::usecases::dto::collection_work::{
    CollectionWork,
    CollectionWorkForReconstruct
};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Collection {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub cover_image_path: Option<String>,
    pub works: Vec<CollectionWork>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub(crate) struct CollectionForReconstruct {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub cover_image_path: Option<String>,
    pub works: Vec<CollectionWorkForReconstruct>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
