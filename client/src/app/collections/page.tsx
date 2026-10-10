
'use client';

import Link from 'next/link';
import CollectionCard from '@/components/collection/CollectionCard';
import { useCollections } from '@/hooks/useCollections';

export default function CollectionsPage() {
  const { collections, isLoading, error, fetchCollections } = useCollections();

  return (
    <div className="mx-auto min-h-[70vh] max-w-7xl px-4 py-8 text-white sm:px-6">
      <header className="mb-10 border-b border-white/10 pb-6">
        <h1 className="font-serif text-4xl tracking-wide sm:text-5xl">Collections</h1>
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
          <p className="text-sm text-gray-400">No collections yet. Find a movie or TV show to start your first collection.</p>
          <Link href="/search" className="mt-4 inline-block border border-white/20 px-4 py-2 text-xs uppercase tracking-widest hover:border-gold hover:text-gold">
            Search movies &amp; TV
          </Link>
        </div>
      ) : (
        <div className="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-3">
          {collections.map((collection) => (
            <CollectionCard key={collection.id} collection={collection} />
          ))}
        </div>
      )}
    </div>
  );
}
