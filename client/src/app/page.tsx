import Link from 'next/link';

export default function Home() {
  return (
    <div className="flex flex-col items-center justify-center min-h-[60vh] text-center space-y-12">
      <div className="space-y-4">
        <h2 className="text-5xl md:text-7xl font-serif tracking-[0.3em] text-white opacity-90">
          ARCHIVE YOUR <br /> 
          {/* text-[#d4af37] ではなく text-gold を使用 */}
          <span className="text-gold">CINEMA</span>
        </h2>
        <p className="max-w-md mx-auto text-gray-400/50 text-[10px] uppercase tracking-[0.4em] leading-loose">
          Record every frame, every emotion. <br />
          Your personal theater database starts here.
        </p>
      </div>
      
      <Link 
        href="/signup" 
        className="px-12 py-4 border border-gold text-gold text-[10px] font-bold uppercase tracking-[0.5em] hover:bg-gold hover:text-black transition-all duration-700 ease-in-out"
      >
        Get Started
      </Link>
    </div>
  );
}