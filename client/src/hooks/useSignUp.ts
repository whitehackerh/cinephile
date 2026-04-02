import { useState } from 'react';
import { AxiosError } from 'axios';
import { apiClient } from '@/lib/apiClient';
import { signUpSchema } from '@/lib/validations/auth';
import { ApiResponse } from '@/types/api';

export const useSignUp = () => {
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [errors, setErrors] = useState<string[]>([]);
  const [success, setSuccess] = useState(false);

  const handleSignUp = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    
    // 状態のリセット
    setIsSubmitting(true);
    setErrors([]);
    setSuccess(false);

    // フォームデータの抽出
    const formData = new FormData(event.currentTarget);
    const data = Object.fromEntries(formData.entries());

    // 1. Zodによるバリデーション実行
    const result = signUpSchema.safeParse(data);

    // バリデーション失敗時の処理
    if (!result.success) {
      // ZodErrorの型定義に合わせ、flatten() または issues からメッセージを抽出
      const fieldErrors = result.error.flatten().fieldErrors;
      const errorMessages = Object.values(fieldErrors).flat().filter(Boolean) as string[];
      
      setErrors(errorMessages.map(msg => msg.toUpperCase()));
      setIsSubmitting(false);
      return;
    }

    // 2. APIリクエスト (バリデーション済みの result.data を使用)
    try {
      const response = await apiClient.post<ApiResponse<any>>('/signup', result.data);

      // Rust側から正常なレスポンス（dataがnullでない）が返った場合
      if (response.data && response.data.data) {
        setSuccess(true);
      } else if (response.data.error) {
        // サーバー側で定義されたエラーがある場合
        setErrors([response.data.error.message.toUpperCase()]);
      }
    } catch (error) {
      // Axiosエラーのハンドリング
      const axiosError = error as AxiosError<ApiResponse<null>>;
      
      // src/types/api.ts の ApiErrorDetail 構造に従って抽出
      const serverError = axiosError.response?.data?.error;
      const fallbackMsg = 'CONNECTION ERROR TO RUST SERVER';
      
      setErrors([(serverError?.message || fallbackMsg).toUpperCase()]);
    } finally {
      setIsSubmitting(false);
    }
  };

  return {
    handleSignUp,
    isSubmitting,
    errors,
    success,
  };
};