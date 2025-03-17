import React, { useCallback, useState } from 'react';
import { useDropzone } from 'react-dropzone';
import Image from 'next/image';

interface ImageUploaderProps {
  onImageSelect: (file: File) => void;
  maxSize?: number;
  acceptedFileTypes?: string[];
  className?: string;
}

const ImageUploader: React.FC<ImageUploaderProps> = ({
  onImageSelect,
  maxSize = 5242880, // 5MB
  acceptedFileTypes = ['image/jpeg', 'image/png', 'application/pdf'],
  className = '',
}) => {
  const [preview, setPreview] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const onDrop = useCallback((acceptedFiles: File[], rejectedFiles: any[]) => {
    // Handle rejected files
    if (rejectedFiles.length > 0) {
      const { errors } = rejectedFiles[0];
      if (errors[0]?.code === 'file-too-large') {
        setError(`File is too large. Max size is ${maxSize / 1024 / 1024}MB`);
      } else if (errors[0]?.code === 'file-invalid-type') {
        setError('Invalid file type. Please upload an image (JPEG, PNG) or PDF.');
      } else {
        setError('Error uploading file. Please try again.');
      }
      return;
    }

    // Handle accepted files
    if (acceptedFiles.length > 0) {
      const file = acceptedFiles[0];
      setError(null);
      
      // Create preview
      const reader = new FileReader();
      reader.onload = () => {
        setPreview(reader.result as string);
      };
      reader.readAsDataURL(file);
      
      // Pass file to parent component
      onImageSelect(file);
    }
  }, [maxSize, onImageSelect]);

  const { getRootProps, getInputProps, isDragActive } = useDropzone({
    onDrop,
    accept: {
      'image/jpeg': ['.jpg', '.jpeg'],
      'image/png': ['.png'],
      'application/pdf': ['.pdf'],
    },
    maxSize,
    multiple: false,
  });

  return (
    <div className={`w-full ${className}`}>
      <div
        {...getRootProps({
          className: `border-2 border-dashed rounded-lg p-8 text-center transition-colors ${
            isDragActive ? 'border-primary-500 bg-primary-50' : 'border-gray-300 hover:border-primary-300'
          }`,
        })}
      >
        <input {...getInputProps()} />
        
        {preview ? (
          <div className="flex flex-col items-center">
            <div className="relative w-48 h-48 mb-4">
              <Image
                src={preview}
                alt="Preview"
                fill
                className="object-contain"
              />
            </div>
            <p className="text-sm text-gray-600">
              Drop another file to replace this one, or click to select a file
            </p>
          </div>
        ) : (
          <div className="flex flex-col items-center">
            <svg
              className="w-12 h-12 mb-4 text-gray-400"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
              />
            </svg>
            {isDragActive ? (
              <p className="text-primary-600">Drop the bill image here</p>
            ) : (
              <div>
                <p className="mb-2 text-sm text-gray-700">
                  Drag and drop your bill image, or click to select
                </p>
                <p className="text-xs text-gray-500">
                  Supported formats: JPEG, PNG, PDF (Max {maxSize / 1024 / 1024}MB)
                </p>
              </div>
            )}
          </div>
        )}
      </div>
      
      {error && (
        <div className="mt-2 text-sm text-red-600">
          {error}
        </div>
      )}
    </div>
  );
};

export default ImageUploader; 