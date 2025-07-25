<template>
  <div id="app-layout">
    <!-- Only show navigation if user is not authenticated -->
    <header v-if="!auth.isAuthenticated" class="md-nav">
      <nav>
        <router-link to="/login">Login</router-link>
        <span class="nav-separator"> | </span>
        <router-link to="/register">Register</router-link>
      </nav>
    </header>

    <main>
      <router-view />
    </main>
  </div>
</template>

<script setup lang="ts">
import { useAuthStore } from './stores/auth';

const auth = useAuthStore();
</script>

<style scoped>
#app-layout {
  min-height: 100vh;
  background-color: var(--md-background);
}

nav {
  display: flex;
  align-items: center;
  gap: var(--md-spacing-xs);
}

.nav-separator {
  color: var(--md-on-surface-variant);
  margin: 0 var(--md-spacing-xs);
}
</style>