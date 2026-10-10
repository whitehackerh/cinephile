'use client';

import Link from 'next/link';
import { useEffect, useState } from 'react';

export default function HeaderNav() {
  const [isLoggedIn, setIsLoggedIn] = useState(false);

  useEffect(() => {
    const token = localStorage.getItem('auth_token');
    setIsLoggedIn(!!token);
  }, []);

  const handleSignout = () => {
    localStorage.removeItem('auth_token');
    window.location.href = '/signin';
  };

  return (
    <nav className="flex items-center gap-10 text-[10px] font-bold uppercase tracking-[0.2em] text-gray-400">
      {isLoggedIn ? (
        <>
          <Link
            href="/search"
            className="transition-colors hover:text-white"
          >
            Search
          </Link>
          <Link
            href="/reviews"
            className="transition-colors hover:text-white"
          >
            Reviews
          </Link>
          <Link
            href="/collections"
            className="transition-colors hover:text-white"
          >
            Collections
          </Link>
          <button
            onClick={handleSignout}
            className="uppercase transition-colors hover:text-red-500"
          >
            Sign out
          </button>
        </>
      ) : (
        <>
          <Link href="/signin" className="transition-colors hover:text-gold">Sign In</Link>
          <Link href="/signup" className="transition-colors hover:text-gold">Sign Up</Link>
        </>
      )}
    </nav>
  );
}
