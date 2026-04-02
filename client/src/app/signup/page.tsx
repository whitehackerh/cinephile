'use client';
import { useSignUp } from '@/hooks/useSignUp';

export default function SignUpPage() {
  // message ではなく、errors と success を受け取るように修正
  const { handleSignUp, isSubmitting, errors, success } = useSignUp();

  return (
    <div className="max-w-sm mx-auto space-y-16 py-12">
      <div className="text-center space-y-4">
        <h2 className="font-serif text-3xl tracking-[0.5em] text-gold">SIGN UP</h2>
        <div className="h-px w-12 bg-gold/30 mx-auto"></div>
      </div>

      <form onSubmit={handleSignUp} className="space-y-10">
        <div className="space-y-6">
          {/* バリデーションに合わせて Name フィールドを追加 */}
          <div className="group">
            <label className="text-[9px] uppercase tracking-[0.3em] text-gray-500 group-focus-within:text-gold transition-colors">Full Name</label>
            <input name="name" type="text" className="w-full bg-transparent border-b border-white/10 py-3 outline-none focus:border-gold transition-all" />
          </div>

          <div className="group">
            <label className="text-[9px] uppercase tracking-[0.3em] text-gray-500 group-focus-within:text-gold transition-colors">Email Address</label>
            <input name="email" type="email" className="w-full bg-transparent border-b border-white/10 py-3 outline-none focus:border-gold transition-all" />
          </div>
          
          <div className="group">
            <label className="text-[9px] uppercase tracking-[0.3em] text-gray-500 group-focus-within:text-gold transition-colors">Password</label>
            <input name="password" type="password" className="w-full bg-transparent border-b border-white/10 py-3 outline-none focus:border-gold transition-all" />
          </div>
        </div>

        <button type="submit" disabled={isSubmitting} className="w-full bg-gold text-black py-5 text-[10px] font-bold uppercase tracking-[0.5em] hover:brightness-110 disabled:opacity-50 transition-all cursor-pointer">
          {isSubmitting ? 'Processing...' : 'Create Account'}
        </button>

        {/* 成功時の表示 */}
        {success && (
          <p className="text-[10px] text-green-500 text-center tracking-[0.2em] uppercase">
            Account created successfully.
          </p>
        )}

        {/* エラー時の表示（errors配列をループで回す） */}
        {errors.length > 0 && (
          <div className="space-y-1">
            {errors.map((error, index) => (
              <p key={index} className="text-[10px] text-red-500 text-center tracking-[0.2em] uppercase">
                {error}
              </p>
            ))}
          </div>
        )}
      </form>
    </div>
  );
}