import type { NextApiRequest, NextApiResponse } from 'next';
import httpProxyMiddleware from 'next-http-proxy-middleware';

export const config = {
  api: {
    bodyParser: false,
  },
};

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  const RUST_API_URL = process.env.RUST_API_URL || 'http://localhost:8080';
  
  return httpProxyMiddleware(req, res, {
    target: RUST_API_URL,
    pathRewrite: [
      {
        patternStr: '^/api/proxy',
        replaceStr: '',
      },
    ],
    changeOrigin: true,
  });
} 