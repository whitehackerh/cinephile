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
    <nav className="flex gap-10 text-[10px] font-bold uppercase tracking-[0.2em] text-gray-400 items-center">     
      {isLoggedIn ? (
        <>
          <Link 
            href="/search" 
            className="hover:text-white transition-colors"
          >
            Search
          </Link>
          <button 
            onClick={handleSignout}
            className="hover:text-red-500 transition-colors uppercase"
          >
            Signout
          </button>
        </>
      ) : (
        <>
          <Link href="/signin" className="hover:text-gold transition-colors">Sign In</Link>
          <Link href="/signup" className="hover:text-gold transition-colors">Sign Up</Link>
        </>
      )}
    </nav>
  );
}