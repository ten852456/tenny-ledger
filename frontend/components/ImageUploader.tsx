import React, { useCallback, useState } from 'react';
import { useDropzone } from 'react-dropzone';
import { ocrAPI } from '../services/api';

interface ImageUploaderProps {
  onProcessed: (data: any) => void;
  onError?: (error: string) => void;
}

const ImageUploader: React.FC<ImageUploaderProps> = ({ onProcessed, onError }) => {
  const [isUploading, setIsUploading] = useState(false);
  const [preview, setPreview] = useState<string | null>(null);

  const onDrop = useCallback(async (acceptedFiles: File[]) => {
    if (acceptedFiles.length === 0) return;

    const file = acceptedFiles[0];
    
    // Create preview
    const objectUrl = URL.createObjectURL(file);
    setPreview(objectUrl);
    
    // Upload and process
    try {
      setIsUploading(true);
      
      const formData = new FormData();
      formData.append('image', file);
      
      const response = await ocrAPI.processImage(formData);
      
      onProcessed(response.data);
    } catch (error) {
      console.error('Error processing image:', error);
      onError?.(error instanceof Error ? error.message : 'Failed to process image');
    } finally {
      setIsUploading(false);
    }
  }, [onProcessed, onError]);

  const { getRootProps, getInputProps, isDragActive } = useDropzone({
    onDrop,
    accept: {
      'image/jpeg': ['.jpg', '.jpeg'],
      'image/png': ['.png'],
      'application/pdf': ['.pdf'],
    },
    maxFiles: 1,
  });

  return (
    <div className="w-full">
      <div 
        {...getRootProps()} 
        className={`border-2 border-dashed rounded-lg p-8 text-center cursor-pointer transition-colors
          ${isDragActive ? 'border-blue-500 bg-blue-50' : 'border-gray-300 hover:border-blue-400'}`}
      >
        <input {...getInputProps()} />
        
        {preview ? (
          <div className="space-y-4">
            <img src={preview} alt="Bill preview" className="max-h-60 mx-auto" />
            <p className="text-sm text-gray-500">Drop another image to replace</p>
          </div>
        ) : (
          <div className="space-y-2">
            <svg className="w-12 h-12 mx-auto text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
            </svg>
            <p className="text-lg font-medium">Drag & drop your bill or receipt</p>
            <p className="text-sm text-gray-500">Supports JPG, PNG, and PDF</p>
          </div>
        )}
      </div>
      
      {isUploading && (
        <div className="mt-4 flex items-center justify-center">
          <div className="animate-spin rounded-full h-6 w-6 border-t-2 border-b-2 border-blue-500 mr-2"></div>
          <p>Processing your bill...</p>
        </div>
      )}
    </div>
  );
};

export default ImageUploader; 