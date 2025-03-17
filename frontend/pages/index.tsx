import { useEffect } from 'react';
import { useRouter } from 'next/router';
import Head from 'next/head';

export default function Home() {
  const router = useRouter();
  
  useEffect(() => {
    router.replace('/dashboard');
  }, [router]);
  
  return (
    <div className="flex items-center justify-center min-h-screen bg-gray-100">
      <Head>
        <title>Tenny Ledger</title>
        <meta name="description" content="Tenny Ledger - Bill Scanning and Personal Finance Tracking" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      
      <div className="text-center">
        <h1 className="text-3xl font-bold text-blue-600 mb-4">Tenny Ledger</h1>
        <p className="text-gray-500">Redirecting to dashboard...</p>
        <div className="mt-4">
          <svg className="animate-spin h-8 w-8 text-blue-600 mx-auto" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
            <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
        </div>
      </div>
    </div>
  );
} 