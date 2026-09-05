use crate::domain::errors::AppError;

pub struct TargetPathParser;

impl TargetPathParser {
    pub fn extract_movie_id(path: &str) -> Result<i32, AppError> {
        let segs: Vec<&str> = path.trim_matches('/').split('/').collect();
        match segs.as_slice() {
            ["movie", id] => {
                let parsed_id: i32 = id
                    .parse()
                    .map_err(|_| AppError::Validation("Invalid movie id".into()))?;
                if parsed_id < 1 {
                    return Err(AppError::Validation("Movie id must be >= 1".into()));
                }
                Ok(parsed_id)
            }
            _ => Err(AppError::Validation("Invalid movie path format (expected: /movie/{id})".into())),
        }
    }

    pub fn extract_series_id(path: &str) -> Result<i32, AppError> {
        let segs: Vec<&str> = path.trim_matches('/').split('/').collect();
        match segs.as_slice() {
            ["tv", id] => {
                let parsed_id: i32 = id
                    .parse()
                    .map_err(|_| AppError::Validation("Invalid series id".into()))?;
                if parsed_id < 1 {
                    return Err(AppError::Validation("Series id must be >= 1".into()));
                }
                Ok(parsed_id)
            }
            _ => Err(AppError::Validation("Invalid series path format (expected: /tv/{id})".into())),
        }
    }

    pub fn extract_season_params(path: &str) -> Result<(i32, i32), AppError> {
        let segs: Vec<&str> = path.trim_matches('/').split('/').collect();
        match segs.as_slice() {
            ["tv", series_id, "season", season_no] => {
                let parsed_series_id: i32 = series_id
                    .parse()
                    .map_err(|_| AppError::Validation("Invalid series id".into()))?;
                let parsed_season_no: i32 = season_no
                    .parse()
                    .map_err(|_| AppError::Validation("Invalid season number".into()))?;

                if parsed_series_id < 1 {
                    return Err(AppError::Validation("Series id must be >= 1".into()));
                }
                if parsed_season_no < 0 {
                    return Err(AppError::Validation("Season number must be >= 0".into()));
                }

                Ok((parsed_series_id, parsed_season_no))
            }
            _ => Err(AppError::Validation("Invalid season path format (expected: /tv/{id}/season/{no})".into())),
        }
    }

    pub fn extract_episode_params(path: &str) -> Result<(i32, i32, i32), AppError> {
        let segs: Vec<&str> = path.trim_matches('/').split('/').collect();
        match segs.as_slice() {
            ["tv", series_id, "season", season_no, "episode", episode_no] => {
                let parsed_series_id: i32 = series_id
                    .parse()
                    .map_err(|_| AppError::Validation("Invalid series id".into()))?;
                let parsed_season_no: i32 = season_no
                    .parse()
                    .map_err(|_| AppError::Validation("Invalid season number".into()))?;
                let parsed_episode_no: i32 = episode_no
                    .parse()
                    .map_err(|_| AppError::Validation("Invalid episode number".into()))?;

                if parsed_series_id < 1 {
                    return Err(AppError::Validation("Series id must be >= 1".into()));
                }
                if parsed_season_no < 0 {
                    return Err(AppError::Validation("Season number must be >= 0".into()));
                }
                if parsed_episode_no < 0 {
                    return Err(AppError::Validation("Episode number must be >= 0".into()));
                }

                Ok((parsed_series_id, parsed_season_no, parsed_episode_no))
            }
            _ => Err(AppError::Validation("Invalid episode path format (expected: /tv/{id}/season/{no}/episode/{no})".into())),
        }
    }
}