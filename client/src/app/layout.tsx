import './globals.css';
import Link from 'next/link';
import Image from 'next/image';
import HeaderNav from '@/components/HeaderNav';

export const metadata = {
  title: 'CINEPHILE',
  description: 'Your Cinematic Journey Tracker',
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="ja">
      <body className="antialiased selection:bg-gold selection:text-black bg-black">
        <header className="fixed top-0 z-50 w-full border-b border-white/5 bg-black/60 backdrop-blur-xl">
          <div className="mx-auto flex max-w-7xl items-center justify-between px-8 py-5">
            <Link href="/" className="hover:opacity-80 transition-opacity">
              <Image 
                src="/images/logo-header.png" 
                alt="CINEPHILE LOGO" 
                width={200} 
                height={40} 
                priority 
                className="object-contain"
                style={{ width: 'auto', height: '40px' }} 
              />
            </Link>
            
            {/* クライアントコンポーネント化したナビゲーション */}
            <HeaderNav />
          </div>
        </header>

        <main className="pt-32 px-6 min-h-screen bg-cinema-black text-white">
          {children}
        </main>
      </body>
    </html>
  );
}