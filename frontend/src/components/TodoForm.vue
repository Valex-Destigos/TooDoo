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

const repeatOptions: RepeatRule[] = ['Never', 'Daily', 'Weekly', 'Monthly', 'Yearly']

function handleSave() {
  if (!editableTodo.value.title.trim()) {
    alert('Title cannot be empty.')
    return
  }

  const payload = { ...editableTodo.value }

  // If 'due' is present, convert it to ISO format. Otherwise, delete the key.
  if (payload.due) {
    payload.due = dateTimeInputToIso(payload.due)
  } else {
    delete payload.due
  }
  payload.reminder = payload.reminder
    .map((r) => dateTimeInputToIso(r))
    .filter((r): r is string => typeof r === 'string')

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

function isoToDateTimeInput(val?: string) {
  // Converts ISO string to 'YYYY-MM-DDTHH:mm' for datetime-local input
  if (val) {
    const d = new Date(val)
    // Pad month, day, hours, minutes
    const pad = (n: number) => n.toString().padStart(2, '0')
    return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}T${pad(d.getHours())}:${pad(d.getMinutes())}`
  }
  return ''
}

function dateTimeInputToIso(val?: string) {
  if (val) {
    return new Date(val).toISOString()
  }
  return undefined
}
</script>

<template>
  <div class="md-card">
    <div class="md-card-content">
      <div class="form-fields">
        <input
          type="text"
          class="md-input form-title"
          v-model="editableTodo.title"
          placeholder="What needs to be done?"
        />
        <textarea
          class="md-input md-textarea"
          v-model="editableTodo.description"
          placeholder="Add a description..."
          rows="3"
        ></textarea>
        <div class="md-form-row">
          <div class="md-form-group">
            <label for="due-date" class="md-form-label">Due Date & Time</label>
            <input
              type="datetime-local"
              id="due-date"
              class="md-input"
              :value="isoToDateTimeInput(editableTodo.due)"
              @input="(e) => (editableTodo.due = (e.target as HTMLInputElement).value)"
            />
            <button type="button" @click="clearDue" class="md-button-text">Clear Due Date</button>
          </div>
          <div class="md-form-group">
            <label for="repeat-rule" class="md-form-label">Repeat</label>
            <select id="repeat-rule" v-model="editableTodo.repeat" class="md-input md-select">
              <option v-for="option in repeatOptions" :key="option" :value="option">
                {{ option }}
              </option>
            </select>
          </div>
        </div>
        <div class="md-form-group">
          <label class="md-form-label">Reminders (Date & Time)</label>
          <div v-if="editableTodo.reminder && editableTodo.reminder.length">
            <div
              v-for="(rem, idx) in editableTodo.reminder"
              :key="idx"
              class="reminder-item"
            >
              <input
                type="datetime-local"
                class="md-input"
                :value="isoToDateTimeInput(rem)"
                @input="(e) => updateReminder(idx, (e.target as HTMLInputElement).value)"
              />
              <button type="button" @click="removeReminder(idx)" class="md-button-secondary">Remove</button>
            </div>
          </div>
          <div class="utility-buttons">
            <button type="button" @click="addReminder" class="md-button-text">Add Reminder</button>
            <button
              type="button"
              @click="clearReminders"
              v-if="editableTodo.reminder && editableTodo.reminder.length"
              class="md-button-text"
            >
              Clear All Reminders
            </button>
          </div>
        </div>
      </div>
    </div>
    <div class="md-card-actions">
      <button class="md-button-secondary" @click="handleCancel">Cancel</button>
      <button class="md-button-primary" @click="handleSave">Save</button>
    </div>
  </div>
</template>

<style scoped>
.form-fields {
  display: flex;
  flex-direction: column;
  gap: var(--md-spacing-md);
}

.form-title {
  font-size: var(--md-type-scale-subtitle1);
  font-weight: 500;
}

.reminder-item {
  display: flex;
  align-items: center;
  gap: var(--md-spacing-sm);
  margin-bottom: var(--md-spacing-sm);
}

.utility-buttons {
  display: flex;
  gap: var(--md-spacing-sm);
  margin-top: var(--md-spacing-xs);
}

/* Due Date Section Spacing */
.md-form-group #due-date {
  margin-bottom: var(--md-spacing-sm);
}

.md-form-group .md-button-text {
  margin-top: var(--md-spacing-xs);
}

/* Bigger buttons */
.md-button-primary,
.md-button-secondary {
  padding: var(--md-spacing-sm) var(--md-spacing-lg);
}

.reminder-item .md-button-secondary {
  padding: var(--md-spacing-sm) var(--md-spacing-md);
}

/* Mobile form layout adjustments */
@media (max-width: 640px) {
  .md-form-row {
    flex-direction: column !important;
    gap: var(--md-spacing-md) !important;
  }
}
</style>
