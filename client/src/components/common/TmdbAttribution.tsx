export const TmdbAttribution = () => {
  return (
    <footer className="w-full py-10 px-6 border-t border-gray-800 bg-gray-900 mt-auto">
      <div className="max-w-7xl mx-auto flex flex-col items-center gap-4">
        {/* TMDB Logo */}
        <a 
          href="https://www.themoviedb.org/" 
          target="_blank" 
          rel="noopener noreferrer"
          className="transition-opacity hover:opacity-80"
        >
          <img 
            src="https://www.themoviedb.org/assets/2/v4/logos/v2/blue_short-8e7b30f73a4020692ccca9c88bafe5dcb6f8a62a4c6bc55cd9ba82bb2cd95f6c.svg" 
            alt="The Movie Database" 
            width="100" 
          />
        </a>

        {/* Legal Notice (Required by TMDB) */}
        <p className="text-[10px] text-gray-500 uppercase tracking-widest text-center max-w-lg leading-relaxed">
          This product uses the TMDB API but is not endorsed or certified by TMDB.
        </p>
      </div>
    </footer>
  );
};