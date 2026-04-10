import { Genre } from '@/types/genre'; 

export interface Movie {
  id: number;
  title: string;
  original_title: string;
  overview: string | null;
  poster_path: string | null;
  backdrop_path: string | null;
  release_date: string | null;
  runtime: number | null;
  vote_average: number | null;
  tagline: string | null;
  genres: Genre[];
}
