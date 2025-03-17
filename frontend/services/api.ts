import axios from 'axios';

// Configure API URL based on environment
const API_URL = process.env.NEXT_PUBLIC_API_URL || 'http://localhost:8080';

// Create axios instance
const api = axios.create({
  baseURL: API_URL,
  headers: {
    'Content-Type': 'application/json',
  },
  // Add reasonable timeout
  timeout: 10000,
});

// Add request interceptor to include auth token
api.interceptors.request.use((config) => {
  // Get token from local storage or cookie
  const token = localStorage.getItem('token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
}, (error) => {
  return Promise.reject(error);
});

// Add response interceptor for error handling
api.interceptors.response.use(
  (response) => response,
  (error) => {
    // Handle session expiry
    if (error.response && error.response.status === 401) {
      // Clear token and redirect to login
      localStorage.removeItem('token');
      if (typeof window !== 'undefined') {
        window.location.href = '/login';
      }
    }
    return Promise.reject(error);
  }
);

// Auth API
export const authAPI = {
  register: (userData: { email: string; password: string; name: string }) => 
    api.post('/api/auth/register', userData),
  login: (credentials: { email: string; password: string }) => 
    api.post('/api/auth/login', credentials),
  logout: () => {
    localStorage.removeItem('token');
    return Promise.resolve();
  },
  getCurrentUser: () => api.get('/api/auth/me'),
};

// OCR API
export const ocrAPI = {
  processImage: (formData: FormData) => 
    api.post('/api/ocr/process', formData, {
      headers: {
        'Content-Type': 'multipart/form-data',
      },
    }),
};

// Transactions API
export const transactionsAPI = {
  getTransactions: (filters = {}) => 
    api.get('/api/transactions', { params: filters }),
  getTransaction: (id: string) => 
    api.get(`/api/transactions/${id}`),
  createTransaction: (transactionData: any) => 
    api.post('/api/transactions', transactionData),
  updateTransaction: (id: string, transactionData: any) => 
    api.put(`/api/transactions/${id}`, transactionData),
  deleteTransaction: (id: string) => 
    api.delete(`/api/transactions/${id}`),
};

// Categories API
export const categoriesAPI = {
  getCategories: () => api.get('/api/categories'),
  getCategory: (id: string) => api.get(`/api/categories/${id}`),
  createCategory: (categoryData: any) => api.post('/api/categories', categoryData),
  updateCategory: (id: string, categoryData: any) => api.put(`/api/categories/${id}`, categoryData),
  deleteCategory: (id: string) => api.delete(`/api/categories/${id}`),
};

// Users API (for admin)
export const usersAPI = {
  getUsers: (filters = {}) => api.get('/api/users', { params: filters }),
  getUser: (id: string) => api.get(`/api/users/${id}`),
  createUser: (userData: any) => api.post('/api/users', userData),
  updateUser: (id: string, userData: any) => api.put(`/api/users/${id}`, userData),
  deleteUser: (id: string) => api.delete(`/api/users/${id}`),
};

// Reports API
export const reportsAPI = {
  getSpendingByCategory: (dateRange: { startDate: string; endDate: string }) => 
    api.get('/api/reports/spending-by-category', { params: dateRange }),
  getMonthlySpending: (year: number) => 
    api.get('/api/reports/monthly-spending', { params: { year } }),
  getTransactionTrends: (period: string) => 
    api.get('/api/reports/transaction-trends', { params: { period } }),
};

export default api; 