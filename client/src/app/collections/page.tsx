'use client';

import Link from 'next/link';
import { useCallback, useEffect, useState } from 'react';
import { CollectionSummary } from '@/types/collection';
import { apiService } from '@/service/api';
import { getImageUrl } from '@/utils/tmdb';

export default function CollectionsPage() {
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

  return (
    <div className="mx-auto min-h-[70vh] max-w-7xl px-4 py-8 text-white sm:px-6">
      <header className="mb-10 border-b border-white/10 pb-6">
        <p className="mb-3 text-xs uppercase tracking-[0.35em] text-gold">Your Archive</p>
        <h1 className="font-serif text-4xl tracking-wide sm:text-5xl">Collections</h1>
        <p className="mt-3 max-w-xl text-sm leading-6 text-gray-400">
          A personal archive for the movies and shows you want to keep together.
        </p>
      </header>

      {isLoading ? (
        <p className="py-16 text-center text-sm text-gray-400">Loading collections...</p>
      ) : error ? (
        <div className="py-12 text-center">
          <p className="text-sm text-red-400">{error}</p>
          <button onClick={fetchCollections} className="mt-4 border border-white/20 px-4 py-2 text-xs uppercase tracking-widest hover:border-gold hover:text-gold">
            Retry
          </button>
        </div>
      ) : collections.length === 0 ? (
        <div className="border border-dashed border-white/15 px-6 py-16 text-center">
          <p className="font-serif text-2xl text-gray-200">Your archive is empty</p>
          <p className="mt-3 text-sm text-gray-500">Add a movie or show to create your first collection.</p>
          <Link href="/search" className="mt-6 inline-block border border-gold px-5 py-3 text-xs font-bold uppercase tracking-[0.2em] text-gold transition-colors hover:bg-gold hover:text-black">
            Explore titles
          </Link>
        </div>
      ) : (
        <div className="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-3">
          {collections.map((collection) => (
            <Link
              key={collection.id}
              href={`/collections/${collection.id}`}
              className="group overflow-hidden border border-white/10 bg-white/[0.02] transition-all duration-300 hover:-translate-y-1 hover:border-gold/50 hover:bg-white/[0.04]"
            >
              <div className="relative aspect-[16/9] overflow-hidden bg-gradient-to-br from-zinc-900 to-black">
                {collection.cover_image_path ? (
                  <img
                    src={getImageUrl(collection.cover_image_path, 'w500')}
                    alt={collection.title}
                    className="h-full w-full object-cover opacity-75 transition duration-500 group-hover:scale-105 group-hover:opacity-100"
                  />
                ) : (
                  <div className="flex h-full items-center justify-center">
                    <span className="font-serif text-5xl text-white/10">C</span>
                  </div>
                )}
                <div className="absolute inset-0 bg-gradient-to-t from-black via-black/10 to-transparent" />
                <div className="absolute bottom-4 left-5 right-5">
                  <p className="text-[10px] uppercase tracking-[0.25em] text-gold">Collection</p>
                  <h2 className="mt-1 truncate font-serif text-2xl text-white">{collection.title}</h2>
                </div>
              </div>
              <div className="flex min-h-24 items-start justify-between gap-4 p-5">
                <p className="line-clamp-2 text-sm leading-6 text-gray-400">
                  {collection.description || 'No description'}
                </p>
                <span className="shrink-0 whitespace-nowrap text-xs text-gray-500">
                  {collection.work_target_paths.length} {collection.work_target_paths.length === 1 ? 'work' : 'works'}
                </span>
              </div>
              <div className="flex items-center justify-between border-t border-white/5 px-5 py-3 text-[10px] uppercase tracking-[0.2em] text-gray-500 transition-colors group-hover:text-gold">
                <span>Open collection</span>
                <span aria-hidden="true">↗</span>
              </div>
            </Link>
          ))}
        </div>
      )}
    </div>
  );
}
