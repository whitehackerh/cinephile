export interface BaseSummary {
  id: number;
  overview: string | null;
  poster_path: string | null;
}

export interface MovieSummary extends BaseSummary {
  media_type: 'movie';
  title: string;
  release_date: string | null;
}

export interface TvSummary extends BaseSummary {
  media_type: 'tv';
  name: string;
  first_air_date: string | null;
}

export type Work = MovieSummary | TvSummary;

export interface SearchResponse {
  works: Work[];
  page: number;
  total_pages: number;
  total_results: number;
}