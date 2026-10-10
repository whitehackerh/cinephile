
'use client';

import { FormEvent, useCallback, useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';
import { Collection, PatchCollectionsRequest } from '@/types/collection';
import { apiService } from '@/service/api';

export function useCollectionDetail(collectionId: string) {
  const router = useRouter();

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

  const toggleEditing = () => {
    if (collection) {
      setTitle(collection.title);
      setDescription(collection.description ?? '');
    }
    setIsEditing((current) => !current);
    setError(null);
  };

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
      await apiService.deleteCollectionWorks(collectionId, collectionWorkId);
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

  return {
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
    fetchCollection,
    toggleEditing,
    handleUpdate,
    handleDeleteCollection,
    handleDeleteWork
  };
}
