export interface TvEpisodeSummary {
  id: number;
  episode_number: number;
  title: string;
  overview: string | null;
  runtime: number | null;
  poster_path: string | null;
  air_date: string | null;
  vote_average: number | null;
}