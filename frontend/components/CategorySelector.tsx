import React from 'react';

interface Category {
  id: string;
  name: string;
  color?: string;
}

interface CategorySelectorProps {
  value: string;
  onChange: (category: string) => void;
  categories: Category[];
  isLoading?: boolean;
}

const CategorySelector: React.FC<CategorySelectorProps> = ({ 
  value, 
  onChange, 
  categories, 
  isLoading = false 
}) => {
  return (
    <div>
      {isLoading ? (
        <div className="mt-1 animate-pulse h-10 bg-gray-200 rounded-md"></div>
      ) : (
        <select
          value={value}
          onChange={(e) => onChange(e.target.value)}
          className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
        >
          <option value="">Select a category</option>
          {categories.map((category) => (
            <option key={category.id} value={category.name}>
              {category.name}
            </option>
          ))}
          {categories.length === 0 && (
            <option disabled value="">No categories available</option>
          )}
        </select>
      )}
    </div>
  );
};

export default CategorySelector; 