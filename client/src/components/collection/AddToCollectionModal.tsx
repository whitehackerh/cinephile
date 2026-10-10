"use client";

import React from "react";
import { WorkType } from "@/types/work";
import { useAddToCollection } from "@/hooks/useAddToCollection";
import { getImageUrl } from '@/utils/tmdb';
import Image from 'next/image';

interface AddToCollectionModalProps {
  isOpen: boolean;
  onClose: () => void;
  targetPath: string;
  workType: WorkType;
}

export const AddToCollectionModal = ({
  isOpen,
  onClose,
  targetPath,
  workType,
}: AddToCollectionModalProps) => {
  const { state, actions } = useAddToCollection({
    isOpen,
    onClose,
    targetPath,
    workType,
  });

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4">
      <div className="w-full max-w-md rounded-2xl bg-gray-900 border border-gray-800 p-6 shadow-xl text-white">
        
        {/* Header */}
        <div className="flex items-center justify-between pb-4 mb-4 border-b border-gray-800">
          <h2 className="text-xl font-bold tracking-wide">
            {state.isCreating ? "Create Collection" : "Add to Collection"}
          </h2>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-white transition-colors text-sm"
          >
            ✕
          </button>
        </div>

        {/* View A: Collection List */}
        {!state.isCreating ? (
          <div className="space-y-4">
            {state.loading ? (
              <div className="py-8 text-center text-gray-400 text-sm">
                Loading collections...
              </div>
            ) : state.collections.length === 0 ? (
              <div className="py-8 text-center text-gray-400 text-sm">
                No collections found. Create one below!
              </div>
            ) : (
              <div className="max-h-60 overflow-y-auto space-y-2 pr-1">
                {state.collections.map((collection) => {
                  const work = collection.work_summaries.find(
                    (work) => work.target_path === targetPath
                  );
                  const isAdded = work !== undefined;
                  const itemCount = collection.work_summaries.length;

                  return (
                    <div
                      key={collection.id}
                      className="flex items-center justify-between p-3 rounded-xl bg-gray-800/50 hover:bg-gray-800 border border-gray-700/50 transition-colors"
                    >
                      {/* Left side: Cover Image + Info */}
                      <div className="flex items-center space-x-3 min-w-0 pr-3">
                        {/* Cover Image or Fallback */}
                        <div className="relative w-12 h-12 rounded-lg overflow-hidden bg-gray-800 shrink-0 border border-gray-700/60">
                          {collection.cover_image_path ? (
                            <Image
                              src={getImageUrl(collection.cover_image_path, "w500")}
                              alt={collection.title}
                              width={500}
                              height={750}
                              className="w-full h-auto object-cover"
                            />
                          ) : (
                            <div className="w-full h-full flex items-center justify-center bg-gradient-to-br from-gray-800 to-gray-700 text-gray-400">
                              <svg
                                className="w-5 h-5 opacity-60"
                                fill="none"
                                stroke="currentColor"
                                viewBox="0 0 24 24"
                              >
                                <path
                                  strokeLinecap="round"
                                  strokeLinejoin="round"
                                  strokeWidth={1.5}
                                  d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"
                                />
                              </svg>
                            </div>
                          )}
                        </div>

                        {/* Title & Item count */}
                        <div className="min-w-0">
                          <p className="font-semibold text-sm truncate text-gray-100">
                            {collection.title}
                          </p>
                          <p className="text-xs text-gray-400 mt-0.5">
                            {itemCount} {itemCount === 1 ? "item" : "items"}
                          </p>
                        </div>
                      </div>

                      {/* Bookmark Button */}
                      <button
                        type="button"
                        disabled={state.addingId === collection.id}
                        onClick={() =>
                          isAdded
                            ? actions.handleRemoveWork(collection.id, work.id)
                            : actions.handleAddWork(collection.id)
                        }
                        aria-label={
                          isAdded
                            ? `Remove from ${collection.title}`
                            : `Add to ${collection.title}`
                        }
                        aria-pressed={isAdded}
                        title={
                          isAdded
                            ? "Remove from collection"
                            : "Add to collection"
                        }
                        className={`shrink-0 p-2 rounded-lg transition-colors disabled:opacity-50 disabled:cursor-wait ${
                          isAdded
                            ? "text-blue-500 hover:text-blue-400"
                            : "text-gray-400 hover:text-white"
                        }`}
                      >
                        <svg
                          className="w-5 h-5"
                          viewBox="0 0 24 24"
                          fill={isAdded ? "currentColor" : "none"}
                          stroke="currentColor"
                          strokeWidth={1.8}
                          strokeLinecap="round"
                          strokeLinejoin="round"
                        >
                          <path d="M6 4.75A1.75 1.75 0 0 1 7.75 3h8.5A1.75 1.75 0 0 1 18 4.75V21l-6-4-6 4V4.75Z" />
                        </svg>
                      </button>
                    </div>
                  );
                })}
              </div>
            )}

            {/* List Footer Actions */}
            <div className="pt-4 border-t border-gray-800 flex flex-col space-y-2">
              <button
                type="button"
                onClick={() => actions.setIsCreating(true)}
                className="w-full py-2.5 rounded-xl bg-gray-800 hover:bg-gray-700 text-sm font-semibold text-blue-400 hover:text-blue-300 transition-colors border border-gray-700"
              >
                + Create New Collection
              </button>
            </div>
          </div>
        ) : (
          /* View B: Create Collection Form */
          <form onSubmit={actions.handleCreateCollection} className="space-y-4">
            <div>
              <label className="block text-xs font-semibold text-gray-300 mb-1">
                Title <span className="text-red-400">*</span>
              </label>
              <input
                type="text"
                value={state.newTitle}
                onChange={(e) => actions.setNewTitle(e.target.value)}
                placeholder="Enter collection title..."
                className="w-full px-3 py-2 rounded-xl bg-gray-800 border border-gray-700 text-sm text-white placeholder-gray-500 focus:outline-none focus:border-blue-500 transition-colors"
              />
              {state.titleError && (
                <p className="text-xs text-red-400 mt-1">{state.titleError}</p>
              )}
            </div>

            <div>
              <label className="block text-xs font-semibold text-gray-300 mb-1">
                Description
              </label>
              <textarea
                value={state.newDescription}
                onChange={(e) => actions.setNewDescription(e.target.value)}
                placeholder="Add an optional description..."
                rows={3}
                className="w-full px-3 py-2 rounded-xl bg-gray-800 border border-gray-700 text-sm text-white placeholder-gray-500 focus:outline-none focus:border-blue-500 transition-colors resize-none"
              />
            </div>

            {/* Form Footer Actions */}
            <div className="pt-2 flex items-center space-x-3">
              <button
                type="button"
                onClick={() => actions.setIsCreating(false)}
                className="flex-1 py-2.5 rounded-xl bg-gray-800 hover:bg-gray-700 text-sm font-semibold text-gray-300 transition-colors"
              >
                Back
              </button>
              <button
                type="submit"
                disabled={state.submitting}
                className="flex-1 py-2.5 rounded-xl bg-blue-600 hover:bg-blue-500 disabled:opacity-50 text-sm font-semibold text-white transition-colors shadow-sm"
              >
                {state.submitting ? "Creating..." : "Create & Add"}
              </button>
            </div>
          </form>
        )}

      </div>
    </div>
  );
};