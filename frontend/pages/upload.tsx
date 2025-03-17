import React, { useState } from 'react';
import { useRouter } from 'next/router';
import Layout from '../components/Layout';
import ImageUploader from '../components/ImageUploader';
import TransactionForm from '../components/TransactionForm';
import BillPreview from '../components/BillPreview';
import { transactionsAPI } from '../services/api';

const UploadPage: React.FC = () => {
  const router = useRouter();
  const [step, setStep] = useState<'upload' | 'confirm'>('upload');
  const [extractedData, setExtractedData] = useState<any>(null);
  const [ocrText, setOcrText] = useState<string>('');
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleProcessed = (data: any) => {
    setOcrText(data.text);
    setExtractedData(data.extractedData);
    setStep('confirm');
  };

  const handleError = (errorMessage: string) => {
    setError(errorMessage);
  };

  const handleSubmit = async (formData: any) => {
    try {
      setIsSubmitting(true);
      setError(null);

      // Submit transaction to API
      await transactionsAPI.createTransaction(formData);
      
      // Redirect to transactions list
      router.push('/transactions');
    } catch (err) {
      console.error('Error creating transaction:', err);
      setError(err instanceof Error ? err.message : 'Failed to create transaction');
      setIsSubmitting(false);
    }
  };

  return (
    <Layout title="Upload Receipt">
      <div className="max-w-4xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
        <h1 className="text-2xl font-semibold text-gray-900">Upload Receipt</h1>
        
        {error && (
          <div className="mt-4 bg-red-50 border-l-4 border-red-400 p-4">
            <div className="flex">
              <div className="flex-shrink-0">
                <svg className="h-5 w-5 text-red-400" viewBox="0 0 20 20" fill="currentColor">
                  <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clipRule="evenodd" />
                </svg>
              </div>
              <div className="ml-3">
                <p className="text-sm text-red-700">{error}</p>
              </div>
            </div>
          </div>
        )}
        
        <div className="mt-6">
          {step === 'upload' ? (
            <div>
              <p className="mb-4 text-gray-600">
                Upload your receipt or bill image to automatically extract transaction details.
              </p>
              <ImageUploader onProcessed={handleProcessed} onError={handleError} />
            </div>
          ) : (
            <div className="grid grid-cols-1 gap-6 lg:grid-cols-2">
              <div>
                <h2 className="text-lg font-medium text-gray-900 mb-4">Review Extracted Data</h2>
                <TransactionForm 
                  initialData={{
                    amount: extractedData.total,
                    date: extractedData.date,
                    merchant: extractedData.merchant,
                    items: extractedData.items,
                  }}
                  onSubmit={handleSubmit}
                  isSubmitting={isSubmitting}
                />
                <button
                  onClick={() => setStep('upload')}
                  className="mt-4 text-blue-600 hover:text-blue-800"
                >
                  Upload a different image
                </button>
              </div>
              
              <div>
                <h2 className="text-lg font-medium text-gray-900 mb-4">OCR Results</h2>
                <BillPreview text={ocrText} />
              </div>
            </div>
          )}
        </div>
      </div>
    </Layout>
  );
};

export default UploadPage;