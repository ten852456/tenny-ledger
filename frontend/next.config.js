/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  images: {
    domains: ['localhost'],
    formats: ['image/webp'],
  },
  async rewrites() {
    return [
      {
        source: '/api/ocr/:path*',
        destination: 'http://localhost:8080/api/ocr/:path*' // Proxy to Rust backend
      }
    ];
  }
};

module.exports = nextConfig; 