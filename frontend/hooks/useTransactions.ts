import useSWR, { KeyedMutator } from 'swr';
import { transactionsAPI } from '@/services/api';

export interface Transaction {
  id: string;
  amount: number;
  date: string;
  merchant: string;
  category: string;
  notes?: string;
  items?: Array<{name: string; price?: number; quantity?: number}>;
  billImage?: string;
  createdAt: string;
}

export interface TransactionsResponse {
  transactions: Transaction[];
  total: number;
  page: number;
  pages: number;
}

export const useTransactions = (filters = {}) => {
  const { data, error, mutate } = useSWR<TransactionsResponse>(
    ['transactions', filters],
    async () => {
      try {
        const response = await transactionsAPI.getTransactions(filters);
        return response.data;
      } catch (error) {
        console.error('Error fetching transactions:', error);
        throw error;
      }
    },
    {
      revalidateOnFocus: false,
      dedupingInterval: 10000, // 10 seconds
    }
  );

  return {
    transactions: data,
    isLoading: !error && !data,
    error,
    mutate,
  };
};

export const useTransaction = (id: string) => {
  const { data, error, mutate } = useSWR<Transaction>(
    id ? `transaction-${id}` : null,
    async () => {
      try {
        const response = await transactionsAPI.getTransaction(id);
        return response.data;
      } catch (error) {
        console.error(`Error fetching transaction ${id}:`, error);
        throw error;
      }
    },
    {
      revalidateOnFocus: false,
      dedupingInterval: 10000, // 10 seconds
    }
  );

  return {
    transaction: data,
    isLoading: !error && !data,
    error,
    mutate,
  };
}; 