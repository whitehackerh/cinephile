'use client';

import { use } from 'react';
import Link from 'next/link';
import Image from 'next/image';
import { useTvSeason } from '@/hooks/useTvSeason';
import { getImageUrl } from '@/utils/tmdb';

export default function TvSeasonPage({ 
  params 
}: { 
  params: Promise<{ seriesId: string; seasonNumber: string }> 
}) {
  const resolvedParams = use(params);
  const { tvSeason, loading, error } = useTvSeason(
    resolvedParams.seriesId, 
    resolvedParams.seasonNumber
  );

  if (loading) return <LoadingPlaceholder />;
  if (error) return <ErrorDisplay message={error} />;
  if (!tvSeason) return null;

  const releaseYear = tvSeason.air_date?.split("-")[0] || 'TBA';

  return (
    <main className="min-h-screen bg-slate-950 text-white pb-12">
      {/* Background Hero Header */}
      <div className="relative h-[40vh] w-full bg-slate-900 border-b border-white/10 overflow-hidden">
        {tvSeason.poster_path && (
          <Image
            src={getImageUrl(tvSeason.poster_path, "original")}
            alt=""
            fill
            className="object-cover opacity-15 blur-md"
            priority
          />
        )}
        <div className="absolute inset-0 bg-gradient-to-t from-slate-950 via-slate-950/60 to-transparent" />
        
        {/* Navigation / Breadcrumb */}
        <div className="container mx-auto px-4 pt-6 relative z-10">
          <Link 
            href={`/tv/${resolvedParams.seriesId}`}
            className="inline-flex items-center gap-2 text-sm text-slate-400 hover:text-white transition-colors font-semibold bg-slate-900/80 px-3 py-1.5 rounded-lg border border-white/10"
          >
            ← Back to Series
          </Link>
        </div>
      </div>

      <div className="container mx-auto -mt-32 px-4 relative z-10 flex flex-col md:flex-row gap-10">
        {/* Left Column: Season Poster */}
        <div className="w-full md:w-80 flex-shrink-0">
          <div className="shadow-2xl overflow-hidden rounded-xl bg-slate-800 border border-white/10">
            <Image
              src={getImageUrl(tvSeason.poster_path, "w500")}
              alt={tvSeason.title}
              width={500}
              height={750}
              className="w-full h-auto object-cover"
            />
          </div>
        </div>

        {/* Right Column: Detailed Info & Episode List */}
        <div className="flex-1 space-y-7">
          {/* 1. Title Section */}
          <section className="space-y-2">
            <h1 className="text-4xl md:text-5xl font-black tracking-tight leading-tight">
              {tvSeason.title}
              <span className="ml-3 text-2xl md:text-3xl text-slate-500 font-light">({releaseYear})</span>
            </h1>
            
            <div className="flex flex-wrap items-center gap-x-4 gap-y-2 text-slate-400 text-sm font-medium">
              {tvSeason.air_date && <span>{tvSeason.air_date}</span>}
              <span className="text-slate-300 font-bold">
                {tvSeason.episode_count} {tvSeason.episode_count === 1 ? 'Episode' : 'Episodes'}
              </span>
            </div>
          </section>

          {/* 2. User Score Section */}
          {tvSeason.vote_average !== null && tvSeason.vote_average > 0 && (
            <div className="flex items-center gap-4">
              <div className="relative w-16 h-16 flex items-center justify-center rounded-full border-4 border-emerald-500 bg-slate-900 shadow-lg">
                <span className="text-xl font-bold text-emerald-400">
                  {tvSeason.vote_average.toFixed(1)}
                </span>
              </div>
              <div className="text-xs font-black uppercase tracking-widest text-slate-400 leading-tight">
                User Score
              </div>
            </div>
          )}

          {/* 3. Overview Section */}
          <div className="space-y-3 border-t border-white/10 pt-6">
            <h2 className="text-2xl font-bold tracking-wide">Overview</h2>
            <p className="text-slate-300 text-lg leading-relaxed max-w-4xl font-light">
              {tvSeason.overview || "No overview available for this season."}
            </p>
          </div>

          {/* 4. Episodes Section */}
          {tvSeason.episode_summaries && tvSeason.episode_summaries.length > 0 && (
            <div className="space-y-4 border-t border-white/10 pt-6">
              <h2 className="text-2xl font-bold tracking-wide">Episodes</h2>
              <div className="grid grid-cols-1 gap-4">
                {tvSeason.episode_summaries.map((episode) => (
                  <div 
                    key={episode.id} 
                    className="flex flex-col sm:flex-row gap-4 bg-slate-900/60 border border-white/5 rounded-xl p-4 overflow-hidden hover:border-white/10 transition-colors"
                  >
                    {/* Episode Thumbnail */}
                    <div className="w-full sm:w-48 flex-shrink-0">
                      <Image
                        src={getImageUrl(episode.poster_path, "w500")}
                        alt={episode.title}
                        width={192}
                        height={108}
                        className="rounded-lg object-cover w-full h-auto aspect-video bg-slate-800"
                      />
                    </div>

                    {/* Episode Detail Info */}
                    <div className="flex-1 space-y-2">
                      <div className="flex items-center justify-between flex-wrap gap-2">
                        <h3 className="text-lg font-bold text-white">
                          <span className="text-indigo-400 mr-2">
                            {episode.episode_number}.
                          </span>
                          {episode.title}
                        </h3>
                        {episode.vote_average !== null && episode.vote_average > 0 && (
                          <span className="text-xs bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 px-2 py-0.5 rounded font-bold">
                            ★ {episode.vote_average.toFixed(1)}
                          </span>
                        )}
                      </div>
                      
                      <div className="text-xs text-slate-400 space-x-3">
                        {episode.air_date && <span>{episode.air_date}</span>}
                        {episode.runtime !== null && (
                          <span>{formatRuntime(episode.runtime)}</span>
                        )}
                      </div>

                      <p className="text-slate-300 text-sm line-clamp-3 font-light">
                        {episode.overview || "No episode description available."}
                      </p>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          )}
        </div>
      </div>
    </main>
  );
}

const formatRuntime = (minutes: number) => {
  const hours = Math.floor(minutes / 60);
  const mins = minutes % 60;
  if (hours === 0) return `${mins}m`;
  return `${hours}h ${mins}m`;
};

const LoadingPlaceholder = () => (
  <div className="min-h-screen bg-slate-950 flex flex-col items-center justify-center text-white gap-4">
    <div className="w-12 h-12 border-4 border-indigo-500 border-t-transparent rounded-full animate-spin" />
    <p className="text-slate-400 font-bold tracking-widest uppercase text-sm">Loading Season Details</p>
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