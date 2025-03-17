import React, { useState } from 'react';
import { NextPage } from 'next';
import Head from 'next/head';
import { useRouter } from 'next/router';
import ImageUploader from '../components/ImageUploader';
import useUpload from '../hooks/useUpload';

const UploadPage: NextPage = () => {
  const router = useRouter();
  const { uploadImage, isUploading, progress, error, reset } = useUpload();
  const [selectedFile, setSelectedFile] = useState<File | null>(null);
  const [ocrResult, setOcrResult] = useState<any>(null);
  
  const handleImageSelect = (file: File) => {
    setSelectedFile(file);
    setOcrResult(null);
  };
  
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!selectedFile) {
      return;
    }
    
    try {
      const result = await uploadImage(selectedFile);
      setOcrResult(result);
    } catch (err) {
      console.error('Upload failed:', err);
    }
  };
  
  const handleCreateTransaction = () => {
    if (!ocrResult) return;
    
    // Navigate to transaction creation page with extracted data
    router.push({
      pathname: '/transactions/new',
      query: {
        amount: ocrResult.extractedData.total || '',
        date: ocrResult.extractedData.date || '',
        merchant: ocrResult.extractedData.merchant || '',
      },
    });
  };
  
  return (
    <>
      <Head>
        <title>Upload Bill | Tenny Ledger</title>
      </Head>
      
      <div className="container max-w-4xl mx-auto px-4 py-8">
        <h1 className="text-3xl font-bold mb-6">Upload Bill</h1>
        
        <div className="bg-white rounded-lg shadow-md p-6">
          <form onSubmit={handleSubmit}>
            <div className="mb-6">
              <ImageUploader onImageSelect={handleImageSelect} />
            </div>
            
            {error && (
              <div className="mb-4 p-3 bg-red-50 text-red-700 rounded-md">
                {error}
              </div>
            )}
            
            <div className="flex justify-end">
              <button
                type="submit"
                disabled={!selectedFile || isUploading}
                className={`px-4 py-2 rounded-lg font-medium ${
                  !selectedFile || isUploading
                    ? 'bg-gray-300 text-gray-500 cursor-not-allowed'
                    : 'bg-primary-600 text-white hover:bg-primary-700'
                }`}
              >
                {isUploading ? `Processing (${progress}%)` : 'Process Bill'}
              </button>
            </div>
          </form>
          
          {ocrResult && (
            <div className="mt-8 border-t pt-6">
              <h2 className="text-xl font-semibold mb-4">Extracted Information</h2>
              
              <div className="space-y-4">
                <div className="bg-gray-50 p-4 rounded-md">
                  <h3 className="font-medium text-gray-700 mb-2">Total Amount</h3>
                  <p className="text-2xl font-bold">
                    ${ocrResult.extractedData.total?.toFixed(2) || 'Not detected'}
                  </p>
                </div>
                
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                  <div className="bg-gray-50 p-4 rounded-md">
                    <h3 className="font-medium text-gray-700 mb-2">Merchant</h3>
                    <p>{ocrResult.extractedData.merchant || 'Not detected'}</p>
                  </div>
                  
                  <div className="bg-gray-50 p-4 rounded-md">
                    <h3 className="font-medium text-gray-700 mb-2">Date</h3>
                    <p>{ocrResult.extractedData.date || 'Not detected'}</p>
                  </div>
                </div>
                
                {ocrResult.extractedData.items && ocrResult.extractedData.items.length > 0 && (
                  <div className="bg-gray-50 p-4 rounded-md">
                    <h3 className="font-medium text-gray-700 mb-2">Items</h3>
                    <ul className="divide-y">
                      {ocrResult.extractedData.items.map((item: any, index: number) => (
                        <li key={index} className="py-2 flex justify-between">
                          <span>{item.name}</span>
                          <span>${item.price?.toFixed(2) || 'N/A'}</span>
                        </li>
                      ))}
                    </ul>
                  </div>
                )}
                
                <div className="bg-gray-50 p-4 rounded-md">
                  <h3 className="font-medium text-gray-700 mb-2">OCR Confidence</h3>
                  <div className="w-full bg-gray-200 rounded-full h-2.5">
                    <div 
                      className="bg-blue-600 h-2.5 rounded-full" 
                      style={{ width: `${(ocrResult.confidence * 100).toFixed(0)}%` }}
                    ></div>
                  </div>
                  <p className="mt-1 text-sm text-gray-500">
                    {(ocrResult.confidence * 100).toFixed(0)}% confidence
                  </p>
                </div>
              </div>
              
              <div className="mt-6 flex justify-end">
                <button
                  onClick={handleCreateTransaction}
                  className="px-4 py-2 bg-green-600 text-white rounded-lg font-medium hover:bg-green-700"
                >
                  Create Transaction
                </button>
              </div>
            </div>
          )}
        </div>
      </div>
    </>
  );
};

export default UploadPage;