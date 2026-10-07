use chrono::{DateTime, Utc};
use crate::domain::entities::collection_work::CollectionWork;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Collection {
    id: Uuid,
    user_id: Uuid,
    title: String,
    description: Option<String>,
    cover_image_path: Option<String>,
    works: Vec<CollectionWork>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Collection {
    pub fn new(
        user_id: Uuid,
        title: String,
        description: Option<String>,
    ) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            user_id,
            title,
            description,
            cover_image_path: None,
            works: Vec::new(),
            created_at: now,
            updated_at: now
        }
    }

    pub fn reconstruct(
        id: Uuid,
        user_id: Uuid,
        title: String,
        description: Option<String>,
        cover_image_path: Option<String>,
        works: Vec<CollectionWork>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            user_id,
            title,
            description,
            cover_image_path,
            works,
            created_at,
            updated_at,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn cover_image_path(&self) -> &Option<String> {
        &self.cover_image_path
    }

    pub fn works(&self) -> &Vec<CollectionWork> {
        &self.works
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn into_parts(self) -> (
        Uuid, Uuid, String, Option<String>, Option<String>,
        Vec<CollectionWork>, DateTime<Utc>, DateTime<Utc>,
    ) {
        (
            self.id, self.user_id, self.title, self.description, self.cover_image_path,
            self.works, self.created_at, self.updated_at,
        )
    }

    pub fn sync_cover_image(&mut self) {
        self.cover_image_path = self
            .works
            .iter()
            .find_map(|w| w.work().image_path())
            .map(|s| s.to_string());
    }

    pub fn add_work(&mut self, work: CollectionWork) {
        self.works.push(work);

        if self.cover_image_path.is_none() {
            self.sync_cover_image();
        }

        self.updated_at = Utc::now();
    }

    pub fn update(&mut self, title: String, description: Option<String>) {
        self.title = title;
        self.description = description;
        self.updated_at = Utc::now();
    }
}
