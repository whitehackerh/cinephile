'use client';

import Link from 'next/link';
import { useParams } from 'next/navigation';
import CollectionEditForm from '@/components/collection/CollectionEditForm';
import CollectionWorkCard from '@/components/collection/CollectionWorkCard';
import { useCollectionDetail } from '@/hooks/useCollectionDetail';
import { getImageUrl } from '@/utils/tmdb';

export default function CollectionDetailPage() {
  const params = useParams<{ id: string }>();
  const collectionId = params.id;

  const {
    collection,
    isLoading,
    error,
    isEditing,
    title,
    description,
    isSaving,
    isDeleting,
    deletingWorkId,
    setTitle,
    setDescription,
    toggleEditing,
    handleUpdate,
    handleDeleteCollection,
    handleDeleteWork
  } = useCollectionDetail(collectionId);

  if (isLoading) {
    return <p className="py-20 text-center text-sm text-gray-400">Loading collection...</p>;
  }

  if (!collection) {
    return (
      <div className="mx-auto max-w-3xl px-4 py-16 text-center">
        <p className="text-sm text-red-400">{error || 'Collection not found.'}</p>
        <Link href="/collections" className="mt-6 inline-block text-xs uppercase tracking-widest text-gold hover:text-white">
          ← Back to collections
        </Link>
      </div>
    );
  }

  return (
    <div className="mx-auto min-h-[70vh] max-w-6xl px-4 py-8 text-white sm:px-6">
      <Link href="/collections" className="text-xs uppercase tracking-[0.2em] text-gray-500 transition-colors hover:text-gold">
        ← All collections
      </Link>

      <header className="mt-6 overflow-hidden border border-white/10 bg-white/[0.02]">
        <div className="relative min-h-52 bg-gradient-to-br from-zinc-900 to-black sm:min-h-64">
          {collection.cover_image_path && (
            <img
              src={getImageUrl(collection.cover_image_path, 'w500')}
              alt={collection.title}
              className="absolute inset-0 h-full w-full object-cover opacity-30"
            />
          )}
          <div className="absolute inset-0 bg-gradient-to-r from-black via-black/70 to-transparent" />
          <div className="relative flex min-h-52 flex-col justify-end p-6 sm:min-h-64 sm:p-10">
            <h1 className="max-w-3xl break-words font-serif text-4xl tracking-wide sm:text-5xl">{collection.title}</h1>
            <p className="mt-4 max-w-2xl whitespace-pre-wrap text-sm leading-6 text-gray-300">
              {collection.description || 'No description'}
            </p>
          </div>
        </div>
        <div className="flex flex-wrap gap-3 border-t border-white/10 p-4 sm:px-6">
          <button
            type="button"
            onClick={toggleEditing}
            aria-label="Edit collection"
            title="Edit collection"
            className="inline-flex h-10 w-10 items-center justify-center border border-gold/60 text-gold transition-colors hover:bg-gold hover:text-black"
          >
            <svg aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <path d="M12 20h9" />
              <path d="M16.5 3.5a2.12 2.12 0 0 1 3 3L8 18l-4 1 1-4Z" />
            </svg>
          </button>
          <button
            type="button"
            onClick={handleDeleteCollection}
            disabled={isDeleting}
            aria-label={isDeleting ? 'Deleting collection' : 'Delete collection'}
            title={isDeleting ? 'Deleting collection' : 'Delete collection'}
            className="inline-flex h-10 w-10 items-center justify-center border border-red-500/40 text-red-400 transition-colors hover:bg-red-500 hover:text-white disabled:opacity-50"
          >
            <svg aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
              <path d="M3 6h18" />
              <path d="M8 6V4h8v2" />
              <path d="m19 6-1 14H6L5 6" />
              <path d="M10 11v5" />
              <path d="M14 11v5" />
            </svg>
          </button>
        </div>
      </header>

      {error && (
        <div role="alert" className="mt-5 border border-red-500/30 bg-red-500/5 px-4 py-3 text-sm text-red-300">
          {error}
        </div>
      )}

      {isEditing && (
        <CollectionEditForm
          title={title}
          description={description}
          isSaving={isSaving}
          onTitleChange={setTitle}
          onDescriptionChange={setDescription}
          onSubmit={handleUpdate}
          onClose={toggleEditing}
        />
      )}

      <section className="mt-10">
        {collection.works.length === 0 ? (
          <div className="border border-dashed border-white/15 px-6 py-14 text-center">
            <p className="font-serif text-xl text-gray-300">No works yet</p>
            <p className="mt-2 text-sm text-gray-500">Find a title and add it to this collection.</p>
            <Link href="/search" className="mt-5 inline-block text-xs uppercase tracking-widest text-gold hover:text-white">
              Explore titles →
            </Link>
          </div>
        ) : (
          <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
            {collection.works.map((collectionWork) => (
              <CollectionWorkCard
                key={collectionWork.id}
                collectionWork={collectionWork}
                deleting={deletingWorkId === collectionWork.id}
                onDelete={handleDeleteWork}
              />
            ))}
          </div>
        )}
      </section>
    </div>
  );
}