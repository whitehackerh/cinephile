'use client';

import Link from 'next/link';
import { useReviews } from '@/hooks/useReviews';
import { getImageUrl } from '@/utils/tmdb';

export default function ReviewList() {
  const { reviews, isLoading, error, getDisplayInfo } = useReviews();
  if (isLoading) {
    return <div className="text-center py-8 text-gray-500">Loading...</div>;
  }

  if (reviews.length === 0) {
    return <div className="text-center py-8 text-gray-500">No reviews found.</div>;
  }

  return (
    <div className="space-y-4 max-w-2xl mx-auto p-4">
      <h2 className="text-xl font-bold mb-4">Reviews</h2>
      {reviews.map((review) => {
        const { title, imagePath } = getDisplayInfo(review.work);
        return (
          <article
            key={review.id}
            className="flex gap-4 p-4 border rounded-lg shadow-sm bg-white hover:shadow-md transition-shadow"
          >
            <Link href={review.target_path} className="flex gap-4 group">
              {/* Poster Image */}
              <div className="w-20 h-30 flex-shrink-0 bg-gray-200 rounded overflow-hidden">
                {imagePath ? (
                  <img
                    src={getImageUrl(imagePath)}
                    alt={title}
                    className="w-full h-full object-cover"
                  />
                ) : (
                  <div className="w-full h-full flex items-center justify-center text-xs text-gray-400">
                    No Image
                  </div>
                )}
              </div>

              {/* Review Details */}
              <div className="flex-1 flex flex-col justify-between">
                <div>
                  {/* Unified Display Title */}
                  <h3 className="font-bold text-lg text-gray-900 leading-snug">
                    {title}
                  </h3>

                  {/* Rating & Date */}
                  <div className="flex items-center gap-2 mt-1">
                    <span className="text-yellow-500 font-semibold">
                      ★ {review.rating}
                    </span>
                    <span className="text-xs text-gray-400">
                      {new Date(review.updated_at).toLocaleDateString('en-US', {
                        year: 'numeric',
                        month: 'short',
                        day: 'numeric',
                      })}
                    </span>
                  </div>

                  <p className="mt-2 text-sm text-gray-700 line-clamp-3">
                    {review.content}
                  </p>
                </div>
              </div>
            </Link>
          </article>
        );
      })}
    </div>
  );
};