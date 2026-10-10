
import Link from 'next/link';
import { CollectionSummary } from '@/types/collection';
import { getImageUrl } from '@/utils/tmdb';

type Props = {
  collection: CollectionSummary;
};

export default function CollectionCard({ collection }: Props) {
  return (
    <Link
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
            <span className="font-serif text-5xl text-white/10"></span>
          </div>
        )}
        <div className="absolute inset-0 bg-gradient-to-t from-black via-black/10 to-transparent" />
        <div className="absolute bottom-4 left-5 right-5">
          <h2 className="mt-1 truncate font-serif text-2xl text-white">{collection.title}</h2>
        </div>
      </div>
      <div className="flex min-h-24 items-start justify-between gap-4 p-5">
        <p className="line-clamp-2 text-sm leading-6 text-gray-400">
          {collection.description || 'No description'}
        </p>
        <span className="shrink-0 whitespace-nowrap text-xs text-gray-500">
          {collection.work_summaries.length} {collection.work_summaries.length === 1 ? 'work' : 'works'}
        </span>
      </div>
    </Link>
  );
}
