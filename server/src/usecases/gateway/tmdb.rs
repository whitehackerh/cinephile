use async_trait::async_trait;
use crate::usecases::dto::search::SearchOutput;
use crate::domain::{
    entities::{
        movie::Movie,
        tv_season::TvSeason,
        tv_series::TvSeries
    },
    errors::AppError
};

#[async_trait]
pub(crate) trait TmdbGateway: Send + Sync {
    async fn fetch_search_results(&self, query: &str, page: u32) -> Result<SearchOutput, AppError>;
    async fn fetch_movie_by_id(&self, id: i32) -> Result<Movie, AppError>;
    async fn fetch_tv_series_by_id(&self, id: i32) -> Result<TvSeries, AppError>;
    async fn fetch_tv_season(&self, series_id: i32, season_number: i32) -> Result<TvSeason, AppError>;
}
