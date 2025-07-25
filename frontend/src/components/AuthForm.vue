<script setup>
import { reactive } from 'vue';
import { zodResolver } from '@primevue/forms/resolvers/zod';
import { z } from 'zod';
import { useToast } from 'primevue/usetoast';
import Form from '@primevue/forms/form';
import FormField from '@primevue/forms/formfield';
import InputText from 'primevue/inputtext';
import Password from 'primevue/password';
import Button from 'primevue/button';
import Message from 'primevue/message';

const props = defineProps({
    submitLabel: {
        type: String,
        default: 'Submit'
    },
    loading: {
        type: Boolean,
        default: false
    }
});

const emit = defineEmits(['submit']);

const toast = useToast();

const resolver = zodResolver(
    z.object({
        username: z.string().min(1, { message: 'Username is required.' }),
        password: z.string().min(1, { message: 'Password is required.' })
    })
);

const onFormSubmit = ({ valid, values }) => {
    if (valid) {
        emit('submit', values);
    }
};
</script>

<template>
    <div class="auth-form-container">
        <Form :resolver="resolver" @submit="onFormSubmit" class="auth-form">
            <FormField v-slot="$field" as="section" name="username" initialValue="" class="md-form-group">
                <label for="username" class="md-form-label">Username</label>
                <InputText 
                    id="username"
                    type="text" 
                    placeholder="Enter your username" 
                    v-model="$field.value"
                    class="md-input"
                />
                <Message v-if="$field?.invalid" severity="error" size="small" variant="simple" class="field-error">
                    {{ $field.error?.message }}
                </Message>
            </FormField>
            
            <FormField v-slot="$field" asChild name="password" initialValue="">
                <section class="md-form-group">
                    <label for="password" class="md-form-label">Password</label>
                    <Password 
                        id="password"
                        type="password" 
                        placeholder="Enter your password" 
                        :feedback="false" 
                        toggleMask 
                        fluid 
                        v-model="$field.value"
                    />
                    <Message v-if="$field?.invalid" severity="error" size="small" variant="simple" class="field-error">
                        {{ $field.error?.message }}
                    </Message>
                </section>
            </FormField>
            
            <div class="form-actions">
                <Button 
                    type="submit" 
                    :label="submitLabel" 
                    :loading="loading"
                    class="md-button-primary auth-submit-btn"
                />
            </div>
        </Form>
    </div>
</template>

<style scoped>
.auth-form-container {
  background-color: var(--md-surface);
  border-radius: var(--md-shape-corner-medium);
  border: 1px solid var(--md-outline);
  padding: var(--md-spacing-lg);
  box-shadow: var(--md-elevation-1);
}

.auth-form {
    display: flex;
    flex-direction: column;
    gap: var(--md-spacing-lg);
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
    border-color: var(--md-primary);
    box-shadow: 0 0 0 2px rgba(25, 118, 210, 0.2);
}

/* Fix PrimeVue Password component styling */
:deep(.p-password) {
    width: 100%;
}

:deep(.p-password .p-inputtext) {
    width: 100%;
    padding: var(--md-spacing-md);
    border: 1px solid var(--md-outline);
    border-radius: var(--md-shape-corner-small);
    font-size: var(--md-type-scale-body1);
    background-color: var(--md-surface);
    color: var(--md-on-surface);
    transition: all var(--md-motion-duration-short2) var(--md-motion-easing-standard);
}

:deep(.p-password .p-inputtext:focus) {
    outline: none;
    border-color: var(--md-primary);
    box-shadow: 0 0 0 2px rgba(25, 118, 210, 0.2);
}

:deep(.p-password .p-input-icon-right) {
    right: var(--md-spacing-md);
    color: var(--md-on-surface-variant);
}

:deep(.p-password .p-input-icon-right:hover) {
    color: var(--md-primary);
}

.field-error {
    margin-top: var(--md-spacing-xs);
}

.form-actions {
    margin-top: var(--md-spacing-md);
    padding-top: var(--md-spacing-md);
    border-top: 1px solid var(--md-outline);
}

.auth-submit-btn {
    width: 100%;
    padding: var(--md-spacing-md) var(--md-spacing-lg);
    font-weight: 500;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    background-color: var(--md-primary) !important;
    color: var(--md-on-primary) !important;
    border: none !important;
    border-radius: var(--md-shape-corner-small) !important;
    transition: all var(--md-motion-duration-short2) var(--md-motion-easing-standard);
}

.auth-submit-btn:hover {
    background-color: var(--md-primary-light) !important;
    transform: translateY(-1px);
}

.auth-submit-btn:active {
    transform: translateY(0);
}

.auth-submit-btn:disabled {
    background-color: var(--md-surface-container-high) !important;
    color: var(--md-on-surface-variant) !important;
    transform: none;
}
</style>
