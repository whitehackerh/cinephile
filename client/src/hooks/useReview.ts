'use client';

import { useState, useCallback, useEffect } from 'react';
import { WorkType } from '@/types/review';
import { apiService } from '@/service/api';

interface UseReviewParams {
  workType: WorkType;
  targetPath: string;
}

export function useReview({ workType, targetPath }: UseReviewParams) {
  const [id, setId] = useState<string | null>(null);
  const [rating, setRating] = useState<number>(80);
  const [content, setContent] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [isSubmitting, setIsSubmitting] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const [isSuccess, setIsSuccess] = useState<boolean>(false);

  const fetchReview = useCallback(async () => {
    setIsLoading(true);
    setError(null);

    try {
      const res = await apiService.getReview({
        work_type: workType,
        target_path: targetPath
      });
      if (res) {
        setId(res.id);
        setRating(res.rating);
        setContent(res.content);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'error occurred.');
    } finally {
      setIsLoading(false);
    }
  }, []);

  useEffect(() => {
    fetchReview();
  }, [fetchReview])

  const handleSubmit = useCallback(async (e: React.FormEvent) => {
    e.preventDefault();
    setIsSubmitting(true);
    setError(null);
    setIsSuccess(false);

    try {
      const res = await apiService.postReviews({
        rating,
        content,
        work_type: workType,
        target_path: targetPath
      });
      setId(res.id);
      setIsSuccess(true);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'error occurred.');
    } finally {
      setIsSubmitting(false);
    }
  }, [rating, content, workType, targetPath]);

  return {
    id,
    rating,
    content,
    isLoading,
    isSubmitting,
    error,
    isSuccess,
    handleRatingChange: setRating,
    handleContentChange: setContent,
    handleSubmit,
    refetch: fetchReview
  };
}