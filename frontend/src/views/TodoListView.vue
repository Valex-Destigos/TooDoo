<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import apiClient from '@/api/axios'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import TodoList from '../components/TodoList.vue' // Import the new component
import type { Todo, NewTodo } from '../types/todo'
import TodoForm from '../components/TodoForm.vue'

const router = useRouter()
const auth = useAuthStore()

function handleLogout() {
  auth.clearToken()
  router.push('/login')
}

const todos = ref<Todo[]>([])
const sortKey = ref<'recency' | 'alphabetical' | 'due-date'>('recency')
const sortAscending = ref(false) // false = descending (newest first), true = ascending (oldest first)

// A state object for the creation form
const getBlankTodo = (): NewTodo => ({
  title: '',
  description: '',
  repeat: 'Never',
  reminder: [],
})
const newTodo = ref<NewTodo>(getBlankTodo())

// --- DATA MANAGEMENT FUNCTIONS ---

function sortTodos() {
  todos.value.sort((a, b) => {
    // Primary sort: incomplete todos first
    if (a.completed !== b.completed) {
      return Number(a.completed) - Number(b.completed)
    }
    
    let result = 0
    
    // Secondary sort: based on the current sort key
    if (sortKey.value === 'recency') {
      result = b.id - a.id // Newest (higher ID) first by default
    } else if (sortKey.value === 'alphabetical') {
      result = a.title.localeCompare(b.title) // Alphabetical by default
    } else if (sortKey.value === 'due-date') {
      // Sort by due date: items with due dates first, then items without due dates
      if (a.due && b.due) {
        // Both have due dates, sort by date (earliest first by default)
        result = new Date(a.due).getTime() - new Date(b.due).getTime()
      } else if (a.due && !b.due) {
        // a has due date, b doesn't - a comes first
        return -1
      } else if (!a.due && b.due) {
        // b has due date, a doesn't - b comes first
        return 1
      } else {
        // Neither has due date, sort by recency as fallback
        result = b.id - a.id
      }
    }
    
    // Apply sort direction (flip result if ascending for recency and due-date)
    if (sortKey.value === 'recency' || sortKey.value === 'due-date') {
      return sortAscending.value ? -result : result
    } else {
      // For alphabetical, ascending is the default, so flip if descending
      return sortAscending.value ? result : -result
    }
  })
}

function setSortKey(key: 'recency' | 'alphabetical' | 'due-date') {
  sortKey.value = key
  sortTodos()
}

function toggleSortDirection() {
  sortAscending.value = !sortAscending.value
  sortTodos()
}

async function listAllTodos() {
  try {
    const response = await apiClient.get('/todos')
    // Sort todos so incomplete ones are at the top
    todos.value = response.data
    sortTodos()
  } catch (error) {
    console.error('Failed to fetch todos:', error)
  }
}

// The create function now receives the full object from the form's 'save' event
async function createTodo(payload: NewTodo) {
  if (!payload.title.trim()) return

  // The payload from TodoForm is already
  const cleanedPayload = { ...payload }

  try {
    // Send the cleaned payload, not the original one.
    const response = await apiClient.post('/todos', cleanedPayload)
    todos.value.unshift(response.data)
    newTodo.value = getBlankTodo()
    sortTodos() // Ensure new items are sorted correctly
  } catch (error) {
    console.error('Failed to create todo:', error)
  }
}

async function updateTodo(updatedTodo: Todo) {
  try {
    // Optimistic update for a snappy UI
    const index = todos.value.findIndex((todo) => todo.id === updatedTodo.id)
    if (index !== -1) {
      todos.value[index] = updatedTodo
      sortTodos() // Re-sort the list to reflect completion change
    }
    await apiClient.put(`/todos/${updatedTodo.id}`, updatedTodo)
  } catch (error) {
    console.error(`Failed to update todo with id ${updatedTodo.id}:`, error)
    // If the API call fails, revert the change
    await listAllTodos()
  }
}

async function deleteTodo(id: number) {
  try {
    await apiClient.delete(`/todos/${id}`)
    todos.value = todos.value.filter((todo) => todo.id !== id)
  } catch (error) {
    console.error(`Failed to delete todo with id ${id}:`, error)
  }
}

// --- POLLING FOR UPDATES ---
let pollingInterval: number | undefined

// Fetch initial data when the component is mounted and set up polling
onMounted(() => {
  listAllTodos()
})
</script>

<template>
  <div id="app-container">
    <header class="app-header">
      <div class="header-content">
        <div class="header-title">
          <h1 class="md-headline-large">TooDoo</h1>
          <p class="md-body-medium">your todo list companion</p>
        </div>
        <div class="user-section">
          <div v-if="auth.user" class="welcome-message">
            <span class="md-body-large">Hey, {{ auth.user.username }}!</span>
          </div>
          <button @click="handleLogout" class="md-button-text logout-btn">Logout</button>
        </div>
      </div>
    </header>
    <main>
      <div class="form-container">
        <TodoForm :initial-data="newTodo" @save="createTodo" @cancel="newTodo = getBlankTodo()" />
      </div>

      <div class="sort-controls">
        <span class="md-body-medium">Sort by:</span>
        <button
          @click="setSortKey('recency')"
          :class="['md-button-text', { active: sortKey === 'recency' }]"
        >
          Recently Added
        </button>
        <button
          @click="setSortKey('alphabetical')"
          :class="['md-button-text', { active: sortKey === 'alphabetical' }]"
        >
          Alphabetical
        </button>
        <button
          @click="setSortKey('due-date')"
          :class="['md-button-text', { active: sortKey === 'due-date' }]"
        >
          Due Date
        </button>
        <button
          @click="toggleSortDirection"
          class="sort-direction-btn"
          :title="sortAscending ? 'Switch to descending order' : 'Switch to ascending order'"
        >
          <svg
            :class="['sort-arrow', { flipped: sortAscending }]"
            width="20"
            height="20"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <path d="M7 14l5-5 5 5"/>
          </svg>
        </button>
      </div>

      <TodoList :todos="todos" @delete-todo="deleteTodo" @update-todo="updateTodo" />
    </main>
  </div>
</template>

<style scoped>
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@300;400;500;700&display=swap');

#app-container {
  max-width: 1000px;
  margin: 0 auto;
}

.app-header {
  text-align: center;
  margin-bottom: var(--md-spacing-xl);
  padding-top: var(--md-spacing-lg);
}

.header-content {
  display: grid;
  grid-template-columns: 1fr auto 1fr;
  align-items: center;
  max-width: 1000px;
  margin: 0 auto;
  padding: 0 var(--md-spacing-md);
}

.header-title {
  grid-column: 2;
  text-align: center;
}

.user-section {
  grid-column: 3;
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: var(--md-spacing-xs);
  justify-self: end;
}

.header-title h1 {
  margin-bottom: var(--md-spacing-xs);
}

.welcome-message {
  color: var(--md-on-surface);
  font-weight: 500;
}

.welcome-message span {
  color: var(--md-primary);
}

.logout-btn {
  background: none;
  color: var(--md-primary);
  border: none;
  padding: var(--md-spacing-xs) var(--md-spacing-sm);
  border-radius: var(--md-shape-corner-small);
  cursor: pointer;
  font-size: var(--md-type-scale-body2);
  font-weight: 500;
  transition: all var(--md-motion-duration-short2) var(--md-motion-easing-standard);
  pointer-events: auto;
}

.logout-btn:hover {
  background-color: rgba(25, 118, 210, 0.08);
}

.form-container {
  margin-bottom: var(--md-spacing-lg);
}

.sort-controls {
  display: flex;
  align-items: center;
  gap: var(--md-spacing-sm);
  margin-bottom: var(--md-spacing-lg);
  padding: var(--md-spacing-sm) 0;
}

.sort-controls button.active {
  background-color: var(--md-primary);
  color: var(--md-on-primary);
}

.sort-direction-btn {
  background: none;
  border: 1px solid var(--md-outline);
  color: var(--md-on-surface);
  padding: var(--md-spacing-xs);
  border-radius: var(--md-shape-corner-small);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all var(--md-motion-duration-short2) var(--md-motion-easing-standard);
  min-width: 36px;
  height: 36px;
}

.sort-direction-btn:hover {
  background-color: rgba(25, 118, 210, 0.08);
  border-color: var(--md-primary);
}

.sort-arrow {
  transition: transform var(--md-motion-duration-medium2) var(--md-motion-easing-emphasized);
}

.sort-arrow.flipped {
  transform: rotate(180deg);
}

@media (min-width: 1024px) {
  #app-container {
    padding: 0 var(--md-spacing-lg);
  }
}

@media (max-width: 768px) {
  .header-content {
    grid-template-columns: 1fr;
    gap: var(--md-spacing-md);
    text-align: center;
  }

  .header-title {
    grid-column: 1;
  }
  
  .user-section {
    grid-column: 1;
    align-items: center;
    justify-self: center;
  }
}
</style>
