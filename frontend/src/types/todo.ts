export type RepeatRule = 'Never' | 'Daily' | 'Weekly' | 'Monthly' | 'Yearly';

export interface Todo {
    id: number,
    title: string,
    description: string,
    due?: string,
    reminder: string[],
    repeat: RepeatRule,
    completed: boolean,
}

export interface NewTodo {
    title: string,
    description: string,
    due?: string,
    reminder: string[],
    repeat: RepeatRule,
}