<script setup lang="ts">
import { ref, onMounted } from 'vue'
import axios from 'axios'
import type { RepeatRule, Todo, NewTodo } from './types/Todo'

const host = 'http://92.211.61.96:8000/'
const todos = ref<Todo[]>([])
const title = ref('')

async function createTodo() {
  let payload: NewTodo = {
    title: title.value,
    description: '',
    repeat: 'NEVER',
  }
  title.value = ''
  let response = await axios.post(host + 'api/todos', payload)
  console.log(response.data)
  todos.value.push(response.data)
}

async function listAllTodos() {
  let response = await axios.get(host + 'api/todos')
  console.log(response.data)
  todos.value = response.data
}

onMounted(listAllTodos)
</script>

<template>
  <header>
    <h1>TooDoo - your checklist companion</h1>
  </header>
  <main>
    <div class="todo-form">
      <form v-on:submit.prevent="createTodo" class="form-box">
        <input type="text" v-model="title" />
        <button type="submit">Add Todo</button>
      </form>
    </div>
    <div class="request-button">
      <button @click="listAllTodos">Print all todos to console</button>
    </div>
    <div class="todo-list">
      <ul v-if="todos.length > 0">
        <li v-for="todo in todos" :key="todo.id">
          {{ todo.title }}
        </li>
      </ul>
      <p v-else>No todos yet. Add one above!</p>
    </div>
  </main>
</template>

<style scoped>
header {
  line-height: 1.5;
}

main {
  top: 500pt;
}

.todo-form {
  place-items: center;
}

.form-box {
  position: absolute;
}

.request-button {
  top: 30pt;
  place-self: center;
  position: relative;
}

.todo-list {
  top: 100pt;
  place-self: left;
  position: relative;
}

@media (min-width: 1024px) {
  header {
    display: flex;
    place-items: center;
    padding-right: calc(var(--section-gap) / 2);
  }

  header .wrapper {
    display: flex;
    place-items: flex-start;
    flex-wrap: wrap;
  }
}
</style>
