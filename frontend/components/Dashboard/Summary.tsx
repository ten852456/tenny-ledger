import React from 'react';
import { useTransactions } from '@/hooks/useTransactions';

interface Transaction {
  id: string;
  amount: number;
  date: string;
  merchant: string;
  category: string;
  createdAt: string;
}

interface CategorySummary {
  category: string;
  amount: number;
}

const Summary: React.FC = () => {
  const today = new Date();
  const startOfMonth = new Date(today.getFullYear(), today.getMonth(), 1).toISOString().split('T')[0];
  const endOfMonth = new Date(today.getFullYear(), today.getMonth() + 1, 0).toISOString().split('T')[0];
  
  const { transactions, isLoading, error } = useTransactions({
    startDate: startOfMonth,
    endDate: endOfMonth,
    limit: 100, // Get all transactions for the month
  });
  
  if (isLoading) {
    return (
      <div className="bg-white shadow rounded-lg p-6">
        <h2 className="text-lg font-medium text-gray-900 mb-4">Monthly Summary</h2>
        <div className="animate-pulse space-y-4">
          <div className="grid grid-cols-1 sm:grid-cols-3 gap-5">
            <div className="h-24 bg-gray-200 rounded"></div>
            <div className="h-24 bg-gray-200 rounded"></div>
            <div className="h-24 bg-gray-200 rounded"></div>
          </div>
        </div>
      </div>
    );
  }
  
  if (error) {
    return (
      <div className="bg-white shadow rounded-lg p-6">
        <h2 className="text-lg font-medium text-gray-900 mb-4">Monthly Summary</h2>
        <div className="py-4 text-center text-red-500">
          Error loading summary data
        </div>
      </div>
    );
  }
  
  // Calculate summary metrics
  const totalSpent = transactions?.transactions.reduce((sum: number, t: Transaction) => sum + t.amount, 0) || 0;
  const avgPerDay = totalSpent / (today.getDate());
  const remainingDays = new Date(today.getFullYear(), today.getMonth() + 1, 0).getDate() - today.getDate();
  const projectedTotal = totalSpent + (avgPerDay * remainingDays);
  
  // Group by category
  const categorySummary: Record<string, number> = {};
  
  transactions?.transactions.forEach((t: Transaction) => {
    if (!categorySummary[t.category]) {
      categorySummary[t.category] = 0;
    }
    categorySummary[t.category] += t.amount;
  });
  
  // Find top categories
  const topCategories: CategorySummary[] = Object.entries(categorySummary)
    .map(([category, amount]): CategorySummary => ({ category, amount }))
    .sort((a, b) => b.amount - a.amount)
    .slice(0, 3);
  
  return (
    <div className="bg-white shadow rounded-lg p-6">
      <h2 className="text-lg font-medium text-gray-900 mb-4">Monthly Summary</h2>
      
      <div className="grid grid-cols-1 sm:grid-cols-3 gap-5">
        <div className="bg-blue-50 rounded-lg p-4">
          <h3 className="text-sm font-medium text-blue-700">Month-to-Date Spent</h3>
          <p className="mt-2 text-3xl font-semibold text-blue-900">${totalSpent.toFixed(2)}</p>
          <p className="mt-1 text-sm text-blue-500">
            {transactions?.transactions.length || 0} transactions
          </p>
        </div>
        
        <div className="bg-green-50 rounded-lg p-4">
          <h3 className="text-sm font-medium text-green-700">Daily Average</h3>
          <p className="mt-2 text-3xl font-semibold text-green-900">${avgPerDay.toFixed(2)}</p>
          <p className="mt-1 text-sm text-green-500">
            per day this month
          </p>
        </div>
        
        <div className="bg-purple-50 rounded-lg p-4">
          <h3 className="text-sm font-medium text-purple-700">Projected Total</h3>
          <p className="mt-2 text-3xl font-semibold text-purple-900">${projectedTotal.toFixed(2)}</p>
          <p className="mt-1 text-sm text-purple-500">
            {remainingDays} days remaining
          </p>
        </div>
      </div>
      
      {topCategories.length > 0 && (
        <div className="mt-6">
          <h3 className="text-sm font-medium text-gray-700 mb-3">Top Spending Categories</h3>
          <div className="space-y-2">
            {topCategories.map(({ category, amount }) => (
              <div key={category} className="flex items-center">
                <div className="w-32 sm:w-40 pr-4">
                  <span className="text-sm font-medium text-gray-900">{category}</span>
                </div>
                <div className="flex-1">
                  <div className="relative h-4 overflow-hidden rounded bg-gray-200">
                    <div 
                      className="h-full bg-blue-600 rounded"
                      style={{ width: `${Math.min(100, (amount / totalSpent) * 100)}%` }}
                    ></div>
                  </div>
                </div>
                <div className="w-20 pl-4 text-right">
                  <span className="text-sm font-medium text-gray-900">${amount.toFixed(2)}</span>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
};

export default Summary; 