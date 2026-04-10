'use client';

import { use } from 'react';
import { useMovieDetail } from '@/hooks/useMovieDetail';
import { getImageUrl } from '@/utils/tmdb';
import Image from 'next/image';

export default function MovieDetailPage({ 
  params 
}: { 
  params: Promise<{ id: string }> 
}) {
  const resolvedParams = use(params);
  const { movie, loading, error } = useMovieDetail(resolvedParams.id);

  if (loading) return <LoadingPlaceholder />;
  if (error) return <ErrorDisplay message={error} />;
  if (!movie) return null;

  const formatRuntime = (minutes: number | null) => {
    if (!minutes) return 'N/A';
    const hours = Math.floor(minutes / 60);
    const mins = minutes % 60;
    return `${hours}h ${mins}m`;
  };

  const releaseYear = movie.release_date?.split("-")[0] || 'TBA';

  return (
    <main className="min-h-screen bg-slate-950 text-white pb-12">
      {/* Background Backdrop */}
      <div className="relative h-[60vh] w-full">
        <Image
          src={getImageUrl(movie.backdrop_path, "original")}
          alt=""
          fill
          className="object-cover opacity-20"
          priority
        />
        <div className="absolute inset-0 bg-gradient-to-t from-slate-950 via-slate-950/40 to-transparent" />
      </div>

      <div className="container mx-auto -mt-48 px-4 relative z-10 flex flex-col md:flex-row gap-10">
        {/* Left Column: Poster */}
        <div className="w-full md:w-80 flex-shrink-0">
          <div className="shadow-2xl overflow-hidden rounded-xl bg-slate-800 border border-white/10">
            <Image
              src={getImageUrl(movie.poster_path, "w500")}
              alt={movie.title}
              width={500}
              height={750}
              className="w-full h-auto"
            />
          </div>
        </div>

        {/* Right Column: Detailed Info */}
        <div className="flex-1 space-y-7">
          {/* 1. Title Section */}
          <section className="space-y-2">
            <h1 className="text-5xl font-black tracking-tight leading-tight">
              {movie.title}
              <span className="ml-3 text-3xl text-slate-500 font-light">({releaseYear})</span>
            </h1>
            
            <div className="flex flex-wrap items-center gap-x-4 gap-y-2 text-slate-400 text-sm font-medium">
              {movie.original_title !== movie.title && (
                <span className="px-2 py-0.5 bg-slate-800 rounded text-xs border border-white/5">
                  Original: {movie.original_title}
                </span>
              )}
              <span className="hidden md:inline"></span>
              <span>{movie.release_date}</span>
              <span className="hidden md:inline"></span>
              <span>{formatRuntime(movie.runtime)}</span>
            </div>
          </section>

          {/* 2. User Score Section */}
          <div className="flex items-center gap-4">
            <div className="relative w-16 h-16 flex items-center justify-center rounded-full border-4 border-emerald-500 bg-slate-900 shadow-lg">
              <span className="text-xl font-bold text-emerald-400">
                {movie.vote_average?.toFixed(1) ?? "0.0"}
              </span>
            </div>
            <div className="text-xs font-black uppercase tracking-widest text-slate-400 leading-tight">
              UserScore
            </div>
          </div>

          {/* 3. Tagline Section (Below User Score, Above Overview) */}
          {movie.tagline && (
            <div className="py-2">
              <p className="text-2xl italic text-indigo-300/90 font-serif leading-relaxed">
                "{movie.tagline}"
              </p>
            </div>
          )}

          {/* 4. Overview Section */}
          <div className="space-y-3 border-t border-white/10 pt-6">
            <h2 className="text-2xl font-bold tracking-wide">Overview</h2>
            <p className="text-slate-300 text-lg leading-relaxed max-w-4xl font-light">
              {movie.overview || "No plot summary available for this title."}
            </p>
          </div>

          {/* 5. Genres Section */}
          <div className="flex gap-2 flex-wrap pt-2">
            {movie.genres.map((genre) => (
              <span 
                key={genre.id} 
                className="px-3 py-1 bg-slate-800 text-slate-300 border border-white/10 rounded text-xs font-bold uppercase tracking-wider"
              >
                {genre.name}
              </span>
            ))}
          </div>
        </div>
      </div>
    </main>
  );
}

const LoadingPlaceholder = () => (
  <div className="min-h-screen bg-slate-950 flex flex-col items-center justify-center text-white gap-4">
    <div className="w-12 h-12 border-4 border-indigo-500 border-t-transparent rounded-full animate-spin" />
    <p className="text-slate-400 font-bold tracking-widest uppercase text-sm">Loading Movie Details</p>
  </div>
);

const ErrorDisplay = ({ message }: { message: string }) => (
  <div className="min-h-screen bg-slate-950 flex flex-col items-center justify-center p-6 text-center">
    <div className="bg-red-500/10 border border-red-500/20 p-8 rounded-2xl max-w-md">
      <div className="text-red-500 text-4xl mb-4">⚠️</div>
      <h2 className="text-white text-xl font-bold mb-2">Request Failed</h2>
      <p className="text-slate-400 text-sm mb-6">{message}</p>
      <button 
        onClick={() => window.location.reload()}
        className="px-6 py-2 bg-slate-800 hover:bg-slate-700 rounded-lg text-sm font-bold transition-colors"
      >
        Retry
      </button>
    </div>
  </div>
);