<template>
  <div id="app-container">
    <header class="app-header">
      <h1 class="md-headline-large">TooDoo</h1>
      <p class="md-body-medium">your todo list companion</p>
    </header>

    <main>
      <div class="form-container">
        <div class="auth-welcome">
          <h2 class="md-headline-medium">Welcome Back</h2>
          <p class="md-body-medium">Please sign in to access your todos</p>
        </div>

        <AuthForm submit-label="Login" :loading="loading" @submit="handleLogin" />

        <div v-if="errorMessage" class="error-message md-body-medium">
          {{ errorMessage }}
        </div>

        <div class="auth-footer">
          <p class="md-body-medium">
            Don't have an account?
            <router-link to="/register" class="auth-link">Create one here</router-link>
          </p>
        </div>
      </div>
    </main>

    <!-- Toast component for notifications -->
    <Toast />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import apiClient from '@/api/axios'
import { useRouter } from 'vue-router'
import { useAuthStore, type User } from '../stores/auth'
import { useToast } from 'primevue/usetoast'
import Toast from 'primevue/toast'
import AuthForm from '../components/AuthForm.vue'

const errorMessage = ref('')
const loading = ref(false)

const router = useRouter()
const auth = useAuthStore()
const toast = useToast()

async function handleLogin(formData: { username: string; password: string }) {
  errorMessage.value = ''
  loading.value = true

  try {
    const payload = {
      username: formData.username,
      password: formData.password,
    }

    const response = await apiClient.post('/users/login', payload)

    // Store the token and redirect to todos
    auth.setToken(response.data.token)

    // Store user info (we need to get it from the backend)
    // For now, we'll store just the username since that's what we have
    auth.setUser({ id: 0, username: formData.username } as User)

    toast.add({
      severity: 'success',
      summary: 'Welcome Back!',
      detail: `Welcome back, ${formData.username}!`,
      life: 3000,
    })

    router.push('/')
  } catch (error: any) {
    if (error.response?.data?.error) {
      errorMessage.value = error.response.data.error
      toast.add({
        severity: 'error',
        summary: 'Login Failed',
        detail: error.response.data.error,
        life: 5000,
      })
    } else if (error.response?.data?.message) {
      errorMessage.value = error.response.data.message
      toast.add({
        severity: 'error',
        summary: 'Login Failed',
        detail: error.response.data.message,
        life: 5000,
      })
    } else if (error.message) {
      errorMessage.value = error.message
      toast.add({
        severity: 'error',
        summary: 'Login Failed',
        detail: error.message,
        life: 5000,
      })
    } else {
      errorMessage.value = 'Login failed. Please try again.'
      toast.add({
        severity: 'error',
        summary: 'Login Failed',
        detail: 'Login failed. Please try again.',
        life: 5000,
      })
    }
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@300;400;500;700&display=swap');

#app-container {
  max-width: 1000px;
  margin: 0 auto;
}

.app-header {
  text-align: center;
  margin-bottom: var(--md-spacing-xl);
}

.app-header h1 {
  margin-bottom: var(--md-spacing-xs);
}

.form-container {
  max-width: 500px;
  margin: 0 auto;
}

.auth-welcome {
  text-align: center;
  margin-bottom: var(--md-spacing-lg);
}

.auth-welcome h2 {
  margin-bottom: var(--md-spacing-xs);
  color: var(--md-on-surface);
}

.auth-welcome p {
  color: var(--md-on-surface-variant);
}

.error-message {
  margin-top: var(--md-spacing-md);
  padding: var(--md-spacing-sm) var(--md-spacing-md);
  background-color: rgba(176, 0, 32, 0.1);
  border: 1px solid var(--md-error);
  border-radius: var(--md-shape-corner-small);
  text-align: center;
  color: var(--md-error);
}

.auth-footer {
  text-align: center;
  margin-top: var(--md-spacing-lg);
  padding-top: var(--md-spacing-md);
  border-top: 1px solid var(--md-outline);
}

.auth-footer p {
  color: var(--md-on-surface-variant);
  margin: 0;
}

.auth-link {
  color: var(--md-primary);
  text-decoration: none;
  font-weight: 500;
  transition: color var(--md-motion-duration-short2) var(--md-motion-easing-standard);
}

.auth-link:hover {
  color: var(--md-primary-light);
  text-decoration: underline;
}

@media (min-width: 1024px) {
  #app-container {
    padding: 0 var(--md-spacing-lg);
  }
}
</style>
