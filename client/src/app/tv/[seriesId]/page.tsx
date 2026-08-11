'use client';

import { use } from 'react';
import { useTvSeries } from '@/hooks/useTvSeries';
import { getImageUrl } from '@/utils/tmdb';
import Image from 'next/image';

export default function TvSeriesPage({ 
  params 
}: { 
  params: Promise<{ seriesId: string }> 
}) {
  const resolvedParams = use(params);
  const { tvSeries, loading, error } = useTvSeries(resolvedParams.seriesId);

  if (loading) return <LoadingPlaceholder />;
  if (error) return <ErrorDisplay message={error} />;
  if (!tvSeries) return null;

  const releaseYear = tvSeries.first_air_date?.split("-")[0] || 'TBA';

  return (
    <main className="min-h-screen bg-slate-950 text-white pb-12">
      {/* Background Backdrop */}
      <div className="relative h-[60vh] w-full">
        <Image
          src={getImageUrl(tvSeries.backdrop_path, "original")}
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
              src={getImageUrl(tvSeries.poster_path, "w500")}
              alt={tvSeries.title}
              width={500}
              height={750}
              className="w-full h-auto object-cover"
            />
          </div>
        </div>

        {/* Right Column: Detailed Info */}
        <div className="flex-1 space-y-7">
          {/* 1. Title Section */}
          <section className="space-y-2">
            <h1 className="text-5xl font-black tracking-tight leading-tight">
              {tvSeries.title}
              <span className="ml-3 text-3xl text-slate-500 font-light">({releaseYear})</span>
            </h1>
            
            <div className="flex flex-wrap items-center gap-x-4 gap-y-2 text-slate-400 text-sm font-medium">
              {tvSeries.original_title !== tvSeries.title && (
                <span className="px-2 py-0.5 bg-slate-800 rounded text-xs border border-white/5">
                  Original: {tvSeries.original_title}
                </span>
              )}
              {tvSeries.first_air_date && <span>{tvSeries.first_air_date}</span>}
              
              {/* Season & Episode Count */}
              {tvSeries.number_of_seasons !== null && (
                <span className="text-slate-300 font-bold">
                  {tvSeries.number_of_seasons} {tvSeries.number_of_seasons === 1 ? 'Season' : 'Seasons'}
                </span>
              )}
              {tvSeries.number_of_episodes !== null && (
                <span>
                  ({tvSeries.number_of_episodes} {tvSeries.number_of_episodes === 1 ? 'Episode' : 'Episodes'})
                </span>
              )}
            </div>
          </section>

          {/* 2. User Score Section */}
          <div className="flex items-center gap-4">
            <div className="relative w-16 h-16 flex items-center justify-center rounded-full border-4 border-emerald-500 bg-slate-900 shadow-lg">
              <span className="text-xl font-bold text-emerald-400">
                {tvSeries.vote_average?.toFixed(1) ?? "0.0"}
              </span>
            </div>
            <div className="text-xs font-black uppercase tracking-widest text-slate-400 leading-tight">
              User Score
            </div>
          </div>

          {/* 3. Tagline Section */}
          {tvSeries.tagline && (
            <div className="py-2">
              <p className="text-2xl italic text-indigo-300/90 font-serif leading-relaxed">
                "{tvSeries.tagline}"
              </p>
            </div>
          )}

          {/* 4. Overview Section */}
          <div className="space-y-3 border-t border-white/10 pt-6">
            <h2 className="text-2xl font-bold tracking-wide">Overview</h2>
            <p className="text-slate-300 text-lg leading-relaxed max-w-4xl font-light">
              {tvSeries.overview || "No plot summary available for this title."}
            </p>
          </div>

          {/* 5. Genres Section */}
          <div className="flex gap-2 flex-wrap pt-2">
            {tvSeries.genres.map((genre) => (
              <span 
                key={genre.id} 
                className="px-3 py-1 bg-slate-800 text-slate-300 border border-white/10 rounded text-xs font-bold uppercase tracking-wider"
              >
                {genre.name}
              </span>
            ))}
          </div>

          {/* 6. Seasons Section */}
          {tvSeries.season_summaries && tvSeries.season_summaries.length > 0 && (
            <div className="space-y-4 border-t border-white/10 pt-6">
              <h2 className="text-2xl font-bold tracking-wide">Seasons</h2>
              <div className="grid grid-cols-1 gap-4">
                {tvSeries.season_summaries.map((season) => (
                  <div 
                    key={season.id} 
                    className="flex flex-col sm:flex-row gap-4 bg-slate-900/60 border border-white/5 rounded-xl p-4 overflow-hidden"
                  >
                    <div className="w-24 flex-shrink-0 mx-auto sm:mx-0">
                      <Image
                        src={getImageUrl(season.poster_path, "w500")}
                        alt={season.title}
                        width={96}
                        height={144}
                        className="rounded-lg object-cover w-full h-auto bg-slate-800"
                      />
                    </div>
                    <div className="flex-1 space-y-2">
                      <div className="flex items-center justify-between flex-wrap gap-2">
                        <h3 className="text-lg font-bold text-white">
                          {season.title}
                        </h3>
                        {season.vote_average !== null && season.vote_average > 0 && (
                          <span className="text-xs bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 px-2 py-0.5 rounded font-bold">
                            ★ {season.vote_average.toFixed(1)}
                          </span>
                        )}
                      </div>
                      
                      <div className="text-xs text-slate-400 space-x-3">
                        {season.air_date && <span>{season.air_date.split('-')[0]}</span>}
                        <span>{season.episode_count} {season.episode_count === 1 ? 'Episode' : 'Episodes'}</span>
                      </div>

                      <p className="text-slate-300 text-sm line-clamp-3 font-light">
                        {season.overview || "No season overview available."}
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

const LoadingPlaceholder = () => (
  <div className="min-h-screen bg-slate-950 flex flex-col items-center justify-center text-white gap-4">
    <div className="w-12 h-12 border-4 border-indigo-500 border-t-transparent rounded-full animate-spin" />
    <p className="text-slate-400 font-bold tracking-widest uppercase text-sm">Loading TV Series Details</p>
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