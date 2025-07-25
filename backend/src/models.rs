use super::schema::{users, todos, reminders}; // Use `super` to refer to parent module
use chrono::NaiveDateTime;
use serde::Serialize;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable, Identifiable, Associations)]
#[diesel(table_name = users)]
#[diesel(belongs_to(User, foreign_key = id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct InsertableUser {
    pub username: String,
    pub password: String,
}


#[derive(Queryable, Selectable, Identifiable, Associations, Serialize, Clone)]
#[diesel(table_name = todos)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Todo {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: String,
    pub due: Option<NaiveDateTime>,
    pub repeat: String,
    pub completed: bool,
}


#[derive(Queryable, Selectable, Identifiable, Associations, Serialize, Clone)]
#[diesel(table_name = reminders)]
#[diesel(belongs_to(Todo))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Reminder {
    pub id: i32,
    pub todo_id: i32,
    pub reminder: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = reminders)]
pub struct InsertableReminder {
    pub todo_id: i32,
    pub reminder: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = todos)]
pub struct InsertableTodo {
    pub user_id: i32,
    pub title: String,
    pub description: String,
    pub due: Option<chrono::NaiveDateTime>,
    pub repeat: String,
    pub completed: bool,
}