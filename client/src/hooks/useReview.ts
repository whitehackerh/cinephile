'use client';

import { useState, useCallback } from 'react';
import { WorkType } from '@/types/review';
import { apiService } from '@/service/api';

interface UseReviewParams {
  workType: WorkType;
  targetPath: string;
}

export function useReview({ workType, targetPath }: UseReviewParams) {
  const [reviewId, setReviewId] = useState<string | null>(null);
  const [rating, setRating] = useState<number>(80);
  const [content, setContent] = useState<string>('');
  const [isSubmitting, setIsSubmitting] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const [isSuccess, setIsSuccess] = useState<boolean>(false);

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
      setIsSuccess(true);
      setReviewId(res.id);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'error occurred.');
    } finally {
      setIsSubmitting(false);
    }
  }, [rating, content, workType, targetPath]);

  return {
    rating,
    content,
    isSubmitting,
    error,
    isSuccess,
    handleRatingChange: setRating,
    handleContentChange: setContent,
    handleSubmit,
  };
}