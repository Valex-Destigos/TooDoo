<script setup lang="ts">
import { ref } from 'vue'
import TodoForm from './TodoForm.vue'
import type { Todo, NewTodo } from '../types/todo'
import '../App.vue'

const props = defineProps<{
  todo: Todo
}>()

const emit = defineEmits<{
  (e: 'delete', id: number): void
  (e: 'update', todo: Todo): void
}>()

const isExpanded = ref(false)

function toggleDetails() {
  isExpanded.value = !isExpanded.value
}

const isEditing = ref(false)

function handleUpdate(updatedData: Todo | NewTodo) {
  emit('update', updatedData as Todo)
  isEditing.value = false
}

function toggleComplete() {
  const updatedTodo = { ...props.todo, completed: !props.todo.completed }
  emit('update', updatedTodo)
}

function handleDelete() {
  emit('delete', props.todo.id)
}

function beginEdit() {
  isEditing.value = true
}

function cancelEdit() {
  isEditing.value = false
}

function isoToDateTimeDisplay(date: string) {
  const d = new Date(date)
  // remove the seconds with a regex
  return d.toLocaleString().replace(/:\d{2}\b/, '')
}
</script>

<template>
  <li class="todo-item-wrapper">
    <TodoForm v-if="isEditing" :initial-data="todo" @save="handleUpdate" @cancel="cancelEdit" />
    <div v-else class="md-list-item" :class="{ 'md-completed': todo.completed }">
      <div class="md-list-item-content" @click.stop="toggleDetails">
        <!-- Custom Checkbox -->
        <div
          class="md-checkbox"
          :class="{ checked: todo.completed }"
          @click.stop="toggleComplete"
          role="button"
          :aria-checked="todo.completed"
        >
          <svg viewBox="0 0 24 24">
            <polyline class="checkmark" points="20 6 9 17 4 12" />
          </svg>
        </div>

        <!-- Title & Expander Click Area -->
        <div class="content-wrapper">
          <span class="md-title-large">{{ todo.title }}</span>
        </div>

        <!-- Quick Actions -->
        <div class="todo-actions">
          <button class="md-icon-button" @click.stop="beginEdit">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            >
              <path d="M17 3a2.828 2.828 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5L17 3z"></path>
            </svg>
          </button>
          <button class="md-icon-button delete" @click.stop="handleDelete">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            >
              <polyline points="3 6 5 6 21 6"></polyline>
              <path
                d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"
              ></path>
              <line x1="10" y1="11" x2="10" y2="17"></line>
              <line x1="14" y1="11" x2="14" y2="17"></line>
            </svg>
          </button>
          <div class="expand-icon" :class="{ 'is-expanded': isExpanded }">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
              <polyline points="6 9 12 15 18 9"></polyline>
            </svg>
          </div>
        </div>
      </div>

      <!-- Expandable Details Section -->
      <Transition name="expand">
        <div v-show="isExpanded" class="todo-details-wrapper">
          <div class="todo-details">
            <div class="description">
              <h4 class="md-label-large">Description:</h4>
              <p class="md-body-large">{{ todo.description || 'No description provided.' }}</p>
            </div>
            <div class="meta-info">
              <span v-if="todo.due" class="meta-item md-body-medium"
                ><strong>Due:</strong> {{ isoToDateTimeDisplay(todo.due) }}</span
              >
              <span v-if="todo.repeat != 'Never'" class="meta-item md-body-medium"
                ><strong>Repeats:</strong> {{ todo.repeat }}</span
              >
            </div>
            <div class="meta-info">
              <span v-if="todo.reminder.length > 0" class="meta-item md-body-medium">
                <strong>Reminders:</strong>
                <div v-for="reminder in todo.reminder" :key="reminder" class="reminder-date">
                  {{ isoToDateTimeDisplay(reminder) }}
                </div>
              </span>
            </div>
          </div>
        </div>
      </Transition>
    </div>
  </li>
</template>

<style scoped>
.todo-item-wrapper {
  margin-bottom: var(--md-spacing-sm);
}

.content-wrapper {
  flex-grow: 1;
  cursor: pointer;
  overflow: hidden;
}

.todo-actions {
  display: flex;
  align-items: center;
  gap: var(--md-spacing-xs);
  padding-left: var(--md-spacing-md);
}

/* Expander Icon */
.expand-icon {
  margin-left: var(--md-spacing-xs);
  color: var(--md-on-surface-variant);
  transition: transform var(--md-motion-duration-medium1) var(--md-motion-easing-standard);
}

.expand-icon.is-expanded {
  transform: rotate(180deg);
}

.expand-icon svg {
  width: 24px;
  height: 24px;
  stroke: currentColor;
  stroke-width: 2;
  fill: none;
}

/* Details Section */
.todo-details-wrapper {
  overflow: hidden;
}

.todo-details {
  padding: 0 var(--md-spacing-md) var(--md-spacing-md) calc(var(--md-spacing-md) + 40px); /* Aligns with title */
  border-top: 1px solid var(--md-outline);
  color: var(--md-on-surface-variant);
}

.description {
  margin: var(--md-spacing-md) 0;
}

.description p {
  white-space: pre-line;
  margin-top: var(--md-spacing-xs);
}

.meta-info {
  display: flex;
  gap: var(--md-spacing-lg);
  margin-top: var(--md-spacing-sm);
}

.meta-item strong {
  color: var(--md-on-surface);
}

.reminder-date {
  margin-top: var(--md-spacing-xs);
  margin-left: var(--md-spacing-sm);
}

.md-checkbox {
  width: 24px;
  height: 24px;
  border: 2px solid var(--md-outline);
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  background: transparent;
  outline: none;
}

.md-checkbox:focus,
.md-checkbox:active,
.md-checkbox:hover {
  background: transparent;
  outline: none;
}

.md-checkbox.checked {
  background: transparent;
}

.checkmark {
  fill: none;
  stroke: var(--md-primary);
  stroke-width: 4;
  stroke-linecap: round;
  stroke-linejoin: round;
  stroke-dasharray: 24;
  stroke-dashoffset: 24;
  transition: stroke-dashoffset 0.3s ease;
}

.md-checkbox.checked .checkmark {
  stroke-dashoffset: 0;
}
</style>
