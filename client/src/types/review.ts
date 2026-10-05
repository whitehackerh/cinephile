import { Work, WorkType } from '@/types/work';

export interface PostReviewsRequest {
  rating: number;
  content: string | null;
  work_type: WorkType;
  target_path: string;
}

export interface PatchReviewsRequest {
  rating: number;
  content: string | null;
}

export interface GetReviewQueryParams {
  work_type: WorkType;
  target_path: string;
}

export interface Review {
  id: string;
  rating: number;
  content: string | null;
  work_type: WorkType;
  target_path: string;
  work: Work;
  created_at: string;
  updated_at: string;
  deleted_at: string | null;
}
