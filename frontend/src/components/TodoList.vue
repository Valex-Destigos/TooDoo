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
    <div v-else class="md-empty-state">
      <h3 class="md-headline-small">All clear! No todos yet.</h3>
      <p class="md-body-medium">Add one above to get started.</p>
    </div>
  </div>
</template>

<style scoped>
.todo-list ul {
  list-style: none;
  padding: 0;
}
</style>
