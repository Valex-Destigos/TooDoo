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

function isoToDateInput(date: string) {
  return date.slice(0, 10)
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
    <div v-else class="todo-item" :class="{ 'is-completed': todo.completed }">
      <div class="todo-item-main" @click.stop="toggleDetails">
        <!-- Custom Checkbox -->
        <div
          class="checkbox-wrapper"
          @click.stop="toggleComplete"
          role="button"
          :aria-checked="todo.completed"
        >
          <span class="checkmark">
            <svg viewBox="0 0 24 24"><polyline points="20 6 9 17 4 12" /></svg>
          </span>
        </div>

        <!-- Title & Expander Click Area -->
        <div class="content-wrapper">
          <span class="todo-title">{{ todo.title }}</span>
        </div>

        <!-- Quick Actions -->
        <div class="todo-actions">
          <button class="action-btn" @click.stop="beginEdit">
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
          <button class="action-btn" @click.stop="handleDelete">
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
              <h3><strong>Description:</strong></h3>
              <p>{{ todo.description || 'No description provided.' }}</p>
            </div>
            <div class="meta-info">
              <span v-if="todo.due" class="meta-item"
                ><strong>Due:</strong> {{ isoToDateTimeDisplay(todo.due) }}</span
              >
              <span v-if="todo.repeat != 'Never'" class="meta-item"
                ><strong>Repeats:</strong> {{ todo.repeat }}</span
              >
            </div>
            <div class="meta-info">
              <span v-if="todo.reminder.length > 0" class="meta-item">
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
  margin-bottom: 1rem;
}

.todo-item {
  background-color: #ffffff;
  border-radius: 12px;
  margin-bottom: 1rem;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.06);
  transition: all 0.2s ease-in-out;
  overflow: hidden;
}
.todo-item:hover {
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.1);
  transform: translateY(-2px);
}
.is-completed {
  opacity: 0.6;
  background-color: #f8f9fa;
}

.todo-item-main {
  display: flex;
  align-items: center;
  padding: 1rem 1.25rem;
}

/* Custom Checkbox */
.checkbox-wrapper {
  position: relative;
  height: 24px;
  width: 24px;
  margin-right: 1rem;
  flex-shrink: 0;
  cursor: pointer;
}
.checkmark {
  height: 100%;
  width: 100%;
  background-color: #e9ecef;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  transition:
    background-color 0.2s ease,
    transform 0.2s ease;
}
.checkbox-wrapper:hover .checkmark {
  transform: scale(1.1);
}
.checkmark svg {
  width: 14px;
  height: 14px;
  stroke: white;
  stroke-width: 3;
  fill: none;
  stroke-dasharray: 20;
  stroke-dashoffset: 20;
  transition: stroke-dashoffset 0.3s cubic-bezier(0.45, 0, 0.55, 1);
}
.is-completed .checkmark {
  background-color: #28a745; /* Green for completed */
}
.is-completed .checkmark svg {
  stroke-dashoffset: 0;
}

/* Content */
.content-wrapper {
  flex-grow: 1;
  cursor: pointer;
  overflow: hidden;
}
.todo-title {
  font-size: 1rem;
  font-weight: 500;
  color: #343a40;
  transition: color 0.2s ease;
}
.is-completed .todo-title {
  text-decoration: line-through;
  color: #6c757d;
}

/* Actions */
.todo-actions {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding-left: 1rem;
}
.action-btn {
  background: none;
  border: none;
  cursor: pointer;
  color: #adb5bd;
  padding: 4px;
  border-radius: 50%;
  opacity: 0;
  transform: scale(0.8);
  transition: all 0.2s ease;
}
.action-btn svg {
  width: 20px;
  height: 20px;
}
.todo-item:hover .action-btn {
  opacity: 1;
  transform: scale(1);
}
.action-btn:hover {
  background-color: #e9ecef;
  color: #343a40;
}
.action-btn:last-of-type:hover {
  color: #e74c3c;
}

/* Expander Icon */
.expand-icon {
  margin-left: 0.5rem;
  color: #ced4da;
  transition: transform 0.3s ease;
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
  padding: 0 1.25rem 1.25rem 3.75rem; /* Aligns with title */
  border-top: 1px solid #e9ecef;
  color: #6c757d;
}
.description {
  margin: 1rem 0;
  font-size: 0.9rem;
  line-height: 1.6;
}
.description p {
  white-space: pre-line;
}
.meta-info {
  display: flex;
  gap: 1.5rem;
  font-size: 0.8rem;
  text-transform: capitalize;
  margin-top: 10px;
}
.meta-item strong {
  color: #495057;
}

/* Expansion Transition */
.expand-enter-active,
.expand-leave-active {
  transition: all 0.6s ease-in-out;
}
.expand-enter-from,
.expand-leave-to {
  opacity: 0;
  transform: translateY(-10px);
  max-height: 0;
}
.expand-enter-to,
.expand-leave-from {
  max-height: 500px;
  opacity: 1;
}
</style>
