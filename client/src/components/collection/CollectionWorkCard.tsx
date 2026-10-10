
import Link from 'next/link';
import { Collection } from '@/types/collection';
import { getImageUrl } from '@/utils/tmdb';

type CollectionWork = Collection['works'][number];

type Props = {
  collectionWork: CollectionWork;
  deleting: boolean;
  onDelete: (collectionWorkId: string, workTitle: string) => void;
};

export default function CollectionWorkCard({ collectionWork, deleting, onDelete }: Props) {
  const work = collectionWork.work;
  const workTitle =
    (('composite_title' in work && work.composite_title) ||
    ('title' in work && work.title) ||
    'Untitled');
  const imagePath =
    (('poster_path' in work && work.poster_path) ||
    ('still_path' in work && work.still_path) ||
    null);

  return (
    <article className="flex min-w-0 gap-4 border border-white/10 bg-white/[0.02] p-4 transition-colors hover:border-white/20">
      <Link href={collectionWork.target_path} className="flex min-w-0 flex-1 gap-4">
        <div className="h-28 w-20 shrink-0 overflow-hidden bg-zinc-900 sm:h-32 sm:w-22">
          {imagePath ? (
            <img src={getImageUrl(imagePath)} alt={workTitle} className="h-full w-full object-cover" />
          ) : (
            <div className="flex h-full items-center justify-center text-xs text-gray-600">No image</div>
          )}
        </div>
        <div className="min-w-0 py-1">
          <h3 className="line-clamp-3 font-serif text-lg leading-snug text-gray-100">{workTitle}</h3>
          <p className="mt-2 text-[10px] uppercase tracking-[0.2em] text-gray-500">{collectionWork.work_type}</p>
          {('release_date' in work && work.release_date) && (
            <p className="mt-2 text-xs text-gray-500">{work.release_date}</p>
          )}
          {('first_air_date' in work && work.first_air_date) && (
            <p className="mt-2 text-xs text-gray-500">{work.first_air_date}</p>
          )}
          {('air_date' in work && work.air_date) && (
            <p className="mt-2 text-xs text-gray-500">{work.air_date}</p>
          )}
        </div>
      </Link>
      <button
        type="button"
        onClick={() => onDelete(collectionWork.id, workTitle)}
        disabled={deleting}
        aria-label={`Remove ${workTitle} from collection`}
        title="Remove from collection"
        className="inline-flex h-9 w-9 shrink-0 items-center justify-center self-start border border-white/10 text-gray-500 transition-colors hover:border-red-500/50 hover:text-red-400 disabled:opacity-40"
      >
        <svg aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
          <path d="M3 6h18" />
          <path d="M8 6V4h8v2" />
          <path d="m19 6-1 14H6L5 6" />
          <path d="M10 11v5" />
          <path d="M14 11v5" />
        </svg>
      </button>
    </article>
  );
}
