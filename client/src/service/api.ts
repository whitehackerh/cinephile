import { publicClient, authClient } from '@/lib/apiClient';
import { ApiResponse } from '@/types/api';
import { Review, PostReviewsRequest, PatchReviewsRequest, GetReviewQueryParams } from '@/types/review';
import { SignInRequest, SignUpRequest } from '@/lib/validations/auth';
import { Collection, PostCollectionsRequest, PatchCollectionsRequest, PostCollectionWorksRequest, CollectionSummary } from '@/types/collection';
import { SearchResponse } from '@/types/search';
import { Movie } from '@/types/movie';
import { TvEpisode } from '@/types/tvEpisode';
import { TvSeason } from '@/types/tvSeason';
import { TvSeries } from '@/types/tvSeries';

export const apiService = {
  async signUp(data: SignUpRequest) {
    const response = await publicClient.post<ApiResponse<any>>('/signup', data);
    return response.data;
  },

  async signIn(data: SignInRequest) {
    const response = await publicClient.post<ApiResponse<any>>('/signin', data);

    const authHeader = response.headers['authorization'];
    if (authHeader && authHeader.startsWith('Bearer ')) {
      const token = authHeader.substring(7);
      localStorage.setItem('auth_token', token);
    }
    
    return response.data;
  },

  async search(q: string, page: number = 1): Promise<SearchResponse> {
    const response = await authClient.get<ApiResponse<SearchResponse>>('/search', {
      params: { q, page }
    });
    const apiRes = response.data;
    if (apiRes.error) {
      throw new Error(apiRes.error.message);
    }
    if (!apiRes.data) {
      throw new Error('Response data is missing');
    }
    return apiRes.data;
  },

  async getMovieDetail(id: string): Promise<Movie> {
    const response = await authClient.get<ApiResponse<Movie>>(`/movie/${id}`, {});
    if (response.data.error) {
      throw new Error(response.data.error.message);
    }
    if (!response.data.data) {
      throw new Error('Response data is missing');
    }
    return response.data.data
  },

  async getTvSeries(series_id: string): Promise<TvSeries> {
    const response = await authClient.get<ApiResponse<TvSeries>>(`/tv/${series_id}`, {});
    if (response.data.error) {
      throw new Error(response.data.error.message);
    }
    if (!response.data.data) {
      throw new Error('Response data is missing');
    }
    return response.data.data
  },

  async getTvSeason(series_id: string, season_number: string): Promise<TvSeason> {
    const response = await authClient.get<ApiResponse<TvSeason>>(`/tv/${series_id}/season/${season_number}`, {});
    if (response.data.error) {
      throw new Error(response.data.error.message);
    }
    if (!response.data.data) {
      throw new Error('Response data is missing');
    }
    return response.data.data
  },

  async getTvEpisode(series_id: string, season_number: string, episode_number: string): Promise<TvEpisode> {
    const response = await authClient.get<ApiResponse<TvEpisode>>(`/tv/${series_id}/season/${season_number}/episode/${episode_number}`, {});
    if (response.data.error) {
      throw new Error(response.data.error.message);
    }
    if (!response.data.data) {
      throw new Error('Response data is missing');
    }
    return response.data.data
  },

  async postReviews(input: PostReviewsRequest): Promise<Review> {
    const response = await authClient.post<ApiResponse<Review>>(`/reviews`, input);
    if (response.data.error) {
      throw new Error(response.data.error.message);
    }
    if (!response.data.data) {
      throw new Error('Response data is missing');
    }
    return response.data.data
  },

  async patchReviews(id: string, input: PatchReviewsRequest): Promise<Review> {
    const response = await authClient.patch<ApiResponse<Review>>(`/reviews/${id}`, input);
    if (response.data.error) {
      throw new Error(response.data.error.message);
    }
    if (!response.data.data) {
      throw new Error('Response data is missing');
    }
    return response.data.data
  },

  async deleteReviews(id: string): Promise<void> {
    const response = await authClient.delete<ApiResponse<void>>(`/reviews/${id}`);
    if (response.data.error) {
      throw new Error(response.data.error.message);
    }
  },

  async getReviews(): Promise<Review[]> {
    const response = await authClient.get<ApiResponse<Review[]>>('/reviews');
    if (response.data.error) {
      throw new Error(response.data.error.message);
    }
    if (response.data.data == null) {
      return [];
    }
    return response.data.data
  },

  async getReview(queryParams: GetReviewQueryParams): Promise<Review | null> {
    const response = await authClient.get<ApiResponse<Review>>('/reviews/find', {
      params: queryParams
    });
    if (response.data.error) {
      throw new Error(response.data.error.message);
    }
    return response.data.data
  },

  async postCollections(input: PostCollectionsRequest): Promise<Collection> {
    const response = await authClient.post<ApiResponse<Collection>>('/collections', input);
    if (response.data.error) {
      throw new Error(response.data.error.message);
    }
    if (!response.data.data) {
      throw new Error('Response data is missing');
    }
    return response.data.data
  },

  async getCollection(id: string): Promise<Collection> {
    const response = await authClient.get<ApiResponse<Collection>>(`/collections/${id}`);
    if (response.data.error) {
      throw new Error(response.data.error.message);
    }
    if (!response.data.data) {
      throw new Error('Response data is missing');
    }
    return response.data.data;
  },

  async patchCollections(id: string, input: PatchCollectionsRequest): Promise<Collection> {
    const response = await authClient.patch<ApiResponse<Collection>>(`/collections/${id}`, input);
    if (response.data.error) {
      throw new Error(response.data.error.message);
    }
    if (!response.data.data) {
      throw new Error('Response data is missing');
    }
    return response.data.data;
  },

  async deleteCollections(id: string): Promise<void> {
    const response = await authClient.delete<ApiResponse<void>>(`/collections/${id}`);
    if (response.data.error) {
      throw new Error(response.data.error.message);
    }
  },

  async postCollectionWorks(id: string, input: PostCollectionWorksRequest): Promise<Collection> {
    const response = await authClient.post<ApiResponse<Collection>>(`/collections/${id}/works`, input);
    if (response.data.error) {
      throw new Error(response.data.error.message);
    }
    if (!response.data.data) {
      throw new Error('Response data is missing');
    }
    return response.data.data
  },

  async deleteCollectionWork(id: string, collectionWorkId: string): Promise<void> {
    const response = await authClient.delete<ApiResponse<void>>(`/collections/${id}/works/${collectionWorkId}`);
    if (response.data.error) {
      throw new Error(response.data.error.message);
    }
  },

  async getCollections(): Promise<CollectionSummary[]> {
    const response = await authClient.get<ApiResponse<CollectionSummary[]>>('/collections');
    if (response.data.error) {
      throw new Error(response.data.error.message);
    }
    if (response.data.data == null) {
      return [];
    }
    return response.data.data
  },
};
