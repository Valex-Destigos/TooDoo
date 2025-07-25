import { createRouter, createWebHistory } from 'vue-router';
import TodoListView from '../views/TodoListView.vue';
import LoginView from '../views/LoginView.vue';
import RegisterView from '../views/RegisterView.vue';
import { useAuthStore } from '../stores/auth';

declare module 'vue-router' {
  interface RouteMeta {
    requiresAuth?: boolean;
    requiresGuest?: boolean;
  }
}

const routes = [
  {
    path: '/',
    name: 'TodoList',
    component: TodoListView,
    meta: { requiresAuth: true },
  },
  {
    path: '/login',
    name: 'Login',
    component: LoginView,
    meta: { requiresGuest: true },
  },
  {
    path: '/register',
    name: 'Register',
    component: RegisterView,
    meta: { requiresGuest: true },
  },
];

const router = createRouter({
  history: createWebHistory(), // Uses the browser's history API for clean URLs
  routes, // a shorthand for `routes: routes`
});

// Navigation guards
router.beforeEach((to, from, next) => {
  const auth = useAuthStore();
  
  if (to.meta.requiresAuth && !auth.isAuthenticated) {
    // Redirect to login if trying to access protected route without auth
    next('/login');
  } else if (to.meta.requiresGuest && auth.isAuthenticated) {
    // Redirect to home if trying to access guest-only routes while authenticated
    next('/');
  } else {
    next();
  }
});

export default router;