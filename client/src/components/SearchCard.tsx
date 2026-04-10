import Link from 'next/link';
import Image from 'next/image';
import { Work } from '@/types/search';
import { getImageUrl } from '@/utils/tmdb';

export const SearchCard = ({ work }: { work: Work }) => {
  const isMovie = work.media_type === 'movie';
  const title = isMovie ? work.title : work.name;
  const date = isMovie ? work.release_date : work.first_air_date;
  const posterUrl = getImageUrl(work.poster_path, 'w500');
  const detailPath = work.media_type === 'movie' 
    ? `/movie/${work.id}` 
    : `/tv/${work.id}`;

  return (
    <Link href={detailPath} className="group relative border border-gray-700 rounded-lg overflow-hidden shadow-sm hover:shadow-md transition-shadow bg-gray-800">
      <div className="aspect-[2/3] relative bg-gray-700">
        <Image
          src={posterUrl}
          alt={title}
          fill
          className="object-cover transition-transform duration-300 group-hover:scale-105"
          sizes="(max-width: 768px) 50vw, 33vw"
        />
        <div className="absolute top-2 right-2 px-2 py-1 rounded bg-black/60 text-white text-[10px] font-bold uppercase tracking-wider">
          {work.media_type}
        </div>
      </div>
      <div className="p-4 text-gray-100">
        <h3 className="font-bold text-base line-clamp-1" title={title}>
          {title}
        </h3>
        <p className="text-xs text-gray-400 mb-2">
          {date ? date.split('-')[0] : 'TBA'}
        </p>
        <p className="text-xs text-gray-500 line-clamp-3 leading-relaxed">
          {work.overview || 'No description available.'}
        </p>
      </div>
    </Link>
  );
};