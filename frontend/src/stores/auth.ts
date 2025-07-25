import { defineStore } from 'pinia';
import { ref, computed } from 'vue';

export interface User {
    id: number;
    username: string;
}

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

export const useAuthStore = defineStore('auth', () => {
    const storedToken = localStorage.getItem('jwt_token');
    
    // Check if stored token is expired before using it
    const token = ref<string | null>(
        storedToken && !isTokenExpired(storedToken) ? storedToken : null
    );
    
    // If token was expired, clear it from localStorage
    if (storedToken && isTokenExpired(storedToken)) {
        localStorage.removeItem('jwt_token');
        localStorage.removeItem('jwt_expires_at');
        localStorage.removeItem('user');
    }
    
    const user = ref<User | null>(
        token.value ? JSON.parse(localStorage.getItem('user') || 'null') : null
    );

    const isAuthenticated = computed(() => {
        if (!token.value) return false;
        
        // Double-check token expiration in real-time
        if (isTokenExpired(token.value)) {
            clearToken();
            return false;
        }
        
        return true;
    });

    function setToken(newToken: string) {
        // Verify the new token is not expired before setting it
        if (isTokenExpired(newToken)) {
            console.warn('Attempted to set an expired token');
            return;
        }
        
        // Decode token to get expiration for debugging
        try {
            const payload = JSON.parse(atob(newToken.split('.')[1]));
            const expirationDate = new Date(payload.exp * 1000);
            
            // Store token and expiration info
            token.value = newToken;
            localStorage.setItem('jwt_token', newToken);
            localStorage.setItem('jwt_expires_at', expirationDate.toISOString());
            
            console.log(`Token set, expires at: ${expirationDate.toLocaleString()}`);
        } catch (error) {
            console.warn('Could not decode token expiration');
            token.value = newToken;
            localStorage.setItem('jwt_token', newToken);
        }
    }

    function setUser(newUser: User) {
        user.value = newUser;
        localStorage.setItem('user', JSON.stringify(newUser));
    }

    function clearToken() {
        token.value = null;
        user.value = null;
        localStorage.removeItem('jwt_token');
        localStorage.removeItem('jwt_expires_at');
        localStorage.removeItem('user');
    }

    function checkTokenExpiration() {
        if (token.value && isTokenExpired(token.value)) {
            clearToken();
            return false;
        }
        return true;
    }

    return {
        token,
        user,
        isAuthenticated,
        setToken,
        setUser,
        clearToken,
        checkTokenExpiration,
    };
});