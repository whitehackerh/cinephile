'use client';

import React from "react";

interface ReviewFormProps {
  id: string | null;
  rating: number;
  content: string | null;
  isLoading: boolean;
  isSubmitting: boolean;
  error: string | null;
  isSuccess: boolean;
  submitButtonText: string;
  onRatingChange: (rating: number) => void;
  onContentChange: (content: string) => void;
  onSubmit: (e: React.FormEvent) => void;
  onDelete: (e: React.FormEvent) => void;
}

export function ReviewForm({
  id,
  rating,
  content,
  isSubmitting,
  error,
  isSuccess,
  submitButtonText,
  onRatingChange,
  onContentChange,
  onSubmit,
  onDelete
}: ReviewFormProps) {
  return (
    <form onSubmit={onSubmit} className="bg-slate-900/60 border border-white/10 rounded-xl p-5 space-y-4">
      <div>
        <label htmlFor="rating-slider" className="block text-sm font-semibold text-slate-300 mb-1">
          Score: <span className="text-emerald-400 font-bold">{rating}</span> / 100
        </label>
        <input
          id="rating-slider"
          type="range"
          min="0"
          max="100"
          value={rating}
          onChange={(e) => onRatingChange(Number(e.target.value))}
          className="w-full accent-emerald-500 cursor-pointer"
        />
      </div>

      <div>
        <label htmlFor="review-content" className="block text-sm font-semibold text-slate-300 mb-1">
          Comment
        </label>
        <textarea
          id="review-content"
          rows={3}
          value={content ?? ''}
          onChange={(e) => onContentChange(e.target.value)}
          placeholder="Write your review..."
          className="w-full bg-slate-950/70 border border-white/10 rounded-lg p-2.5 text-white text-sm focus:outline-none focus:border-indigo-500 transition-colors"
        />
      </div>

      {error && <p className="text-xs text-rose-400">⚠️ {error}</p>}
      {isSuccess && <p className="text-xs text-emerald-400">✓ Review submitted successfully</p>}

      <div className={`flex items-center ${id ? 'justify-between' : 'justify-end'}`}>
        {id && (
          <button
            type="button"
            onClick={onDelete}
            disabled={isSubmitting}
            className="px-4 py-2 bg-rose-600/20 hover:bg-rose-600/30 text-rose-400 border border-rose-500/30 disabled:opacity-50 text-xs font-bold rounded-lg transition"
          >
            Delete
          </button>
        )}

        <button
          type="submit"
          disabled={isSubmitting}
          className="px-4 py-2 bg-indigo-600 hover:bg-indigo-500 disabled:bg-slate-800 disabled:text-slate-500 text-xs font-bold text-white rounded-lg transition shadow"
        >
          {submitButtonText}
        </button>
      </div>
    </form>
  );
}