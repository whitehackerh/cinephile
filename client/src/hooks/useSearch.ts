import { useState, useCallback } from 'react';
import { apiService } from '@/service/api';
import { SearchResponse } from '@/types/search';

export const useSearch = () => {
  const [results, setResults] = useState<SearchResponse | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [currentQuery, setCurrentQuery] = useState('');

  const search = useCallback(async (query: string, page: number = 1) => {
    if (!query.trim()) return;

    setLoading(true);
    setError(null);
    setCurrentQuery(query);

    try {
      const data = await apiService.search(query, page);
      setResults(data);
      window.scrollTo({ top: 0, behavior: 'smooth' });
    } catch (err: any) {
      setError(err.message || 'An unexpected error occurred.');
      setResults(null);
    } finally {
      setLoading(false);
    }
  }, []);

  return { 
    results, 
    loading, 
    error, 
    search, 
    currentQuery,
    currentPage: results?.page || 1,
    totalPages: results?.total_pages || 0
  };
};