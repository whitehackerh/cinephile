import './globals.css';
import Link from 'next/link';
import Image from 'next/image';

export const metadata = {
  title: 'CINEPHILE',
  description: 'Your Cinematic Journey Tracker',
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="ja">
      <body className="antialiased selection:bg-gold selection:text-black">
        <header className="fixed top-0 z-50 w-full border-b border-white/5 bg-black/60 backdrop-blur-xl">
          <div className="mx-auto flex max-w-7xl items-center justify-between px-8 py-5">
            <Link href="/" className="hover:opacity-80 transition-opacity">
              <Image 
                src="/images/logo-header.png" 
                alt="CINEPHILE LOGO" 
                // 横幅を基準にし、高さは auto で維持
                width={200} 
                height={40} 
                priority 
                className="object-contain"
                // 警告を消すための重要なスタイル
                style={{ width: 'auto', height: '40px' }} 
              />
            </Link>
            <nav className="flex gap-10 text-[10px] font-bold uppercase tracking-[0.2em] text-gray-400">
              <Link href="/movies" className="hover:text-gold transition-colors">Movies</Link>
              <Link href="/signup" className="text-gold border-b border-gold/20 pb-1">Join</Link>
            </nav>
          </div>
        </header>

        <main className="pt-32 px-6 min-h-screen bg-cinema-black">
          {children}
        </main>
      </body>
    </html>
  );
}