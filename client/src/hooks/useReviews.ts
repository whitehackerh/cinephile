'use client';

import { useState, useEffect } from 'react';
import { Review, Work } from '@/types/review';
import { apiService } from '@/service/api';

export const useReviews = () => {
  const [reviews, setReviews] = useState<Review[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    const fetchReviews = async () => {
      setIsLoading(true);
      setError(null);

      try {
        const res = await apiService.getReviews();
        setReviews(res);
      } catch (err) {
        setError(err instanceof Error ? err : new Error('An error occurred while fetching reviews.'));
      } finally {
        setIsLoading(false);
      }
    };

    fetchReviews();
  }, []);

  const getDisplayInfo = (work: Work) => {
    const title =
      ('composite_title' in work && work.composite_title) ||
      ('title' in work && work.title) ||
      'Untitled';

    const imagePath =
      ('poster_path' in work && work.poster_path) ||
      ('still_path' in work && work.still_path) ||
      null;

    return { title, imagePath };
  };

  return { reviews, isLoading, error, getDisplayInfo };
};