import { defineStore } from 'pinia';
import { ref, computed } from 'vue';

export interface User {
    id: number;
    username: string;
}

export const useAuthStore = defineStore('auth', () => {
    // --- STATE ---
    const token = ref<string | null>(localStorage.getItem('jwt_token'));
    const user = ref<User | null>(JSON.parse(localStorage.getItem('user') || 'null'));

    // --- GETTERS ---
    const isAuthenticated = computed(() => !!token.value);

    // --- ACTIONS ---
    function setToken(newToken: string) {
        token.value = newToken;
        localStorage.setItem('jwt_token', newToken);
    }

    function setUser(newUser: User) {
        user.value = newUser;
        localStorage.setItem('user', JSON.stringify(newUser));
    }

    function clearToken() {
        token.value = null;
        user.value = null;
        localStorage.removeItem('jwt_token');
        localStorage.removeItem('user');
    }

    // --- RETURN ---
    return {
        token,
        user,
        isAuthenticated,
        setToken,
        setUser,
        clearToken,
    };
});