import { useState, useEffect, useCallback } from 'react';
import { apiService } from '@/service/api';
import { TvSeason } from '@/types/tvSeason';

export const useTvSeason = (seriesId: string, seasonNumber: string) => {
  const [tvSeason, setTvSeason] = useState<TvSeason | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  const fetchTvSeason = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const data = await apiService.getTvSeason(seriesId, seasonNumber);
      setTvSeason(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'An unexpected error occurred');
    } finally {
      setLoading(false);
    }
  }, [seriesId, seasonNumber]);

  useEffect(() => {
    if (seriesId && seasonNumber != null) {
      fetchTvSeason();
    }
  }, [seriesId, seasonNumber, fetchTvSeason]);

  return { tvSeason, loading, error, refetch: fetchTvSeason };
};
