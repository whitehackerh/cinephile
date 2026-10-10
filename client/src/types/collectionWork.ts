import { Work, WorkType } from '@/types/work';

export interface CollectionWork {
    id: string;
    work_type: WorkType;
    target_path: string;
    work: Work;
    added_at: string;
}

export interface CollectionWorkSummary {
    id: string;
    target_path: string;
}