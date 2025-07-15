<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import axios from 'axios'
import TodoList from './components/TodoList.vue' // Import the new component
import type { Todo, NewTodo } from './types/todo'
import TodoForm from './components/TodoForm.vue'

// Use a relative path for the API host when using a proxy.
const host = '/api'
const todos = ref<Todo[]>([])
const sortKey = ref<'recency' | 'alphabetical'>('recency')

// A state object for the creation form
const getBlankTodo = (): NewTodo => ({
  title: '',
  description: '',
  repeat: 'NEVER',
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
    // Secondary sort: based on the current sort key
    if (sortKey.value === 'recency') {
      return b.id - a.id // Newest (higher ID) first
    } else {
      return a.title.localeCompare(b.title) // Alphabetical
    }
  })
}

function setSortKey(key: 'recency' | 'alphabetical') {
  sortKey.value = key
  sortTodos()
}

async function listAllTodos() {
  try {
    const response = await axios.get(`${host}/todos`)
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
    const response = await axios.post(`${host}/todos`, cleanedPayload)
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
    await axios.put(`${host}/todos/${updatedTodo.id}`, updatedTodo)
  } catch (error) {
    console.error(`Failed to update todo with id ${updatedTodo.id}:`, error)
    // If the API call fails, revert the change
    await listAllTodos()
  }
}

async function deleteTodo(id: number) {
  try {
    await axios.delete(`${host}/todos/${id}`)
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
    <header>
      <h1>TooDoo</h1>
      <span>your todo list companion</span>
    </header>
    <main>
      <div class="form-container">
        <TodoForm :initial-data="newTodo" @save="createTodo" @cancel="newTodo = getBlankTodo()" />
      </div>

      <div class="sort-controls">
        <span>Sort by:</span>
        <button @click="setSortKey('recency')" :class="{ active: sortKey === 'recency' }">
          Recency
        </button>
        <button @click="setSortKey('alphabetical')" :class="{ active: sortKey === 'alphabetical' }">
          Alphabetical
        </button>
      </div>

      <TodoList :todos="todos" @delete-todo="deleteTodo" @update-todo="updateTodo" />
    </main>
  </div>
</template>

<style scoped>
/* Using a modern, clean font */
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;700&display=swap');

#app-container {
  max-width: 1000px;
  margin: 0 auto;
  font-family: 'Inter', sans-serif;
  color: #333;
}

header {
  text-align: center;
  margin-bottom: 2.5rem;
}

header h1 {
  font-size: 3.5rem;
  font-weight: 700;
  margin: 0;
  color: #364d64;
}
header span {
  color: #7f8c8d;
}

.form-container {
  margin-bottom: 1.5rem;
}

.sort-controls {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin-bottom: 1.5rem;
  font-size: 0.9rem;
  color: #7f8c8d;
}

.sort-controls button {
  background: none;
  border: 1px solid #dee2e6;
  padding: 0.25rem 0.75rem;
  border-radius: 1rem;
  cursor: pointer;
  transition: all 0.2s ease;
  color: #7f8c8d;
}

.sort-controls button.active {
  background-color: #ffffff;
  border-color: #ffffff;
  font-weight: 500;
}

.sort-controls button:hover:not(.active) {
  border-color: #7f8c8d;
}

.todo-input {
  flex-grow: 1;
  padding: 0.75rem 1rem;
  border: 1px solid #ddd;
  border-radius: 8px;
  font-size: 1rem;
  transition:
    border-color 0.2s,
    box-shadow 0.2s;
}

.todo-input:focus {
  outline: none;
  border-color: #3498db;
  box-shadow: 0 0 0 3px rgba(52, 152, 219, 0.2);
}

.add-button {
  padding: 0.75rem 1.5rem;
  border: none;
  background-color: #3498db;
  color: white;
  font-weight: 500;
  border-radius: 8px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.add-button:hover {
  background-color: #2980b9;
}

@media (min-width: 1024px) {
  #app {
    padding: 0 2rem;
  }
}
</style>
