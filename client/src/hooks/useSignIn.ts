import { useState } from 'react';
import { AxiosError } from 'axios';
import { apiService } from '@/service/api';
import { signInSchema } from '@/lib/validations/auth';
import { ApiResponse } from '@/types/api';
import { useRouter } from 'next/navigation';

export const useSignIn = () => {
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [errors, setErrors] = useState<string[]>([]);
  const router = useRouter();

  const handleSignIn = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setIsSubmitting(true);
    setErrors([]);

    const formData = new FormData(event.currentTarget);
    const data = Object.fromEntries(formData.entries());

    const result = signInSchema.safeParse(data);
    if (!result.success) {
      const fieldErrors = result.error.flatten().fieldErrors;
      setErrors(Object.values(fieldErrors).flat().map(msg => msg!.toUpperCase()));
      setIsSubmitting(false);
      return;
    }

    try {
      const response = await apiService.signIn(result.data);

      if (response.data) {
        router.push('/');
      }
    } catch (error) {
      const axiosError = error as AxiosError<ApiResponse<null>>;
      const serverError = axiosError.response?.data?.error;
      const fallbackMsg = 'INVALID EMAIL OR PASSWORD';
      setErrors([(serverError?.message || fallbackMsg).toUpperCase()]);
    } finally {
      setIsSubmitting(false);
    }
  };

  return { handleSignIn, isSubmitting, errors };
};