import { useState, useEffect, useCallback } from 'react';
import { apiService } from '@/service/api';
import { TvEpisode } from '@/types/tvEpisode';

export const useTvEpisode = (seriesId: string, seasonNumber: string, episodeNumber: string) => {
  const [tvEpisode, setTvEpisode] = useState<TvEpisode | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  const fetchTvEpisode = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const data = await apiService.getTvEpisode(seriesId, seasonNumber, episodeNumber);
      setTvEpisode(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'An unexpected error occurred');
    } finally {
      setLoading(false);
    }
  }, [seriesId, seasonNumber, episodeNumber]);

  useEffect(() => {
    if (seriesId && seasonNumber != null && episodeNumber != null) {
      fetchTvEpisode();
    }
  }, [seriesId, seasonNumber, fetchTvEpisode]);

  return { tvEpisode, loading, error, refetch: fetchTvEpisode };
};