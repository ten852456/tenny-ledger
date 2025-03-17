import React, { useState } from 'react';
import { useRouter } from 'next/router';
import Layout from '@/components/Layout';
import TransactionForm from '@/components/TransactionForm';
import { useTransaction } from '@/hooks/useTransactions';

interface TransactionItem {
  name: string;
  price?: number;
  quantity?: number;
}

const TransactionDetailPage: React.FC = () => {
  const router = useRouter();
  const { id } = router.query;
  const [isEditing, setIsEditing] = useState(false);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  // In a real app, we would fetch the transaction by ID
  // For this example, we'll use a mock transaction
  const { transaction, isLoading, error: fetchError } = useTransaction(id as string);
  
  // Mock function to handle transaction update
  const handleUpdate = async (formData: any) => {
    try {
      setIsSubmitting(true);
      setError(null);
      
      // In a real app, we would make an API call here
      console.log('Updating transaction:', formData);
      
      // Simulate API delay
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      // Exit edit mode
      setIsEditing(false);
      setIsSubmitting(false);
      
      // Show success (in a real app, we would use a toast notification)
      alert('Transaction updated successfully');
    } catch (err) {
      console.error('Error updating transaction:', err);
      setError(err instanceof Error ? err.message : 'Failed to update transaction');
      setIsSubmitting(false);
    }
  };
  
  // Mock function to handle transaction deletion
  const handleDelete = async () => {
    if (!confirm('Are you sure you want to delete this transaction? This action cannot be undone.')) {
      return;
    }
    
    try {
      // In a real app, we would make an API call here
      console.log('Deleting transaction:', id);
      
      // Simulate API delay
      await new Promise(resolve => setTimeout(resolve, 500));
      
      // Redirect to transactions list
      router.push('/transactions');
      
      // Show success (in a real app, we would use a toast notification)
      alert('Transaction deleted successfully');
    } catch (err) {
      console.error('Error deleting transaction:', err);
      alert(err instanceof Error ? err.message : 'Failed to delete transaction');
    }
  };
  
  if (isLoading) {
    return (
      <Layout title="Transaction Details">
        <div className="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
          <div className="animate-pulse space-y-4">
            <div className="h-10 bg-gray-200 rounded w-1/4"></div>
            <div className="h-96 bg-gray-200 rounded"></div>
          </div>
        </div>
      </Layout>
    );
  }
  
  if (fetchError || !transaction) {
    return (
      <Layout title="Transaction Details">
        <div className="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
          <div className="bg-white shadow rounded-lg p-6 text-center">
            <p className="text-red-500">
              {fetchError || 'Transaction not found. It may have been deleted or never existed.'}
            </p>
            <button
              type="button"
              onClick={() => router.push('/transactions')}
              className="mt-4 inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50"
            >
              Back to Transactions
            </button>
          </div>
        </div>
      </Layout>
    );
  }
  
  return (
    <Layout title={`Transaction - ${transaction.merchant}`}>
      <div className="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
        <div className="flex justify-between items-center mb-6">
          <h1 className="text-2xl font-semibold text-gray-900">
            {isEditing ? 'Edit Transaction' : 'Transaction Details'}
          </h1>
          <div className="flex space-x-4">
            {!isEditing ? (
              <>
                <button
                  type="button"
                  onClick={() => setIsEditing(true)}
                  className="inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50"
                >
                  Edit
                </button>
                <button
                  type="button"
                  onClick={handleDelete}
                  className="inline-flex items-center px-4 py-2 border border-red-300 text-sm font-medium rounded-md text-red-700 bg-white hover:bg-red-50"
                >
                  Delete
                </button>
              </>
            ) : (
              <button
                type="button"
                onClick={() => setIsEditing(false)}
                className="inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50"
              >
                Cancel
              </button>
            )}
          </div>
        </div>
        
        {error && (
          <div className="mb-6 bg-red-50 border border-red-200 text-red-700 px-4 py-3 rounded relative" role="alert">
            <span className="block sm:inline">{error}</span>
          </div>
        )}
        
        <div className="bg-white shadow rounded-lg overflow-hidden">
          {isEditing ? (
            <div className="p-6">
              <TransactionForm
                initialData={transaction}
                onSubmit={handleUpdate}
                isSubmitting={isSubmitting}
              />
            </div>
          ) : (
            <div className="p-6">
              <div className="grid grid-cols-1 gap-y-6 gap-x-4 sm:grid-cols-6">
                <div className="sm:col-span-3">
                  <h3 className="text-lg font-medium text-gray-900">Amount</h3>
                  <p className="mt-1 text-xl font-semibold text-gray-900">
                    ${transaction.amount.toFixed(2)}
                  </p>
                </div>
                
                <div className="sm:col-span-3">
                  <h3 className="text-lg font-medium text-gray-900">Date</h3>
                  <p className="mt-1 text-sm text-gray-500">
                    {new Date(transaction.date).toLocaleDateString()}
                  </p>
                </div>
                
                <div className="sm:col-span-3">
                  <h3 className="text-lg font-medium text-gray-900">Merchant</h3>
                  <p className="mt-1 text-sm text-gray-500">{transaction.merchant}</p>
                </div>
                
                <div className="sm:col-span-3">
                  <h3 className="text-lg font-medium text-gray-900">Category</h3>
                  <p className="mt-1 text-sm text-gray-500">{transaction.category}</p>
                </div>
                
                {transaction.notes && (
                  <div className="sm:col-span-6">
                    <h3 className="text-lg font-medium text-gray-900">Notes</h3>
                    <p className="mt-1 text-sm text-gray-500">{transaction.notes}</p>
                  </div>
                )}
                
                {transaction.items && transaction.items.length > 0 && (
                  <div className="sm:col-span-6">
                    <h3 className="text-lg font-medium text-gray-900">Items</h3>
                    <div className="mt-2 border rounded-md overflow-hidden">
                      <table className="min-w-full divide-y divide-gray-200">
                        <thead className="bg-gray-50">
                          <tr>
                            <th scope="col" className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                              Item
                            </th>
                            <th scope="col" className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                              Price
                            </th>
                            <th scope="col" className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                              Quantity
                            </th>
                            <th scope="col" className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                              Total
                            </th>
                          </tr>
                        </thead>
                        <tbody className="bg-white divide-y divide-gray-200">
                          {transaction.items.map((item: TransactionItem, index: number) => (
                            <tr key={index}>
                              <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                                {item.name}
                              </td>
                              <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500 text-right">
                                {item.price ? `$${item.price.toFixed(2)}` : '-'}
                              </td>
                              <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500 text-right">
                                {item.quantity || 1}
                              </td>
                              <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900 text-right">
                                {item.price ? `$${(item.price * (item.quantity || 1)).toFixed(2)}` : '-'}
                              </td>
                            </tr>
                          ))}
                        </tbody>
                      </table>
                    </div>
                  </div>
                )}
                
                {transaction.billImage && (
                  <div className="sm:col-span-6">
                    <h3 className="text-lg font-medium text-gray-900">Receipt Image</h3>
                    <div className="mt-2">
                      <img
                        src={transaction.billImage}
                        alt="Receipt"
                        className="max-w-full h-auto border rounded-md"
                      />
                    </div>
                  </div>
                )}
                
                <div className="sm:col-span-6 border-t pt-4">
                  <div className="flex justify-between">
                    <p className="text-sm text-gray-500">Created</p>
                    <p className="text-sm text-gray-500">
                      {new Date(transaction.createdAt).toLocaleString()}
                    </p>
                  </div>
                </div>
              </div>
            </div>
          )}
        </div>
      </div>
    </Layout>
  );
};

export default TransactionDetailPage; 