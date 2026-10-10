'use client';

import Link from 'next/link';
import { useParams, useRouter } from 'next/navigation';
import { FormEvent, useCallback, useEffect, useState } from 'react';
import { Collection, PatchCollectionsRequest } from '@/types/collection';
import { apiService } from '@/service/api';
import { getImageUrl } from '@/utils/tmdb';

export default function CollectionDetailPage() {
  const params = useParams<{ id: string }>();
  const router = useRouter();
  const collectionId = params.id;

  const [collection, setCollection] = useState<Collection | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isEditing, setIsEditing] = useState(false);
  const [title, setTitle] = useState('');
  const [description, setDescription] = useState('');
  const [isSaving, setIsSaving] = useState(false);
  const [isDeleting, setIsDeleting] = useState(false);
  const [deletingWorkId, setDeletingWorkId] = useState<string | null>(null);

  const fetchCollection = useCallback(async () => {
    setIsLoading(true);
    setError(null);
    try {
      const data = await apiService.getCollection(collectionId);
      setCollection(data);
      setTitle(data.title);
      setDescription(data.description ?? '');
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load collection.');
    } finally {
      setIsLoading(false);
    }
  }, [collectionId]);

  useEffect(() => {
    fetchCollection();
  }, [fetchCollection]);

  const handleUpdate = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    if (!title.trim()) {
      setError('Title is required.');
      return;
    }

    setIsSaving(true);
    setError(null);
    try {
      const input: PatchCollectionsRequest = {
        title: title.trim(),
        description: description.trim() || null,
      };
      const updatedCollection = await apiService.patchCollections(collectionId, input);
      setCollection(updatedCollection);
      setTitle(updatedCollection.title);
      setDescription(updatedCollection.description ?? '');
      setIsEditing(false);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to update collection.');
    } finally {
      setIsSaving(false);
    }
  };

  const handleDeleteCollection = async () => {
    if (!window.confirm(`Delete "${collection?.title}"? This cannot be undone.`)) {
      return;
    }

    setIsDeleting(true);
    setError(null);
    try {
      await apiService.deleteCollections(collectionId);
      router.push('/collections');
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to delete collection.');
      setIsDeleting(false);
    }
  };

  const handleDeleteWork = async (collectionWorkId: string, workTitle: string) => {
    if (!window.confirm(`Remove "${workTitle}" from this collection?`)) {
      return;
    }

    setDeletingWorkId(collectionWorkId);
    setError(null);
    try {
      await apiService.deleteCollectionWork(collectionId, collectionWorkId);
      setCollection((current) => current
        ? { ...current, works: current.works.filter((work) => work.id !== collectionWorkId) }
        : current
      );
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to remove work.');
    } finally {
      setDeletingWorkId(null);
    }
  };

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
            <p className="mb-3 text-[10px] uppercase tracking-[0.3em] text-gold">Your archive</p>
            <h1 className="max-w-3xl break-words font-serif text-4xl tracking-wide sm:text-5xl">{collection.title}</h1>
            <p className="mt-4 max-w-2xl whitespace-pre-wrap text-sm leading-6 text-gray-300">
              {collection.description || 'No description'}
            </p>
            <p className="mt-4 text-xs text-gray-500">
              {collection.works.length} {collection.works.length === 1 ? 'work' : 'works'}
            </p>
          </div>
        </div>
        <div className="flex flex-wrap gap-3 border-t border-white/10 p-4 sm:px-6">
          <button
            type="button"
            onClick={() => {
              setTitle(collection.title);
              setDescription(collection.description ?? '');
              setIsEditing((current) => !current);
              setError(null);
            }}
            className="border border-gold/60 px-4 py-2 text-xs font-bold uppercase tracking-[0.15em] text-gold transition-colors hover:bg-gold hover:text-black"
          >
            {isEditing ? 'Cancel edit' : 'Edit collection'}
          </button>
          <button
            type="button"
            onClick={handleDeleteCollection}
            disabled={isDeleting}
            className="border border-red-500/40 px-4 py-2 text-xs font-bold uppercase tracking-[0.15em] text-red-400 transition-colors hover:bg-red-500 hover:text-white disabled:opacity-50"
          >
            {isDeleting ? 'Deleting...' : 'Delete collection'}
          </button>
        </div>
      </header>

      {error && (
        <div role="alert" className="mt-5 border border-red-500/30 bg-red-500/5 px-4 py-3 text-sm text-red-300">
          {error}
        </div>
      )}

      {isEditing && (
        <form onSubmit={handleUpdate} className="mt-6 space-y-5 border border-white/10 bg-white/[0.02] p-5 sm:p-6">
          <h2 className="font-serif text-2xl">Edit collection</h2>
          <div>
            <label htmlFor="collection-title" className="mb-2 block text-xs font-semibold uppercase tracking-widest text-gray-400">
              Title
            </label>
            <input
              id="collection-title"
              value={title}
              onChange={(event) => setTitle(event.target.value)}
              maxLength={100}
              required
              className="w-full border border-white/15 bg-black/60 px-4 py-3 text-sm text-white outline-none transition-colors focus:border-gold"
            />
          </div>
          <div>
            <label htmlFor="collection-description" className="mb-2 block text-xs font-semibold uppercase tracking-widest text-gray-400">
              Description
            </label>
            <textarea
              id="collection-description"
              value={description}
              onChange={(event) => setDescription(event.target.value)}
              rows={4}
              className="w-full resize-y border border-white/15 bg-black/60 px-4 py-3 text-sm text-white outline-none transition-colors focus:border-gold"
            />
          </div>
          <button
            type="submit"
            disabled={isSaving}
            className="bg-gold px-5 py-3 text-xs font-bold uppercase tracking-[0.2em] text-black transition-opacity hover:opacity-80 disabled:opacity-50"
          >
            {isSaving ? 'Saving...' : 'Save changes'}
          </button>
        </form>
      )}

      <section className="mt-10">
        <div className="mb-5 flex items-end justify-between border-b border-white/10 pb-4">
          <div>
            <p className="mb-2 text-[10px] uppercase tracking-[0.3em] text-gold">Collected titles</p>
            <h2 className="font-serif text-3xl">Works</h2>
          </div>
          <span className="text-xs text-gray-500">{collection.works.length} total</span>
        </div>

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
            {collection.works.map((collectionWork) => {
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
                <article key={collectionWork.id} className="flex min-w-0 gap-4 border border-white/10 bg-white/[0.02] p-4 transition-colors hover:border-white/20">
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
                    onClick={() => handleDeleteWork(collectionWork.id, workTitle)}
                    disabled={deletingWorkId === collectionWork.id}
                    aria-label={`Remove ${workTitle} from collection`}
                    className="h-fit shrink-0 border border-white/10 px-3 py-2 text-xs text-gray-500 transition-colors hover:border-red-500/50 hover:text-red-400 disabled:opacity-40"
                  >
                    {deletingWorkId === collectionWork.id ? '...' : 'Remove'}
                  </button>
                </article>
              );
            })}
          </div>
        )}
      </section>
    </div>
  );
}
