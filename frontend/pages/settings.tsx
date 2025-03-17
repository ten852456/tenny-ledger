import React, { useState, useEffect } from 'react';
import Layout from '@/components/Layout';

// Custom hook for categories since it's not exported from useTransactions
function useCategories() {
  const [categories, setCategories] = useState<string[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    // Mock categories for development
    const mockCategories = [
      'Food', 'Transportation', 'Entertainment', 'Shopping', 
      'Utilities', 'Healthcare', 'Housing', 'Other'
    ];
    
    // Simulate API call
    setTimeout(() => {
      setCategories(mockCategories);
      setIsLoading(false);
    }, 500);
    
    return () => {
      // Cleanup if needed
    };
  }, []);

  return { categories, isLoading, error };
}

const SettingsPage: React.FC = () => {
  const { categories, isLoading: categoriesLoading } = useCategories();
  const [newCategory, setNewCategory] = useState('');
  const [editMode, setEditMode] = useState(false);
  const [editableCategories, setEditableCategories] = useState<string[]>([]);
  
  // Mock function to handle saving settings
  const handleSaveSettings = (e: React.FormEvent) => {
    e.preventDefault();
    // Here we would send the settings to the API
    alert('Settings saved successfully!');
  };
  
  // Handle adding a new category
  const handleAddCategory = () => {
    if (!newCategory.trim()) return;
    
    // In a real app, we would make an API call here
    setEditableCategories([...editableCategories, newCategory]);
    setNewCategory('');
  };
  
  // Handle removing a category
  const handleRemoveCategory = (category: string) => {
    setEditableCategories(editableCategories.filter(c => c !== category));
  };
  
  // Toggle edit mode for categories
  const handleEditCategories = () => {
    if (!editMode) {
      // Initialize editable categories from current categories
      setEditableCategories([...(categories || [])]);
    }
    setEditMode(!editMode);
  };
  
  // Save category changes
  const handleSaveCategories = () => {
    // In a real app, we would make an API call here
    setEditMode(false);
    alert('Categories updated successfully!');
  };
  
  return (
    <Layout title="Settings">
      <div className="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
        <h1 className="text-2xl font-semibold text-gray-900 mb-8">Settings</h1>
        
        <div className="bg-white shadow rounded-lg overflow-hidden">
          <div className="p-6">
            <h2 className="text-lg font-medium text-gray-900 mb-4">Account Settings</h2>
            
            <form onSubmit={handleSaveSettings}>
              <div className="space-y-6">
                <div>
                  <label htmlFor="name" className="block text-sm font-medium text-gray-700">Name</label>
                  <input
                    type="text"
                    id="name"
                    name="name"
                    defaultValue="User Name"
                    className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                  />
                </div>
                
                <div>
                  <label htmlFor="email" className="block text-sm font-medium text-gray-700">Email Address</label>
                  <input
                    type="email"
                    id="email"
                    name="email"
                    defaultValue="user@example.com"
                    className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                  />
                </div>
                
                <div>
                  <label htmlFor="currency" className="block text-sm font-medium text-gray-700">Currency</label>
                  <select
                    id="currency"
                    name="currency"
                    defaultValue="USD"
                    className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                  >
                    <option value="USD">US Dollar ($)</option>
                    <option value="EUR">Euro (€)</option>
                    <option value="GBP">British Pound (£)</option>
                    <option value="JPY">Japanese Yen (¥)</option>
                    <option value="CAD">Canadian Dollar ($)</option>
                    <option value="AUD">Australian Dollar ($)</option>
                  </select>
                </div>
                
                <div>
                  <label htmlFor="dateFormat" className="block text-sm font-medium text-gray-700">Date Format</label>
                  <select
                    id="dateFormat"
                    name="dateFormat"
                    defaultValue="MM/DD/YYYY"
                    className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                  >
                    <option value="MM/DD/YYYY">MM/DD/YYYY</option>
                    <option value="DD/MM/YYYY">DD/MM/YYYY</option>
                    <option value="YYYY-MM-DD">YYYY-MM-DD</option>
                  </select>
                </div>
                
                <div className="flex items-center">
                  <input
                    id="emailNotifications"
                    name="emailNotifications"
                    type="checkbox"
                    defaultChecked
                    className="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                  />
                  <label htmlFor="emailNotifications" className="ml-2 block text-sm text-gray-700">
                    Receive email notifications for new receipts and reports
                  </label>
                </div>
              </div>
              
              <div className="mt-6">
                <button
                  type="submit"
                  className="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                >
                  Save Settings
                </button>
              </div>
            </form>
          </div>
          
          <div className="bg-gray-50 p-6 border-t border-gray-200">
            <div className="flex justify-between items-center mb-4">
              <h2 className="text-lg font-medium text-gray-900">Transaction Categories</h2>
              {editMode ? (
                <div className="space-x-2">
                  <button
                    onClick={handleSaveCategories}
                    className="inline-flex items-center px-3 py-1 border border-transparent text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700"
                  >
                    Save
                  </button>
                  <button
                    onClick={() => setEditMode(false)}
                    className="inline-flex items-center px-3 py-1 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50"
                  >
                    Cancel
                  </button>
                </div>
              ) : (
                <button
                  onClick={handleEditCategories}
                  className="inline-flex items-center px-3 py-1 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50"
                >
                  Edit Categories
                </button>
              )}
            </div>
            
            {categoriesLoading ? (
              <div className="animate-pulse space-y-2">
                <div className="h-8 bg-gray-200 rounded"></div>
                <div className="h-8 bg-gray-200 rounded"></div>
                <div className="h-8 bg-gray-200 rounded"></div>
              </div>
            ) : editMode ? (
              <div className="space-y-3">
                <div className="flex">
                  <input
                    type="text"
                    value={newCategory}
                    onChange={(e) => setNewCategory(e.target.value)}
                    placeholder="Add new category"
                    className="flex-1 rounded-l-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                  />
                  <button
                    onClick={handleAddCategory}
                    className="inline-flex items-center px-3 py-2 border border-l-0 border-gray-300 text-sm font-medium rounded-r-md text-gray-700 bg-gray-50 hover:bg-gray-100"
                  >
                    Add
                  </button>
                </div>
                
                <ul className="mt-3 divide-y divide-gray-200">
                  {editableCategories.map((category) => (
                    <li key={category} className="py-2 flex justify-between items-center">
                      <span className="text-sm text-gray-800">{category}</span>
                      <button
                        onClick={() => handleRemoveCategory(category)}
                        className="text-sm text-red-600 hover:text-red-900"
                      >
                        Remove
                      </button>
                    </li>
                  ))}
                </ul>
              </div>
            ) : (
              <ul className="mt-2 divide-y divide-gray-200">
                {categories?.map((category) => (
                  <li key={category} className="py-2">
                    <span className="text-sm text-gray-800">{category}</span>
                  </li>
                ))}
              </ul>
            )}
          </div>
          
          <div className="bg-gray-50 p-6 border-t border-gray-200">
            <h2 className="text-lg font-medium text-gray-900 mb-4">Data Management</h2>
            <div className="space-y-4">
              <div>
                <button
                  onClick={() => confirm('Are you sure you want to export all your data? This will download a CSV file.') && console.log('Exporting data...')}
                  className="inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50"
                >
                  Export Data (CSV)
                </button>
                <p className="mt-1 text-xs text-gray-500">
                  Download all your transaction data as a CSV file
                </p>
              </div>
              
              <div>
                <button
                  onClick={() => confirm('WARNING: This will permanently delete all your transaction history. This action cannot be undone. Are you sure?') && console.log('Deleting all data...')}
                  className="inline-flex items-center px-4 py-2 border border-red-300 text-sm font-medium rounded-md text-red-700 bg-white hover:bg-red-50"
                >
                  Clear All Data
                </button>
                <p className="mt-1 text-xs text-gray-500">
                  Permanently delete all your transactions and receipt data
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </Layout>
  );
};

export default SettingsPage; 