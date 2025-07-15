#[macro_use]
extern crate rocket;

use chrono::{DateTime, Utc};
use rocket::{
    http::{ContentType, Status},
    response::{self, Responder, Response},
    serde::json::{json, Json},
    Request, State,
};
use rocket_cors::CorsOptions;
use std::env;
use std::io::Cursor;

use diesel::r2d2::{self, ConnectionManager, Pool};
use diesel::{prelude::*, result::Error as DieselError};
use diesel::BelongingToDsl;
use dotenvy::dotenv;

mod models;
mod schema;
mod todo;

// --- Model and Type Definitions ---
type DbInsertableReminder = models::InsertableReminder;
type DbReminder = models::Reminder;
type DbTodo = models::Todo;
type ApiTodo = todo::Todo;
type NewTodo = todo::NewTodo;
type RepeatRule = todo::RepeatRule;
type DbPool = Pool<ConnectionManager<SqliteConnection>>;

// --- Custom Error Handling ---

// Define a custom error type for our application
#[derive(Debug)]
enum CustomError {
    DatabaseError(DieselError),
    NotFound,
}

// Implement the Responder trait for our custom error
#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for CustomError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let (status, error_json) = match self {
            // Specifically handle the case where an item is not found in the DB
            CustomError::DatabaseError(DieselError::NotFound) | CustomError::NotFound => (
                Status::NotFound,
                json!({"error": "The requested resource was not found."}),
            ),
            // Handle all other database errors generically
            CustomError::DatabaseError(e) => {
                // It's good practice to log the detailed error for debugging
                eprintln!("A database error occurred: {:?}", e);
                (
                    Status::InternalServerError,
                    json!({"error": "An unexpected error occurred. Please try again later."}),
                )
            }
        };

        // Build the response
        Response::build()
            .status(status)
            .header(ContentType::JSON)
            .sized_body(error_json.to_string().len(), Cursor::new(error_json.to_string()))
            .ok()
    }
}

// Allow easy conversion from a Diesel error into our custom error using `?`
impl From<DieselError> for CustomError {
    fn from(error: DieselError) -> Self {
        CustomError::DatabaseError(error)
    }
}


#[launch]
fn rocket() -> _ {
    dotenv().ok(); // Loads .env file
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database pool.");

    let cors = CorsOptions::default()
        .to_cors()
        .expect("error creating CORS fairing");

    let figment = rocket::Config::figment().merge(("address", "0.0.0.0"));

    rocket::custom(figment)
        .manage(pool)
        .mount(
            "/api",
            routes![list_all_todos, add_todo, update_todo, delete_todo],
        )
        .attach(cors)
}

#[get("/todos")]
fn list_all_todos(pool: &State<DbPool>) -> Result<Json<Vec<ApiTodo>>, CustomError> {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    let db_todos = schema::todos::table.load::<DbTodo>(&mut conn)?;

    let db_reminders = DbReminder::belonging_to(&db_todos)
        .load::<DbReminder>(&mut conn)?
        .grouped_by(&db_todos);

    let combined_data: Vec<(DbTodo, Vec<DbReminder>)> =
        db_todos.into_iter().zip(db_reminders).collect();

    let api_todos: Vec<ApiTodo> = combined_data
        .into_iter()
        .map(|(todo, reminders)| {
            let reminder_dates = reminders
                .into_iter()
                .map(|r| DateTime::<Utc>::from_naive_utc_and_offset(r.reminder, Utc))
                .collect();

            ApiTodo {
                id: todo.id,
                title: todo.title,
                description: todo.description,
                completed: todo.completed,
                due: todo
                    .due
                    .map(|d| DateTime::<Utc>::from_naive_utc_and_offset(d, Utc)),
                repeat: RepeatRule::from(todo.repeat),
                reminder: reminder_dates,
            }
        })
        .collect();
    Ok(Json(api_todos))
}

#[post("/todos", data = "<new_todo_json>")]
fn add_todo(
    pool: &State<DbPool>,
    new_todo_json: Json<NewTodo>,
) -> Result<Json<ApiTodo>, CustomError> {
    let new_todo = new_todo_json.into_inner();
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    conn.transaction(|conn| {
        let last_id: Option<i32> = schema::todos::table
            .select(schema::todos::id)
            .order(schema::todos::id.desc())
            .first(conn)
            .optional()?;
        let new_id = last_id.map_or(1, |id| id + 1);

        let db_todo = DbTodo {
            id: new_id,
            title: new_todo.title.clone(),
            description: new_todo.description.clone(),
            due: new_todo.due.map(|dt| dt.naive_utc()),
            repeat: new_todo.repeat.to_string(),
            completed: false,
        };
        diesel::insert_into(schema::todos::table).values(&db_todo).execute(conn)?;

        let db_reminders: Vec<DbInsertableReminder> = new_todo.reminder
            .iter()
            .map(|rem| DbInsertableReminder {
                todo_id: new_id,
                reminder: rem.naive_utc(),
            })
            .collect();
        if !db_reminders.is_empty() {
            diesel::insert_into(schema::reminders::table).values(&db_reminders).execute(conn)?;
        }

        Ok(Json(ApiTodo {
            id: new_id,
            title: new_todo.title,
            description: new_todo.description,
            due: new_todo.due,
            reminder: new_todo.reminder,
            repeat: new_todo.repeat,
            completed: false,
        }))
    })
}

#[put("/todos/<id>", data = "<updated_todo_json>")]
fn update_todo(
    pool: &State<DbPool>,
    id: i32,
    updated_todo_json: Json<ApiTodo>,
) -> Result<Json<ApiTodo>, CustomError> {
    let updated_todo = updated_todo_json.into_inner();
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    conn.transaction(|conn| {
        let target = schema::todos::table.filter(schema::todos::id.eq(id));
        diesel::update(target)
            .set((
                schema::todos::title.eq(&updated_todo.title),
                schema::todos::description.eq(&updated_todo.description),
                schema::todos::due.eq(updated_todo.due.map(|dt| dt.naive_utc())),
                schema::todos::repeat.eq(updated_todo.repeat.to_string()),
                schema::todos::completed.eq(updated_todo.completed),
            ))
            .execute(conn)?;

        diesel::delete(schema::reminders::table.filter(schema::reminders::todo_id.eq(id))).execute(conn)?;

        let db_reminders: Vec<DbInsertableReminder> = updated_todo.reminder
            .iter()
            .map(|rem| DbInsertableReminder {
                todo_id: id,
                reminder: rem.naive_utc(),
            })
            .collect();
        if !db_reminders.is_empty() {
            diesel::insert_into(schema::reminders::table).values(&db_reminders).execute(conn)?;
        }
        
        Ok(Json(updated_todo))
    })
}

#[delete("/todos/<id>")]
fn delete_todo(pool: &State<DbPool>, id: i32) -> Result<Status, CustomError> {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    conn.transaction(|conn| {
        diesel::delete(schema::reminders::table.filter(schema::reminders::todo_id.eq(id))).execute(conn)?;

        let rows_deleted = diesel::delete(schema::todos::table.filter(schema::todos::id.eq(id))).execute(conn)?;

        if rows_deleted == 0 {
            // Return our specific "NotFound" error
            Err(CustomError::NotFound)
        } else {
            // Return a 204 No Content on successful deletion
            Ok(Status::NoContent)
        }
    })
}