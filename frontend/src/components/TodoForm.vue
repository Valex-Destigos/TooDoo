<script setup lang="ts">
import { ref, watch } from 'vue'
import { zodResolver } from '@primevue/forms/resolvers/zod'
import { z } from 'zod'
import { useToast } from 'primevue/usetoast'
import Form from '@primevue/forms/form'
import FormField from '@primevue/forms/formfield'
import InputText from 'primevue/inputtext'
import Textarea from 'primevue/textarea'
import Select from 'primevue/select'
import Button from 'primevue/button'
import Message from 'primevue/message'
import type { Todo, NewTodo, RepeatRule } from '../types/todo'

const props = defineProps<{
  initialData: Todo | NewTodo
  isCreateMode?: boolean
}>()

const emit = defineEmits<{
  (e: 'save', data: Todo | NewTodo): void
  (e: 'cancel'): void
}>()

const toast = useToast()

const resolver = zodResolver(
  z.object({
    title: z.string().min(1, { message: 'Title is required.' }),
    description: z.string().optional(),
    due: z.string().optional(),
    repeat: z.enum(['Never', 'Daily', 'Weekly', 'Monthly', 'Yearly']),
  }),
)

const repeatOptions: RepeatRule[] = ['Never', 'Daily', 'Weekly', 'Monthly', 'Yearly']

const formKey = ref(0)

const getInitialValues = () => {
  if (props.isCreateMode && formKey.value > 0) {
    return {
      title: '',
      description: '',
      due: '',
      repeat: 'Never' as RepeatRule,
    }
  }
  return {
    title: props.initialData.title || '',
    description: props.initialData.description || '',
    due: props.initialData.due ? isoToDateTimeInput(props.initialData.due) : '',
    repeat: props.initialData.repeat || 'Never',
  }
}

const reminders = ref<string[]>([...(props.initialData.reminder || [])])

watch(
  () => props.initialData,
  (newData) => {
    reminders.value = [...(newData.reminder || [])]
  },
  { deep: true },
)

function isoToDateTimeInput(val?: string) {
  if (val) {
    const d = new Date(val)
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

function addReminder() {
  reminders.value.push('')
}

function updateReminder(index: number, value: string) {
  reminders.value[index] = value
}

function removeReminder(index: number) {
  reminders.value.splice(index, 1)
}

function clearReminders() {
  reminders.value = []
}

const onFormSubmit = ({ valid, values }: { valid: boolean; values: any }) => {
  if (!valid) {
    return
  }
  const payload: Partial<Todo> = { ...props.initialData, ...values }
  if (values.due && values.due.trim() !== '') {
    payload.due = dateTimeInputToIso(values.due)
  } else {
    delete payload.due
  }
  payload.reminder = reminders.value
    .map((r) => dateTimeInputToIso(r))
    .filter((r): r is string => typeof r === 'string')

  emit('save', payload as Todo | NewTodo)
  resetFormToBlank(values)
}

function resetFormToBlank(values: Todo | NewTodo) {
  values.title = ''
  values.description = ''
  values.due = ''
  values.repeat = 'Never' as RepeatRule
  reminders.value = []
  formKey.value++
}

function handleCancel() {
  if (props.isCreateMode) {
    reminders.value = []
    formKey.value++
  } else {
    reminders.value = [...(props.initialData.reminder || [])]
  }
  emit('cancel')
}
</script>

<template>
  <div class="md-card">
    <Form
      :key="formKey"
      :resolver="resolver"
      @submit="onFormSubmit"
      :initialValues="getInitialValues()"
    >
      <div class="md-card-content">
        <div class="form-fields">
          <FormField v-slot="$field" name="title" class="md-form-group">
            <label for="todo-title" class="md-form-label">Title</label>
            <InputText
              id="todo-title"
              placeholder="What needs to be done?"
              v-model="$field.value"
              v-bind="$field.props"
              :class="['md-input form-title', { error: $field.invalid }]"
              fluid
            />
            <Message v-if="$field.invalid" severity="error" size="small" variant="simple">
              {{ $field.error?.message }}
            </Message>
          </FormField>

          <FormField v-slot="$field" name="description" class="md-form-group">
            <label for="todo-description" class="md-form-label">Description</label>
            <Textarea
              id="todo-description"
              placeholder="Add a description..."
              v-model="$field.value"
              v-bind="$field.props"
              :class="['md-input md-textarea', { error: $field.invalid }]"
              rows="3"
              fluid
            />
            <Message v-if="$field.invalid" severity="error" size="small" variant="simple">
              {{ $field.error?.message }}
            </Message>
          </FormField>

          <div class="md-form-row">
            <FormField v-slot="$field" name="due" class="md-form-group">
              <label for="due-date" class="md-form-label">Due Date & Time</label>
              <input
                type="datetime-local"
                id="due-date"
                :class="['md-input', { error: $field.invalid }]"
                v-model="$field.value"
                v-bind="$field.props"
              />
              <Button type="button" @click="$field.value = ''" class="md-button-text" size="small">
                Clear Due Date
              </Button>
              <Message v-if="$field.invalid" severity="error" size="small" variant="simple">
                {{ $field.error?.message }}
              </Message>
            </FormField>

            <FormField v-slot="$field" name="repeat" class="md-form-group">
              <label for="repeat-rule" class="md-form-label">Repeat</label>
              <Select
                id="repeat-rule"
                v-model="$field.value"
                v-bind="$field.props"
                :options="repeatOptions"
                :class="['md-input md-select', { error: $field.invalid }]"
                fluid
              />
              <Message v-if="$field.invalid" severity="error" size="small" variant="simple">
                {{ $field.error?.message }}
              </Message>
            </FormField>
          </div>

          <div class="md-form-group">
            <label class="md-form-label">Reminders (Date & Time)</label>
            <div v-if="reminders.length">
              <div v-for="(rem, idx) in reminders" :key="idx" class="reminder-item">
                <input
                  type="datetime-local"
                  class="md-input"
                  :value="isoToDateTimeInput(rem)"
                  @input="(e) => updateReminder(idx, (e.target as HTMLInputElement).value)"
                />
                <Button
                  type="button"
                  @click="removeReminder(idx)"
                  class="md-button-secondary"
                  size="small"
                  >Remove</Button
                >
              </div>
            </div>
            <div class="utility-buttons">
              <Button type="button" @click="addReminder" class="md-button-text" size="small"
                >Add Reminder</Button
              >
              <Button
                type="button"
                @click="clearReminders"
                v-if="reminders.length"
                class="md-button-text"
                size="small"
              >
                Clear All Reminders
              </Button>
            </div>
          </div>
        </div>
      </div>

      <div class="md-card-actions">
        <Button type="button" @click="handleCancel" class="md-button-secondary">Cancel</Button>
        <Button type="submit" class="md-button-primary">Save</Button>
      </div>
    </Form>
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

.md-form-group {
  display: flex;
  flex-direction: column;
  gap: var(--md-spacing-xs);
}

.md-form-label {
  font-size: var(--md-type-scale-caption);
  font-weight: 500;
  color: var(--md-on-surface);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: var(--md-spacing-xs);
}

.md-input {
  height: 48px;
  padding: var(--md-spacing-md);
  border: 1px solid var(--md-outline);
  border-radius: var(--md-shape-corner-small);
  font-size: var(--md-type-scale-body1);
  background-color: var(--md-surface);
  color: var(--md-on-surface);
  transition: all var(--md-motion-duration-short2) var(--md-motion-easing-standard);
}

.md-input:focus {
  outline: none;
  border-color: var(--md-primary) !important;
  box-shadow: 0 0 0 2px rgba(25, 118, 210, 0.2) !important;
}

.md-input.error {
  border-color: var(--md-error) !important;
  background-color: rgba(176, 0, 32, 0.05);
}

.md-input.error:focus {
  border-color: var(--md-error) !important;
  box-shadow: 0 0 0 2px rgba(176, 0, 32, 0.2) !important;
}

.field-error {
  margin-top: var(--md-spacing-xs);
}

:deep(.p-inputtext) {
  width: 100%;
  padding: var(--md-spacing-md);
  border: 1px solid var(--md-outline) !important;
  border-radius: var(--md-shape-corner-small);
  font-size: var(--md-type-scale-body1);
  background-color: var(--md-surface);
  color: var(--md-on-surface);
  transition: all var(--md-motion-duration-short2) var(--md-motion-easing-standard);
}

:deep(.p-inputtext:focus) {
  outline: none;
  border-color: var(--md-primary) !important;
  box-shadow: 0 0 0 2px rgba(25, 118, 210, 0.2) !important;
}

:deep(.p-inputtext.error) {
  border-color: var(--md-error) !important;
  background-color: rgba(176, 0, 32, 0.05);
}

:deep(.p-inputtext.error:focus) {
  border-color: var(--md-error) !important;
  box-shadow: 0 0 0 2px rgba(176, 0, 32, 0.2) !important;
}

:deep(.p-textarea) {
  width: 100%;
  min-height: 80px;
  height: auto;
  padding: var(--md-spacing-md);
  border: 1px solid var(--md-outline) !important;
  border-radius: var(--md-shape-corner-small);
  font-size: var(--md-type-scale-body1);
  background-color: var(--md-surface);
  color: var(--md-on-surface);
  transition: all var(--md-motion-duration-short2) var(--md-motion-easing-standard);
  resize: vertical;
}

:deep(.p-textarea:focus) {
  outline: none;
  border-color: var(--md-primary) !important;
  box-shadow: 0 0 0 2px rgba(25, 118, 210, 0.2) !important;
}

:deep(.p-select) {
  position: relative;
  height: 48px;
}

:deep(.p-select .p-inputtext) {
  width: 100%;
  padding: var(--md-spacing-md);
  border: 1px solid var(--md-outline) !important;
  border-radius: var(--md-shape-corner-small);
  font-size: var(--md-type-scale-body1);
  background-color: var(--md-surface);
  color: var(--md-on-surface);
  transition: all var(--md-motion-duration-short2) var(--md-motion-easing-standard);
  display: flex;
  align-items: center;
  box-sizing: border-box;
  height: 48px;
  line-height: 1.2;
}

:deep(.p-select .p-select-label) {
  display: flex;
  align-items: center;
  height: 100%;
  line-height: 1.2;
  padding: 0;
}

:deep(.p-select .p-inputtext.error) {
  border-color: var(--md-error) !important;
  background-color: rgba(176, 0, 32, 0.05);
}

:deep(.p-select .p-inputtext.error:focus) {
  border-color: var(--md-error) !important;
  box-shadow: 0 0 0 2px rgba(176, 0, 32, 0.2) !important;
}

:deep(.p-select:not(.p-disabled).p-focus .p-inputtext) {
  border-color: var(--md-primary) !important;
  box-shadow: 0 0 0 2px rgba(25, 118, 210, 0.2) !important;
}

:deep(.p-focus),
:deep(.p-inputtext:focus),
:deep(.p-textarea:focus),
:deep(.p-select:focus),
:deep(.p-select.p-focus),
:deep(.p-select:not(.p-disabled).p-focus),
:deep(.p-component:focus),
:deep(.p-component.p-focus) {
  border-color: var(--md-primary) !important;
  box-shadow: 0 0 0 2px rgba(25, 118, 210, 0.2) !important;
  outline: none !important;
}

:deep(.p-select-overlay) {
  background-color: var(--md-surface-container);
  border: 1px solid var(--md-outline);
  border-radius: var(--md-shape-corner-small);
  box-shadow: var(--md-elevation-2);
  min-width: 100%;
}

:deep(.p-select-list) {
  padding: 0;
  margin: 0;
}

:deep(.p-select-option) {
  padding: var(--md-spacing-sm) var(--md-spacing-md);
  color: var(--md-on-surface);
  transition: background-color var(--md-motion-duration-short2) var(--md-motion-easing-standard);
  width: 100%;
  box-sizing: border-box;
  display: flex;
  align-items: center;
  min-height: 40px;
}

:deep(.p-select-option:hover) {
  background-color: var(--md-surface-container-high);
  color: var(--md-on-surface);
}

:deep(.p-select-option:focus) {
  background-color: var(--md-surface-container-high);
  color: var(--md-on-surface);
  outline: none;
  border-color: var(--md-primary) !important;
}

:deep(.p-select-option.p-select-option-selected) {
  background-color: var(--md-surface-container-high) !important;
  color: var(--md-on-surface) !important;
}

:deep(.p-select-option.p-highlight) {
  background-color: var(--md-surface-container-high) !important;
  color: var(--md-on-surface) !important;
}

:deep(.p-select-option.p-select-option-selected.p-highlight) {
  background-color: var(--md-surface-container-high) !important;
  color: var(--md-on-surface) !important;
}

:deep(.p-select-option[aria-selected='true']) {
  background-color: var(--md-surface-container-high) !important;
  color: var(--md-on-surface) !important;
}

:deep(.p-button) {
  border-radius: var(--md-shape-corner-small);
  font-weight: 500;
  text-transform: none;
  letter-spacing: 0.5px;
  transition:
    background-color var(--md-motion-duration-short2) var(--md-motion-easing-standard),
    border-color var(--md-motion-duration-short2) var(--md-motion-easing-standard);
}

:deep(.p-button.md-button-primary) {
  background-color: var(--md-primary);
  border-color: var(--md-primary);
  color: var(--md-on-primary);
}

:deep(.p-button.md-button-primary:hover) {
  background-color: var(--md-primary-light);
  border-color: var(--md-primary-light);
  color: var(--md-on-primary);
}

:deep(.p-button.md-button-secondary) {
  background-color: var(--md-surface-container);
  border-color: var(--md-outline);
  color: var(--md-on-surface);
}

:deep(.p-button.md-button-secondary:hover) {
  background-color: var(--md-surface-container-high);
  border-color: var(--md-outline-variant);
  color: var(--md-on-surface);
}

:deep(.p-button.md-button-text) {
  background-color: transparent;
  border-color: transparent;
  color: var(--md-primary);
  padding: var(--md-spacing-xs) var(--md-spacing-sm);
}

:deep(.p-button.md-button-text:hover) {
  background-color: rgba(25, 118, 210, 0.08);
  border-color: transparent;
  color: var(--md-primary);
}

.md-form-row {
  display: flex;
  gap: var(--md-spacing-md);
}

.md-form-row > .md-form-group {
  flex: 1;
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

.md-form-group #due-date {
  margin-bottom: var(--md-spacing-sm);
}

.md-form-group .md-button-text {
  margin-top: var(--md-spacing-xs);
  align-self: flex-start;
  width: auto;
}

.md-button-primary,
.md-button-secondary {
  padding: var(--md-spacing-sm) var(--md-spacing-lg);
}

.reminder-item .md-button-secondary {
  padding: var(--md-spacing-sm) var(--md-spacing-md);
}

@media (max-width: 640px) {
  .md-form-row {
    flex-direction: column !important;
    gap: var(--md-spacing-md) !important;
  }
}
</style>
