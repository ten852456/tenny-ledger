import React, { useState } from 'react';
import Layout from '@/components/Layout';
import { useTransactions } from '@/hooks/useTransactions';
import Link from 'next/link';

// Define transaction interface
interface Transaction {
  id: string;
  amount: number;
  date: string;
  merchant: string;
  category: string;
  createdAt: string;
}

interface TransactionsResponse {
  transactions: Transaction[];
  total: number;
  page: number;
  pages: number;
}

const ReportsPage: React.FC = () => {
  const [dateRange, setDateRange] = useState({
    startDate: new Date(new Date().setMonth(new Date().getMonth() - 1)).toISOString().split('T')[0],
    endDate: new Date().toISOString().split('T')[0],
  });

  const { transactions, isLoading, error } = useTransactions({
    startDate: dateRange.startDate,
    endDate: dateRange.endDate,
  });

  const handleDateChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value } = e.target;
    setDateRange(prev => ({ ...prev, [name]: value }));
  };

  // Calculate totals by category
  const categorySummary = React.useMemo(() => {
    if (!transactions?.transactions) return [];
    
    const categories: Record<string, number> = {};
    
    transactions.transactions.forEach((transaction: Transaction) => {
      if (!categories[transaction.category]) {
        categories[transaction.category] = 0;
      }
      categories[transaction.category] += transaction.amount;
    });
    
    return Object.entries(categories)
      .map(([category, amount]) => ({ category, amount }))
      .sort((a, b) => b.amount - a.amount);
  }, [transactions]);

  return (
    <Layout title="Reports">
      <div className="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
        <h1 className="text-2xl font-semibold text-gray-900">Reports & Analytics</h1>
        
        <div className="mt-6 bg-white shadow rounded-lg p-6">
          <h2 className="text-lg font-medium text-gray-900 mb-4">Filter by Date Range</h2>
          <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 mb-6">
            <div>
              <label htmlFor="startDate" className="block text-sm font-medium text-gray-700">Start Date</label>
              <input
                type="date"
                id="startDate"
                name="startDate"
                value={dateRange.startDate}
                onChange={handleDateChange}
                className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
              />
            </div>
            <div>
              <label htmlFor="endDate" className="block text-sm font-medium text-gray-700">End Date</label>
              <input
                type="date"
                id="endDate"
                name="endDate"
                value={dateRange.endDate}
                onChange={handleDateChange}
                className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
              />
            </div>
          </div>
          
          {isLoading ? (
            <div className="animate-pulse space-y-4">
              <div className="h-10 bg-gray-200 rounded"></div>
              <div className="h-40 bg-gray-200 rounded"></div>
            </div>
          ) : error ? (
            <div className="text-center py-4 text-red-500">
              <p>Error loading transaction data</p>
            </div>
          ) : !transactions || transactions.transactions.length === 0 ? (
            <div className="text-center py-8">
              <p className="text-gray-500 mb-4">No transactions found for the selected date range</p>
              <Link 
                href="/upload"
                className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700"
              >
                Upload a Receipt
              </Link>
            </div>
          ) : (
            <>
              <div className="mb-8">
                <h3 className="text-lg font-medium text-gray-900 mb-4">Spending by Category</h3>
                <div className="overflow-hidden rounded-lg border border-gray-200">
                  <table className="min-w-full divide-y divide-gray-200">
                    <thead className="bg-gray-50">
                      <tr>
                        <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                          Category
                        </th>
                        <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                          Amount
                        </th>
                        <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                          % of Total
                        </th>
                      </tr>
                    </thead>
                    <tbody className="bg-white divide-y divide-gray-200">
                      {categorySummary.map(({ category, amount }) => (
                        <tr key={category} className="hover:bg-gray-50">
                          <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                            {category}
                          </td>
                          <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500 text-right">
                            ${amount.toFixed(2)}
                          </td>
                          <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500 text-right">
                            {(amount / transactions.transactions.reduce((sum: number, t: Transaction) => sum + t.amount, 0) * 100).toFixed(1)}%
                          </td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              </div>
              
              <div>
                <h3 className="text-lg font-medium text-gray-900 mb-4">Summary</h3>
                <div className="grid grid-cols-1 gap-5 sm:grid-cols-3">
                  <div className="bg-blue-50 p-4 rounded-lg">
                    <h4 className="text-sm font-medium text-blue-700">Total Spent</h4>
                    <p className="mt-2 text-3xl font-semibold text-blue-900">
                      ${transactions.transactions.reduce((sum: number, t: Transaction) => sum + t.amount, 0).toFixed(2)}
                    </p>
                    <p className="mt-1 text-sm text-blue-500">
                      {transactions.transactions.length} transactions
                    </p>
                  </div>
                  <div className="bg-green-50 p-4 rounded-lg">
                    <h4 className="text-sm font-medium text-green-700">Average Transaction</h4>
                    <p className="mt-2 text-3xl font-semibold text-green-900">
                      ${(transactions.transactions.reduce((sum: number, t: Transaction) => sum + t.amount, 0) / transactions.transactions.length).toFixed(2)}
                    </p>
                    <p className="mt-1 text-sm text-green-500">per transaction</p>
                  </div>
                  <div className="bg-purple-50 p-4 rounded-lg">
                    <h4 className="text-sm font-medium text-purple-700">Top Category</h4>
                    <p className="mt-2 text-3xl font-semibold text-purple-900">
                      {categorySummary[0]?.category || 'None'}
                    </p>
                    <p className="mt-1 text-sm text-purple-500">
                      ${categorySummary[0]?.amount.toFixed(2) || '0.00'}
                    </p>
                  </div>
                </div>
              </div>
            </>
          )}
        </div>
      </div>
    </Layout>
  );
};

export default ReportsPage; 