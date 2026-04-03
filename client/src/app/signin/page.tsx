'use client';

import { useSignIn } from '@/hooks/useSignIn';
import Link from 'next/link';

export default function SignInPage() {
  const { handleSignIn, isSubmitting, errors } = useSignIn();

  return (
    <div className="max-w-sm mx-auto space-y-16 py-12">
      {/* HEADER SECTION */}
      <div className="text-center space-y-4">
        <h2 className="font-serif text-3xl tracking-[0.5em] text-gold uppercase">Sign In</h2>
        <div className="h-px w-12 bg-gold/30 mx-auto"></div>
      </div>

      {/* FORM SECTION */}
      <form onSubmit={handleSignIn} className="space-y-10">
        <div className="space-y-6">
          <div className="group">
            <label className="text-[9px] uppercase tracking-[0.3em] text-gray-500 group-focus-within:text-gold transition-colors">
              Email Address
            </label>
            <input 
              name="email" 
              type="email" 
              autoComplete="email"
              required
              className="w-full bg-transparent border-b border-white/10 py-3 outline-none focus:border-gold transition-all text-white" 
            />
          </div>
          
          <div className="group">
            <label className="text-[9px] uppercase tracking-[0.3em] text-gray-500 group-focus-within:text-gold transition-colors">
              Password
            </label>
            <input 
              name="password" 
              type="password" 
              autoComplete="current-password"
              required
              className="w-full bg-transparent border-b border-white/10 py-3 outline-none focus:border-gold transition-all text-white" 
            />
          </div>
        </div>

        <button 
          type="submit" 
          disabled={isSubmitting} 
          className="w-full bg-gold text-black py-5 text-[10px] font-bold uppercase tracking-[0.5em] hover:brightness-110 disabled:opacity-50 transition-all cursor-pointer"
        >
          {isSubmitting ? 'Authenticating...' : 'Sign In'}
        </button>

        {/* ERROR DISPLAY */}
        {errors.length > 0 && (
          <div className="space-y-2">
            {errors.map((error, index) => (
              <p key={index} className="text-[10px] text-red-500 text-center tracking-[0.2em] uppercase">
                {error}
              </p>
            ))}
          </div>
        )}

        {/* FOOTER LINK */}
        <div className="text-center">
          <Link 
            href="/signup" 
            className="text-[9px] uppercase tracking-[0.2em] text-gray-500 hover:text-gold transition-colors"
          >
            Don't have an account? <span className="text-gold ml-2 border-b border-gold/20">Join Now</span>
          </Link>
        </div>
      </form>
    </div>
  );
}