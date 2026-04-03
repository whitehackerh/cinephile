import { publicClient, authClient } from '@/lib/apiClient';
import { ApiResponse } from '@/types/api';
import { SignInRequest, SignUpRequest } from '@/lib/validations/auth';

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
  }
};