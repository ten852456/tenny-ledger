import { useState } from 'react';
import axios from 'axios';

interface UploadResult {
  text: string;
  extractedData: {
    total: number | null;
    date: string | null;
    merchant: string | null;
    items: Array<{
      name: string;
      price: number | null;
      quantity: number | null;
    }>;
  };
  confidence: number;
  processingTime: number;
}

interface UseUploadReturn {
  uploadImage: (file: File) => Promise<UploadResult>;
  isUploading: boolean;
  progress: number;
  error: string | null;
  reset: () => void;
}

export default function useUpload(): UseUploadReturn {
  const [isUploading, setIsUploading] = useState<boolean>(false);
  const [progress, setProgress] = useState<number>(0);
  const [error, setError] = useState<string | null>(null);
  
  const reset = () => {
    setIsUploading(false);
    setProgress(0);
    setError(null);
  };
  
  const uploadImage = async (file: File): Promise<UploadResult> => {
    try {
      setIsUploading(true);
      setProgress(0);
      setError(null);
      
      const formData = new FormData();
      formData.append('image', file);
      
      const response = await axios.post<UploadResult>('/api/proxy/ocr', formData, {
        headers: {
          'Content-Type': 'multipart/form-data',
        },
        onUploadProgress: (progressEvent) => {
          if (progressEvent.total) {
            const percentCompleted = Math.round((progressEvent.loaded * 100) / progressEvent.total);
            setProgress(percentCompleted);
          }
        },
      });
      
      setIsUploading(false);
      return response.data;
    } catch (err) {
      setIsUploading(false);
      if (axios.isAxiosError(err) && err.response) {
        setError(err.response.data.message || 'Failed to process the image');
      } else {
        setError('An unexpected error occurred during upload');
      }
      throw err;
    }
  };
  
  return {
    uploadImage,
    isUploading,
    progress,
    error,
    reset,
  };
} 