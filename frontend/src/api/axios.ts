import axios from 'axios';
import { useAuthStore } from '../stores/auth';
import router from '../router';

// Helper function to decode JWT and check if it's expired
function isTokenExpired(token: string): boolean {
  try {
    const payload = JSON.parse(atob(token.split('.')[1]));
    const currentTime = Math.floor(Date.now() / 1000);
    return payload.exp < currentTime;
  } catch (error) {
    // If we can't decode the token, consider it expired
    return true;
  }
}

const apiClient = axios.create({
  baseURL: '/api',
  headers: {
    'Content-Type': 'application/json',
  },
});

// --- Request Interceptor ---
apiClient.interceptors.request.use(
  (config) => {
    const authStore = useAuthStore();
    const token = authStore.token;

    if (token) {
      // Check if token is expired before using it
      if (isTokenExpired(token)) {
        // Token is expired, clear it and redirect to login
        authStore.clearToken();
        router.push('/login');
        return Promise.reject(new Error('Token expired'));
      }
      
      // If the token exists and is valid, add the Authorization header
      config.headers.Authorization = `Bearer ${token}`;
    }
    
    return config;
  },
  (error) => {
    // Handle request error
    return Promise.reject(error);
  }
);

// --- Response Interceptor for handling expired tokens ---
apiClient.interceptors.response.use(
  (response) => {
    // If response is successful, just return it
    return response;
  },
  (error) => {
    // Handle response errors
    if (error.response?.status === 401) {
      // Token is expired or invalid
      const authStore = useAuthStore();
      authStore.clearToken(); // Clear the expired token
      router.push('/login'); // Redirect to login page
    }
    
    return Promise.reject(error);
  }
);

export default apiClient;