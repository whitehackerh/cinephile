import { useState, useEffect, useCallback } from 'react';
import { apiService } from '@/service/api';
import { Movie } from '@/types/movie';

export const useMovieDetail = (id: string) => {
  const [movie, setMovie] = useState<Movie | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  const fetchMovie = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const data = await apiService.getMovieDetail(id);
      setMovie(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'An unexpected error occurred');
    } finally {
      setLoading(false);
    }
  }, [id]);

  useEffect(() => {
    if (id) {
      fetchMovie();
    }
  }, [id, fetchMovie]);

  return { movie, loading, error, refetch: fetchMovie };
};
