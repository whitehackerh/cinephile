export interface TvEpisode {
    id: number;
    episode_number: number;
    season_number: number;
    title: String,
    overview: string | null;
    runtime: number | null;
    still_path: string | null;
    air_date: string | null;
    vote_average: number | null;
    production_code: string | null;
    episode_type: string | null;
}
