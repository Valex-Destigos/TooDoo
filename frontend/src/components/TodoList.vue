<script setup lang="ts">
import TodoItem from './TodoItem.vue'
import type { Todo } from '../types/todo'

// Define the props this component accepts
defineProps<{
  todos: Todo[]
}>()

// Define the events this component can emit upwards
const emit = defineEmits<{
  (e: 'delete-todo', id: number): void
  (e: 'update-todo', todo: Todo): void
}>()

// Forward the events from TodoItem to the parent (App.vue)
function handleDelete(id: number) {
  emit('delete-todo', id)
}

function handleUpdate(todo: Todo) {
  emit('update-todo', todo)
}
</script>

<template>
  <div class="todo-list">
    <TransitionGroup name="list" tag="ul" v-if="todos.length > 0">
      <TodoItem
        v-for="todo in todos"
        :key="todo.id"
        :todo="todo"
        @delete="handleDelete"
        @update="handleUpdate"
      />
    </TransitionGroup>
    <div v-else class="empty-state">
      <p>All clear! No todos yet.</p>
      <span>Add one above to get started.</span>
    </div>
  </div>
</template>

<style scoped>
.todo-list ul {
  list-style: none;
  padding: 0;
}

.empty-state {
  text-align: center;
  background-color: #f9f9f9;
  border: 1px dashed #ddd;
  border-radius: 8px;
  padding: 3rem 2rem;
  margin-top: 1rem;
  color: #777;
}

.empty-state p {
  font-weight: 500;
  font-size: 1.1rem;
  margin: 0 0 0.5rem 0;
}

/* Vue's TransitionGroup styles for smooth list animations */
.list-move,
.list-enter-active,
.list-leave-active {
  transition: all 0.5s cubic-bezier(0.55, 0, 0.1, 1);
}
.list-enter-from,
.list-leave-to {
  opacity: 0;
  transform: scaleY(0.01) translate(30px, 0);
}
.list-leave-active {
  position: absolute;
}
</style>
