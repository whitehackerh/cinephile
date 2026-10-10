'use client';

import { useCallback, useEffect, useState } from 'react';
import { CollectionSummary } from '@/types/collection';
import { apiService } from '@/service/api';

export function useCollections() {
  const [collections, setCollections] = useState<CollectionSummary[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchCollections = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    try {
      setCollections(await apiService.getCollections());
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load collections.');
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchCollections();
  }, [fetchCollections]);

  return {
    collections,
    isLoading,
    error,
    fetchCollections
  };
}
