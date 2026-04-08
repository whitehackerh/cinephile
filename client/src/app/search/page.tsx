'use client';

import { useState } from 'react';
import { useSearch } from '@/hooks/useSearch';
import { SearchCard } from '@/components/SearchCard';

export default function SearchPage() {
  const [query, setQuery] = useState('');
  const { results, loading, error, search, currentQuery } = useSearch();

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    search(query);
  };

  return (
    <div className="min-h-screen bg-gray-900 text-gray-100 p-6">
      <div className="max-w-7xl mx-auto">
        <header className="mb-10 text-center">
          <h1 className="text-4xl font-extrabold tracking-tight mb-2 text-white">
            Explore Movies & TV
          </h1>
          <p className="text-gray-400">Find and record your favorite titles.</p>
        </header>

        {/* Search Form */}
        <div className="max-w-2xl mx-auto mb-12">
          <form onSubmit={handleSubmit} className="relative group">
            <input
              type="text"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder="Enter movie or series title..."
              className="w-full bg-gray-800 border border-gray-700 text-white rounded-full py-4 px-6 pl-12 focus:outline-none focus:ring-2 focus:ring-indigo-500 transition-all shadow-xl"
            />
            <div className="absolute left-4 top-4 text-gray-500">
              <SearchIcon />
            </div>
            <button 
              type="submit"
              disabled={loading}
              className="absolute right-2 top-2 bg-indigo-600 hover:bg-indigo-500 text-white px-6 py-2 rounded-full font-medium transition-colors disabled:bg-gray-600"
            >
              {loading ? 'Searching...' : 'Search'}
            </button>
          </form>
          {error && <p className="mt-4 text-red-400 text-center text-sm">{error}</p>}
        </div>

        {/* Results Section */}
        {results && (
        <div className="space-y-8">
            <div className="flex justify-between items-end border-b border-gray-800 pb-4">
            <h2 className="text-xl font-semibold">Results for "{currentQuery}"</h2>
            <span className="text-gray-500 text-sm">{results.total_results} items found</span>
            </div>
            
            <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 gap-6">
            {results.works.map((work) => (
                <SearchCard key={`${work.media_type}-${work.id}`} work={work} />
            ))}
            </div>

            {/* Pagination UI */}
            {results.total_pages > 1 && (
            <div className="flex justify-center items-center gap-6 py-12">
                <button
                onClick={() => search(currentQuery, results.page - 1)}
                disabled={loading || results.page <= 1}
                className="px-6 py-2 rounded-lg bg-gray-800 hover:bg-gray-700 disabled:opacity-30 disabled:cursor-not-allowed transition-colors text-sm font-medium"
                >
                Previous
                </button>
                
                <span className="text-gray-400 text-sm font-mono">
                Page <span className="text-white">{results.page}</span> / {results.total_pages}
                </span>

                <button
                onClick={() => search(currentQuery, results.page + 1)}
                disabled={loading || results.page >= results.total_pages}
                className="px-6 py-2 rounded-lg bg-gray-800 hover:bg-gray-700 disabled:opacity-30 disabled:cursor-not-allowed transition-colors text-sm font-medium"
                >
                Next
                </button>
            </div>
            )}
        </div>
        )}
      </div>
    </div>
  );

}

const SearchIcon = () => (
  <svg className="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
  </svg>
);