export interface TvSeasonSummary {
  id: number;
  season_number: number;
  episode_count: number;
  title: string;
  overview: string | null;
  poster_path: string | null;
  air_date: string | null;
  vote_average: number | null;
}
