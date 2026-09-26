use chrono::{DateTime, Utc};
use crate::domain::entities::work::Work;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CollectionWork {
    id: Uuid,
    target_path: String,
    work: Work,
    added_at: DateTime<Utc>,
}

impl CollectionWork {
    pub fn new(
        target_path: String,
        work: Work
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            target_path,
            work,
            added_at: Utc::now()
        }
    }

    pub fn reconstruct(
        id: Uuid,
        target_path: String,
        work: Work,
        added_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            target_path,
            work,
            added_at
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

    pub fn target_path(&self) -> &str {
        &self.target_path
    }

    pub fn work(&self) -> &Work {
        &self.work
    }

    pub fn added_at(&self) -> DateTime<Utc> {
        self.added_at
    }

    pub fn into_parts(self) -> (
        Uuid, String, Work, DateTime<Utc>
    ) {
        (
            self.id, self.target_path, self.work, self.added_at
        )
    }
}
