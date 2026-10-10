import { CollectionWork, CollectionWorkSummary } from '@/types/collectionWork';
import { WorkType } from '@/types/work';

export interface Collection {
    id: string;
    title: string;
    description: string | null;
    cover_image_path: string | null;
    works: CollectionWork[];
    created_at: string;
    updated_at: string;
}

export interface PostCollectionsRequest {
    title: string;
    description: string | null;
}

export interface PatchCollectionsRequest {
    title: string;
    description: string | null;
}

export interface PostCollectionWorksRequest {
    work_type: WorkType;
    target_path: string;
}

export interface CollectionSummary {
    id: string;
    title: string;
    description: string | null;
    cover_image_path: string | null;
    work_summaries: CollectionWorkSummary[];
    created_at: string;
    updated_at: string
}