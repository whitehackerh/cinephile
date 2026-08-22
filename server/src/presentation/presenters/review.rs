// use chrono::{DateTime, Utc};
// use serde::{Deserialize, Serialize};
// use uuid::Uuid;

// use crate::{
//     presentation::presenters::work::Work,
//     usecases::dto::review::Review as ReviewOutput
// };

// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct Review {
//     pub id: Uuid,
//     pub rating: i32,
//     pub content: Option<String>,
//     pub work_type: String,
//     pub target_path: String,
//     pub work: Work,
//     pub created_at: DateTime<Utc>,
//     pub updated_at: DateTime<Utc>,
//     pub deleted_at: Option<DateTime<Utc>>,
// }

// impl From<ReviewOutput> for Review {
//     fn from(output: ReviewOutput) -> Self {
//         Self {
//             id: output.id,
//             rating: output.rating,
//             content: output.content,
//             work_type: output.work_type,
//             target_path: output.target_path,
//             work: Work::from(output.work),
//             created_at: output.created_at,
//             updated_at: output.updated_at,
//             deleted_at: output.deleted_at
//         }
//     }
// }