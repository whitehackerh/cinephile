use crate::domain::errors::AppError;

pub struct TargetPathParser;

impl TargetPathParser {
    pub fn extract_movie_id(path: &str) -> Result<i64, AppError> {
        let segs: Vec<&str> = path.trim_matches('/').split('/').collect();
        match segs.as_slice() {
            ["movie", id] => id.parse().map_err(|_| AppError::Validation("Invalid movie id".into())),
            _ => Err(AppError::Validation("Invalid movie path format (expected: /movie/{id})".into())),
        }
    }

    pub fn extract_series_id(path: &str) -> Result<i64, AppError> {
        let segs: Vec<&str> = path.trim_matches('/').split('/').collect();
        match segs.as_slice() {
            ["tv", id] => id.parse().map_err(|_| AppError::Validation("Invalid series id".into())),
            _ => Err(AppError::Validation("Invalid series path format (expected: /tv/{id})".into())),
        }
    }

    pub fn extract_season_params(path: &str) -> Result<(i64, i32), AppError> {
        let segs: Vec<&str> = path.trim_matches('/').split('/').collect();
        match segs.as_slice() {
            ["tv", id, "season", s_no] => {
                let series_id = id.parse().map_err(|_| AppError::Validation("Invalid series id".into()))?;
                let season_no = s_no.parse().map_err(|_| AppError::Validation("Invalid season number".into()))?;
                Ok((series_id, season_no))
            }
            _ => Err(AppError::Validation("Invalid season path format (expected: /tv/{id}/season/{season_number})".into())),
        }
    }

    pub fn extract_episode_params(path: &str) -> Result<(i64, i32, i32), AppError> {
        let segs: Vec<&str> = path.trim_matches('/').split('/').collect();
        match segs.as_slice() {
            ["tv", id, "season", s_no, "episode", e_no] => {
                let series_id = id.parse().map_err(|_| AppError::Validation("Invalid series id".into()))?;
                let season_no = s_no.parse().map_err(|_| AppError::Validation("Invalid season number".into()))?;
                let episode_no = e_no.parse().map_err(|_| AppError::Validation("Invalid episode number".into()))?;
                Ok((series_id, season_no, episode_no))
            }
            _ => Err(AppError::Validation("Invalid episode path format (expected: /tv/{id}/season/{season_number}/episode/{episode_number})".into())),
        }
    }
}