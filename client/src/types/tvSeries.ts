import { Genre } from '@/types/genre'; 
import { TvSeasonSummary } from '@/types/tvSeasonSummary';

export interface TvSeries {
  id: number;
  title: string;
  original_title: string;
  overview: string | null;
  number_of_seasons: number | null;
  number_of_episodes: number | null;
  poster_path: string | null;
  backdrop_path: string | null;
  first_air_date: string | null;
  vote_average: number | null;
  tagline: string | null;
  genres: Genre[];
  season_summaries: TvSeasonSummary[];
}
