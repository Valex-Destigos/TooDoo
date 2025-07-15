<script setup lang="ts">
import { ref, watch } from 'vue'
import type { Todo, NewTodo, RepeatRule } from '../types/todo'

// Use a more flexible prop type that can accept a full Todo or a NewTodo
const props = defineProps<{
  initialData: Todo | NewTodo
}>()

const emit = defineEmits<{
  (e: 'save', data: Todo | NewTodo): void
  (e: 'cancel'): void
}>()

// Create a local, editable copy of the initial data. This is crucial.
// We should never modify a prop directly.
const editableTodo = ref({ ...props.initialData })

function clearDue() {
  // Set 'due' to undefined instead of deleting the key.
  // This is a more reliable way to signal it should be cleared.
  editableTodo.value.due = undefined
}

function addReminder() {
  if (!editableTodo.value.reminder) editableTodo.value.reminder = []
  editableTodo.value.reminder.push('')
}

function updateReminder(index: number, value: string) {
  if (editableTodo.value.reminder) {
    editableTodo.value.reminder[index] = value
  }
}

function removeReminder(index: number) {
  if (editableTodo.value.reminder) {
    editableTodo.value.reminder.splice(index, 1)
  }
}

function clearReminders() {
  editableTodo.value.reminder = []
}

const repeatOptions: RepeatRule[] = ['NEVER', 'DAILY', 'WEEKLY', 'MONTHLY', 'YEARLY']

function handleSave() {
  if (!editableTodo.value.title.trim()) {
    alert('Title cannot be empty.')
    return
  }

  // Create a clean payload to avoid modifying the reactive object directly during processing.
  const payload = { ...editableTodo.value }

  // If 'due' is present, convert it to ISO format. Otherwise, delete the key.
  if (payload.due) {
    payload.due = new Date(payload.due).toISOString()
  } else {
    delete payload.due
  }
  payload.reminder = payload.reminder
    .filter((r) => r) // Remove any empty strings
    .map((r) => new Date(r).toISOString()) // Convert valid dates to ISO

  emit('save', payload)
}

function handleCancel() {
  emit('cancel')
}

// If the initialData prop changes from the outside (e.g., when a user
// cancels an edit and then starts a new one), we reset our local state.
watch(
  () => props.initialData,
  (newData) => {
    editableTodo.value = { ...newData }
  },
)

function isoToDateInput(val?: string) {
  if (val) {
    return val.slice(0, 10)
  }
}

function dateInputToIso(val?: string) {
  if (val) {
    return new Date(val).toISOString()
  }
}
</script>

<template>
  <div class="todo-form-container">
    <div class="form-fields">
      <input
        type="text"
        class="form-title"
        v-model="editableTodo.title"
        placeholder="What needs to be done?"
      />
      <textarea
        class="form-description"
        v-model="editableTodo.description"
        placeholder="Add a description..."
        rows="3"
      ></textarea>

      <div class="form-row">
        <div class="form-group">
          <label for="due-date">Due Date</label>
          <input
            type="date"
            id="due-date"
            :value="isoToDateInput(editableTodo.due)"
            @input="(e) => (editableTodo.due = (e.target as HTMLInputElement).value)"
          />
          <button type="button" @click="clearDue" class="btn-utility">Clear Due Date</button>
        </div>
        <div class="form-group">
          <label for="repeat-rule">Repeat</label>
          <select id="repeat-rule" v-model="editableTodo.repeat">
            <option v-for="option in repeatOptions" :key="option" :value="option">
              {{ option.charAt(0) + option.slice(1).toLowerCase() }}
            </option>
          </select>
        </div>
      </div>
      <div class="form-group">
        <label>Reminders</label>
        <div v-if="editableTodo.reminder && editableTodo.reminder.length">
          <div
            v-for="(rem, idx) in editableTodo.reminder"
            :key="idx"
            style="display: flex; align-items: center; gap: 0.5rem; margin-bottom: 5pt"
          >
            <input
              type="date"
              :value="isoToDateInput(rem)"
              @input="(e) => updateReminder(idx, (e.target as HTMLInputElement).value)"
            />
            <button type="button" @click="removeReminder(idx)">Remove</button>
          </div>
        </div>
        <div class="utility-buttons" style="margin-top: -5pt">
          <button type="button" @click="addReminder" class="btn-utility">Add Reminder</button>
          <button
            type="button"
            @click="clearReminders"
            v-if="editableTodo.reminder && editableTodo.reminder.length"
            class="btn-utility"
          >
            Clear All Reminders
          </button>
        </div>
      </div>
    </div>
    <div class="form-actions">
      <button class="btn-cancel" @click="handleCancel">Cancel</button>
      <button class="btn-save" @click="handleSave">Save</button>
    </div>
  </div>
</template>

<style scoped>
.todo-form-container {
  background-color: #ffffff;
  border-radius: 12px;
  padding: 1.25rem;
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.1);
  border: 1px solid #e0e0e0;
}

.form-fields {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

input,
textarea,
select {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid #ddd;
  border-radius: 8px;
  font-size: 1rem;
  font-family: 'Inter', sans-serif;
}

.form-title {
  font-size: 1.1rem;
  font-weight: 500;
}

.form-row {
  display: flex;
  gap: 1rem;
}

.form-group {
  flex: 1;
}

.form-group label {
  display: block;
  font-size: 0.8rem;
  font-weight: 500;
  margin-bottom: 0.25rem;
  color: #555;
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.75rem;
  margin-top: 1.25rem;
  padding-top: 1rem;
  border-top: 1px solid #f0f0f0;
}

button {
  padding: 0.6rem 1.25rem;
  border: none;
  border-radius: 8px;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.2s;
}

.btn-save {
  background-color: #364d64;
  color: white;
}
.btn-save:hover {
  background-color: rgb(47, 95, 143);
}

.btn-cancel {
  background-color: #f1f1f1;
  color: #555;
}
.btn-cancel:hover {
  background-color: #e0e0e0;
}

.btn-utility {
  margin: 8px;
  margin-left: 0px;
  padding: 0.4rem 0.8rem;
  font-size: 0.8rem;
  background-color: #f0f0f0;
  color: #333;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.btn-utility:hover {
  background-color: #e0e0e0;
}
</style>
