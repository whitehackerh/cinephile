import { Movie } from '@/types/movie';
import { TvEpisode } from '@/types/tvEpisode';
import { TvSeason } from '@/types/tvSeason';
import { TvSeries } from '@/types/tvSeries';

export type WorkType = 'movie' | 'series' | 'season' | 'episode';

export type Work = 
  | ({ work_type: 'movie' } & Movie)
  | ({ work_type: 'series' } & TvSeries)
  | ({ work_type: 'season' } & TvSeason)
  | ({ work_type: 'episode' } & TvEpisode)
  | Movie
  | TvSeries 
  | TvSeason 
  | TvEpisode;