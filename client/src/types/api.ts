export interface ApiResponse<T = any> {
  uri: string;
  timestamp: string;
  data: T | null;
  error: ApiErrorDetail | null;
}

export interface ApiErrorDetail {
  code: string;
  message: string;
}

export type ApiErrorResponse = ApiResponse<null> & {
  error: ApiErrorDetail;
};