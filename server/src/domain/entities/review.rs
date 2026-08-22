use chrono::{DateTime, Utc};
use crate::domain::{
    entities::work::Work,
    errors::AppError
};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub(crate) struct Review {
    id: Uuid,
    user_id: Uuid,
    rating: i32,
    content: Option<String>,
    target_path: String,
    work: Work,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>
}

impl Review {
    pub fn new(
        user_id: Uuid,
        rating: i32,
        content: Option<String>,
        target_path: String,
        work: Work,
    ) -> Result<Self, AppError> {
        if rating < 0 || rating > 100 {
            return Err(AppError::Validation(
                "Rating must be between 0 and 100".to_string(),
            ));
        }

        let now = Utc::now();
        
        Ok(Self {
            id: Uuid::new_v4(),
            user_id,
            rating,
            content,
            target_path,
            work,
            created_at: now,
            updated_at: now,
            deleted_at: None
        })
    }

    pub fn reconstruct(
        id: Uuid,
        user_id: Uuid,
        rating: i32,
        content: Option<String>,
        target_path: String,
        work: Work,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        deleted_at: Option<DateTime<Utc>>
    ) -> Self {
        Self {
            id,
            user_id,
            rating,
            content,
            target_path,
            work,
            created_at,
            updated_at,
            deleted_at
        }
    }

    pub fn work_type(&self) -> &'static str {
        match self.work {
            Work::Movie(_) => "movie",
            Work::TvSeries(_) => "series",
            Work::TvSeason(_) => "season",
            Work::TvEpisode(_) => "episode",
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn user_id(&self) -> Uuid {
        self.id
    }

    pub fn rating(&self) -> i32 {
        self.rating
    }

    pub fn content(&self) -> &Option<String> {
        &self.content
    }

    pub fn target_path(&self) -> &str {
        &self.target_path
    }

    pub fn work(&self) -> &Work {
        &self.work
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn deleted_at(&self) -> &Option<DateTime<Utc>> {
        &self.deleted_at
    }

    pub fn into_parts(self) -> (
        Uuid, Uuid, i32, Option<String>, String,
        Work, DateTime<Utc>, DateTime<Utc>, Option<DateTime<Utc>>
    ) {
        (
            self.id, self.user_id, self.rating, self.content, self.target_path,
            self.work, self.created_at, self.updated_at, self.deleted_at
        )
    }
}