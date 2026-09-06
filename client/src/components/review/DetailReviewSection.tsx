'use client';

import { useReview } from '@/hooks/useReview';
import { ReviewForm } from './ReviewForm';
import { WorkType } from '@/types/review';

interface DetailReviewSectionProps {
  workType: WorkType;
  targetPath: string;
}

export function DetailReviewSection({ workType, targetPath }: DetailReviewSectionProps) {
  const {
    id,
    rating,
    content,
    isLoading,
    isSubmitting,
    error,
    isSuccess,
    handleRatingChange,
    handleContentChange,
    handleSubmit,
    refetch
  } = useReview({ workType, targetPath });

  return (
    <section className="space-y-3 border-t border-white/10 pt-6 mt-6 max-w-2xl">
      <h2 className="text-xl font-bold tracking-wide">Review</h2>
      <ReviewForm
        id={id}
        rating={rating}
        content={content}
        isLoading={isLoading}
        isSubmitting={isSubmitting}
        error={error}
        isSuccess={isSuccess}
        onRatingChange={handleRatingChange}
        onContentChange={handleContentChange}
        onSubmit={handleSubmit}
        refetch={refetch}
      />
    </section>
  );
}