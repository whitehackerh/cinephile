import { useState, useEffect, useCallback } from "react";
import { CollectionSummary, PostCollectionsRequest, PostCollectionWorksRequest } from '@/types/collection';
import { WorkType } from '@/types/work';
import { apiService } from '@/service/api';


interface UseAddToCollectionParams {
  isOpen: boolean;
  onClose: () => void;
  targetPath: string;
  workType: WorkType;
}

export function useAddToCollection({
  isOpen,
  onClose,
  targetPath,
  workType,
}: UseAddToCollectionParams) {
  const [collections, setCollections] = useState<CollectionSummary[]>([]);
  const [loading, setLoading] = useState<boolean>(false);
  const [addingId, setAddingId] = useState<string | null>(null);

  const [isCreating, setIsCreating] = useState<boolean>(false);
  const [newTitle, setNewTitle] = useState<string>("");
  const [newDescription, setNewDescription] = useState<string>("");
  const [titleError, setTitleError] = useState<string | null>(null);
  const [submitting, setSubmitting] = useState<boolean>(false);

  const fetchCollections = useCallback(async () => {
    setLoading(true);
    try {
      const data = await apiService.getCollections();
      setCollections(data);
    } catch (err) {
      console.error("Failed to fetch collections", err);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    if (isOpen) {
      fetchCollections();
      setIsCreating(false);
      setNewTitle("");
      setNewDescription("");
      setTitleError(null);
    }
  }, [isOpen, fetchCollections]);

  const handleAddWork = async (collectionId: string) => {
    setAddingId(collectionId);
    try {
      const input: PostCollectionWorksRequest = {
        target_path: targetPath,
        work_type: workType,
      };
      const createdCollection = await apiService.postCollectionWorks(collectionId, input);
      const addedWork = createdCollection.works.find(
        (work) => work.target_path === targetPath
      );

      if (!addedWork) {
        throw new Error('Added work was not found in the collection response');
      }

      setCollections((prev) =>
        prev.map((c) =>
          c.id === collectionId
            ? {
                ...c,
                work_summaries: c.work_summaries.some(
                  (work) => work.target_path === targetPath
                )
                  ? c.work_summaries
                  : [
                      ...c.work_summaries,
                      {
                        id: addedWork.id,
                        target_path: targetPath,
                      },
                    ],
              }
            : c
        )
      );
    } catch (err) {
      console.error("Failed to add work to collection", err);
    } finally {
      setAddingId(null);
    }
  };

  const handleRemoveWork = async (
    collectionId: string,
    collectionWorkId: string
  ) => {
    setAddingId(collectionId);
    try {
      await apiService.deleteCollectionWorks(collectionId, collectionWorkId);

      setCollections((prev) =>
        prev.map((c) =>
          c.id === collectionId
            ? {
                ...c,
                work_summaries: c.work_summaries.filter(
                  (work) => work.id !== collectionWorkId
                ),
              }
            : c
        )
      );
    } catch (err) {
      console.error("Failed to remove work from collection", err);
    } finally {
      setAddingId(null);
    }
  };

  const handleCreateCollection = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newTitle.trim()) {
      setTitleError("Title is required");
      return;
    }

    setSubmitting(true);
    try {
      const createInput: PostCollectionsRequest = {
        title: newTitle.trim(),
        description: newDescription.trim() || null,
      };

      const createdCollection = await apiService.postCollections(createInput);
      await handleAddWork(createdCollection.id);

      onClose();
    } catch (err) {
      console.error("Failed to create and add", err);
    } finally {
      setSubmitting(false);
    }
  };

  return {
    state: {
      collections,
      loading,
      addingId,
      isCreating,
      newTitle,
      newDescription,
      titleError,
      submitting,
    },
    actions: {
      setIsCreating,
      setNewTitle: (val: string) => {
        setNewTitle(val);
        if (titleError) setTitleError(null);
      },
      setNewDescription,
      handleAddWork,
      handleRemoveWork,
      handleCreateCollection,
    },
  };
};