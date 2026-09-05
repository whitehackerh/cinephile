use crate::domain::entities::genre::Genre as GenreEntity;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub(crate) struct Genre {
    pub id: i32,
    pub name: String,
}

impl From<GenreEntity> for Genre {
    fn from(entity: GenreEntity) -> Self {
        let (id, name) = entity.into_parts();
        Self { id, name }
    }
}
