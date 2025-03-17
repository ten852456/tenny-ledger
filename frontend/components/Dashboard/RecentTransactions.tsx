import React from 'react';
import { useTransactions } from '@/hooks/useTransactions';
import Link from 'next/link';
import { useRouter } from 'next/router';

interface Transaction {
  id: string;
  amount: number;
  date: string;
  merchant: string;
  category: string;
  createdAt: string;
}

const RecentTransactions: React.FC = () => {
  const router = useRouter();
  const { transactions, isLoading, error } = useTransactions({
    limit: 5,
    sortBy: 'date',
    sortDirection: 'desc',
  });
  
  if (isLoading) {
    return (
      <div className="bg-white shadow rounded-lg p-6">
        <h2 className="text-lg font-medium text-gray-900 mb-4">Recent Transactions</h2>
        <div className="animate-pulse space-y-4">
          <div className="h-10 bg-gray-200 rounded"></div>
          <div className="h-10 bg-gray-200 rounded"></div>
          <div className="h-10 bg-gray-200 rounded"></div>
          <div className="h-10 bg-gray-200 rounded"></div>
          <div className="h-10 bg-gray-200 rounded"></div>
        </div>
      </div>
    );
  }
  
  if (error) {
    return (
      <div className="bg-white shadow rounded-lg p-6">
        <h2 className="text-lg font-medium text-gray-900 mb-4">Recent Transactions</h2>
        <div className="py-4 text-center text-red-500">
          Error loading transactions
        </div>
      </div>
    );
  }
  
  if (!transactions || transactions.transactions.length === 0) {
    return (
      <div className="bg-white shadow rounded-lg p-6">
        <h2 className="text-lg font-medium text-gray-900 mb-4">Recent Transactions</h2>
        <div className="py-8 text-center text-gray-500">
          <p className="mb-4">No transactions found</p>
          <Link 
            href="/upload"
            className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700"
          >
            Upload First Receipt
          </Link>
        </div>
      </div>
    );
  }
  
  return (
    <div className="bg-white shadow rounded-lg p-6">
      <div className="flex justify-between items-center mb-4">
        <h2 className="text-lg font-medium text-gray-900">Recent Transactions</h2>
        <Link 
          href="/transactions"
          className="text-sm text-blue-600 hover:text-blue-900"
        >
          View All
        </Link>
      </div>
      
      <div className="space-y-4">
        {transactions.transactions.map((transaction: Transaction) => (
          <div 
            key={transaction.id} 
            className="border border-gray-200 rounded-lg p-4 hover:bg-gray-50 cursor-pointer transition-colors"
            onClick={() => router.push(`/transactions/${transaction.id}`)}
          >
            <div className="flex justify-between items-start mb-1">
              <div className="font-medium text-gray-900">{transaction.merchant}</div>
              <div className="font-medium text-gray-900">${transaction.amount.toFixed(2)}</div>
            </div>
            <div className="flex justify-between items-center">
              <div className="flex items-center">
                <span className="px-2 py-1 text-xs leading-none font-semibold rounded-full bg-blue-100 text-blue-800">
                  {transaction.category}
                </span>
                <span className="ml-2 text-xs text-gray-500">
                  {new Date(transaction.date).toLocaleDateString()}
                </span>
              </div>
              <button 
                onClick={(e) => {
                  e.stopPropagation();
                  router.push(`/transactions/${transaction.id}/edit`);
                }}
                className="text-sm text-blue-600 hover:text-blue-900"
              >
                Edit
              </button>
            </div>
          </div>
        ))}
      </div>
      
      <div className="mt-6 text-center">
        <Link 
          href="/transactions/new"
          className="inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50"
        >
          Add Transaction
        </Link>
      </div>
    </div>
  );
};

export default RecentTransactions; 