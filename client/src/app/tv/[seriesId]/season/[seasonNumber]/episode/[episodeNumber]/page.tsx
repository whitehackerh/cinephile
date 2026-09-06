'use client';

import { use } from 'react';
import Link from 'next/link';
import Image from 'next/image';
import { useTvEpisode } from '@/hooks/useTvEpisode';
import { getImageUrl } from '@/utils/tmdb';
import { DetailReviewSection } from '@/components/review/DetailReviewSection';

export default function TvEpisodePage({ 
  params 
}: { 
  params: Promise<{ seriesId: string; seasonNumber: string; episodeNumber: string }> 
}) {
  const resolvedParams = use(params);
  const { tvEpisode, loading, error } = useTvEpisode(
    resolvedParams.seriesId, 
    resolvedParams.seasonNumber,
    resolvedParams.episodeNumber
  );

  if (loading) return <LoadingPlaceholder />;
  if (error) return <ErrorDisplay message={error} />;
  if (!tvEpisode) return null;

  return (
    <main className="min-h-screen bg-slate-950 text-white pb-16">
      {/* 1. Background Still Image Banner */}
      <div className="relative h-[50vh] w-full bg-slate-900 border-b border-white/10 overflow-hidden">
        {tvEpisode.still_path ? (
          <Image
            src={getImageUrl(tvEpisode.still_path, "original")}
            alt=""
            fill
            className="object-cover opacity-25 blur-sm"
            priority
          />
        ) : (
          <div className="absolute inset-0 bg-slate-900" />
        )}
        <div className="absolute inset-0 bg-gradient-to-t from-slate-950 via-slate-950/60 to-transparent" />
        
        {/* Navigation / Breadcrumbs */}
        <div className="container mx-auto px-4 pt-6 relative z-10">
          <Link 
            href={`/tv/${resolvedParams.seriesId}/season/${resolvedParams.seasonNumber}`}
            className="inline-flex items-center gap-2 text-sm text-slate-300 hover:text-white transition-colors font-semibold bg-slate-900/80 hover:bg-slate-800 px-3 py-1.5 rounded-lg border border-white/10"
          >
            ← Back to Season {resolvedParams.seasonNumber}
          </Link>
        </div>
      </div>

      {/* 2. Main Detail Content */}
      <div className="container mx-auto -mt-40 px-4 relative z-10 flex flex-col md:flex-row gap-10">
        {/* Left Column: Episode Still Image Card */}
        <div className="w-full md:w-96 flex-shrink-0">
          <div className="shadow-2xl overflow-hidden rounded-xl bg-slate-800 border border-white/10 aspect-video relative">
            {tvEpisode.still_path ? (
              <Image
                src={getImageUrl(tvEpisode.still_path, "w500")}
                alt=""
                fill
                className="object-cover"
              />
            ) : (
              <div className="w-full h-full flex flex-col items-center justify-center text-slate-500 bg-slate-900">
                <span className="text-4xl mb-2">🎬</span>
                <span className="text-xs font-bold uppercase tracking-widest">No Image Available</span>
              </div>
            )}
          </div>
        </div>

        {/* Right Column: Information & Metadata */}
        <div className="flex-1 space-y-7">
          {/* Header & Badges */}
          <section className="space-y-3">
            <div className="flex flex-wrap items-center gap-2">
              <span className="px-2.5 py-1 bg-indigo-500/20 text-indigo-400 border border-indigo-500/30 rounded text-xs font-black uppercase tracking-wider">
                Season {tvEpisode.season_number} • Episode {tvEpisode.episode_number}
              </span>
            </div>

            <h1 className="text-3xl md:text-5xl font-black tracking-tight leading-tight">
              {tvEpisode.title}
            </h1>
            
            {/* Meta Items Row */}
            <div className="flex flex-wrap items-center gap-x-4 gap-y-2 text-slate-400 text-sm font-medium">
              {tvEpisode.air_date && (
                <span>{tvEpisode.air_date}</span>
              )}
              {tvEpisode.runtime !== null && (
                <>
                  <span>•</span>
                  <span>{formatRuntime(tvEpisode.runtime)}</span>
                </>
              )}
            </div>
          </section>

          {/* User Score Section */}
          {tvEpisode.vote_average !== null && tvEpisode.vote_average > 0 && (
            <div className="flex items-center gap-4">
              <div className="relative w-16 h-16 flex items-center justify-center rounded-full border-4 border-emerald-500 bg-slate-900 shadow-lg">
                <span className="text-xl font-bold text-emerald-400">
                  {tvEpisode.vote_average.toFixed(1)}
                </span>
              </div>
              <div className="text-xs font-black uppercase tracking-widest text-slate-400 leading-tight">
                TMDB User Score
              </div>
            </div>
          )}

          {/* Overview Section */}
          <div className="space-y-3 border-t border-white/10 pt-6">
            <h2 className="text-2xl font-bold tracking-wide">Overview</h2>
            <p className="text-slate-300 text-lg leading-relaxed max-w-4xl font-light">
              {tvEpisode.overview || "No episode overview available for this title."}
            </p>
          </div>

          <DetailReviewSection 
            workType="episode" 
            targetPath={`/tv/${resolvedParams.seriesId}/season/${resolvedParams.seasonNumber}/episode/${resolvedParams.episodeNumber}`}
          />
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
    <p className="text-slate-400 font-bold tracking-widest uppercase text-sm">Loading Episode Details</p>
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