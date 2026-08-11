import { useState, useEffect, useCallback } from 'react';
import { apiService } from '@/service/api';
import { TvSeries } from '@/types/tvSeries';

export const useTvSeries = (id: string) => {
  const [tvSeries, setTvSeries] = useState<TvSeries | null>(null);
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);

  const fetchTvSeries = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const data = await apiService.getTvSeries(id);
      setTvSeries(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'An unexpected error occurred');
    } finally {
      setLoading(false);
    }
  }, [id]);

  useEffect(() => {
    if (id) {
      fetchTvSeries();
    }
  }, [id, fetchTvSeries]);

  return { tvSeries, loading, error, refetch: fetchTvSeries };
};
