import * as z from 'zod';

export const signUpSchema = z.object({
  name: z.string().min(1, 'Name is required'),
  email: z.string().email('Invalid email address'),
  password: z
    .string()
    .min(1, 'Password is required')
});

export type SignUpRequest = z.infer<typeof signUpSchema>;

export const signInSchema = signUpSchema.pick({
  email: true,
  password: true,
});

export type SignInRequest = z.infer<typeof signInSchema>;