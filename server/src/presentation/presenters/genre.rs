use serde::{Serialize, Deserialize};

use crate::usecases::dto::genre::Genre as GenreDto;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct Genre {
    pub id: i32,
    pub name: String,
}

impl From<GenreDto> for Genre {
    fn from(genre: GenreDto) -> Self {
        Self {
            id: genre.id,
            name: genre.name,
        }
    }
}