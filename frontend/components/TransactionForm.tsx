import React, { useEffect, useState } from 'react';
import { useCategories } from '@/hooks/useCategories';
import CategorySelector from '@/components/CategorySelector';

interface TransactionFormProps {
  initialData?: {
    amount?: number;
    date?: string;
    merchant?: string;
    category?: string;
    notes?: string;
    items?: Array<{name: string; price?: number; quantity?: number}>;
    billImage?: string;
  };
  onSubmit: (data: any) => void;
  isSubmitting?: boolean;
}

const TransactionForm: React.FC<TransactionFormProps> = ({ 
  initialData = {}, 
  onSubmit, 
  isSubmitting = false 
}) => {
  const { categories, isLoading: isLoadingCategories } = useCategories();
  
  const [formData, setFormData] = useState({
    amount: initialData.amount || 0,
    date: initialData.date || new Date().toISOString().split('T')[0],
    merchant: initialData.merchant || '',
    category: initialData.category || '',
    notes: initialData.notes || '',
    items: initialData.items || [],
    billImage: initialData.billImage || '',
  });

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>) => {
    const { name, value } = e.target;
    setFormData(prev => ({ ...prev, [name]: value }));
  };

  const handleCategoryChange = (category: string) => {
    setFormData(prev => ({ ...prev, category }));
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSubmit({
      ...formData,
      amount: parseFloat(formData.amount.toString()),
    });
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      <div className="grid grid-cols-1 gap-6 md:grid-cols-2">
        <div>
          <label className="block text-sm font-medium text-gray-700">Amount</label>
          <div className="mt-1 relative rounded-md shadow-sm">
            <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
              <span className="text-gray-500 sm:text-sm">$</span>
            </div>
            <input
              type="number"
              name="amount"
              value={formData.amount}
              onChange={handleChange}
              required
              step="0.01"
              min="0"
              className="focus:ring-blue-500 focus:border-blue-500 block w-full pl-7 pr-12 sm:text-sm border-gray-300 rounded-md"
            />
          </div>
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700">Date</label>
          <input
            type="date"
            name="date"
            value={formData.date}
            onChange={handleChange}
            required
            className="mt-1 focus:ring-blue-500 focus:border-blue-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md"
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700">Merchant</label>
          <input
            type="text"
            name="merchant"
            value={formData.merchant}
            onChange={handleChange}
            required
            className="mt-1 focus:ring-blue-500 focus:border-blue-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md"
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700">Category</label>
          <CategorySelector 
            value={formData.category} 
            onChange={handleCategoryChange} 
            categories={categories || []} 
            isLoading={isLoadingCategories}
          />
        </div>
      </div>

      <div>
        <label className="block text-sm font-medium text-gray-700">Notes</label>
        <textarea
          name="notes"
          value={formData.notes}
          onChange={handleChange}
          rows={3}
          className="mt-1 focus:ring-blue-500 focus:border-blue-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md"
        />
      </div>

      {formData.items.length > 0 && (
        <div>
          <h3 className="text-lg font-medium text-gray-900">Items</h3>
          <ul className="mt-3 divide-y divide-gray-200 border-t border-b">
            {formData.items.map((item, index) => (
              <li key={index} className="py-3 flex justify-between">
                <div>
                  <p className="text-sm font-medium">{item.name}</p>
                  {item.quantity && <p className="text-sm text-gray-500">Qty: {item.quantity}</p>}
                </div>
                {item.price && <p className="text-sm font-medium">${item.price.toFixed(2)}</p>}
              </li>
            ))}
          </ul>
        </div>
      )}

      <div className="flex justify-end">
        <button
          type="submit"
          disabled={isSubmitting}
          className={`inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500
            ${isSubmitting ? 'opacity-75 cursor-not-allowed' : ''}`}
        >
          {isSubmitting ? 'Saving...' : 'Save Transaction'}
        </button>
      </div>
    </form>
  );
};

export default TransactionForm; 