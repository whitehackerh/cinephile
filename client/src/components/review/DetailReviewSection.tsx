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
    rating,
    content,
    isSubmitting,
    error,
    isSuccess,
    handleRatingChange,
    handleContentChange,
    handleSubmit,
  } = useReview({ workType, targetPath });

  return (
    <section className="space-y-3 border-t border-white/10 pt-6 mt-6 max-w-2xl">
      <h2 className="text-xl font-bold tracking-wide">Review</h2>
      <ReviewForm
        rating={rating}
        content={content}
        isSubmitting={isSubmitting}
        error={error}
        isSuccess={isSuccess}
        onRatingChange={handleRatingChange}
        onContentChange={handleContentChange}
        onSubmit={handleSubmit}
      />
    </section>
  );
}