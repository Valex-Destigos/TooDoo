import axios from 'axios';
import { useAuthStore } from '../stores/auth';

const apiClient = axios.create({
  baseURL: '/api', // Your API's base URL
  headers: {
    'Content-Type': 'application/json',
  },
});

// --- This is the Interceptor ---
apiClient.interceptors.request.use(
  (config) => {
    const authStore = useAuthStore();
    const token = authStore.token; // Get the token from your Pinia store

    if (token) {
      // If the token exists, add the Authorization header
      config.headers.Authorization = `Bearer ${token}`;
    }
    
    return config;
  },
  (error) => {
    // Handle request error
    return Promise.reject(error);
  }
);

export default apiClient;