import useSWR from 'swr';
import { categoriesAPI } from '@/services/api';

export interface Category {
  id: string;
  name: string;
  description?: string;
  color?: string;
  icon?: string;
}

export const useCategories = () => {
  const { data, error, mutate } = useSWR<Category[]>(
    'categories',
    async () => {
      try {
        const response = await categoriesAPI.getCategories();
        // The API returns the categories array directly, not wrapped in a categories property
        return response.data;
      } catch (error) {
        console.error('Error fetching categories:', error);
        throw error;
      }
    },
    {
      revalidateOnFocus: false,
      dedupingInterval: 30000, // 30 seconds - categories don't change often
    }
  );

  return {
    categories: data,
    isLoading: !error && !data,
    error,
    mutate,
  };
}; 