import { publicClient, authClient } from '@/lib/apiClient';
import { ApiResponse } from '@/types/api';
import { SignInRequest, SignUpRequest } from '@/lib/validations/auth';
import { SearchResponse } from '@/types/search';

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
  }
};