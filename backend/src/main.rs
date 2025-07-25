#[macro_use]
extern crate rocket;

use chrono::{DateTime, Utc};
use jsonwebtoken::{DecodingKey, Header, Validation, decode};
use rocket::{
    Request, State,
    http::{ContentType, Status},
    request::{FromRequest, Outcome},
    response::{self, Responder, Response},
    serde::json::{Json, json},
};
use rocket_cors::CorsOptions;
use std::env;
use std::io::Cursor;

use diesel::BelongingToDsl;
use diesel::r2d2::{self, ConnectionManager, Pool};
use diesel::{prelude::*, result::Error as DieselError};
use dotenvy::dotenv;

mod api;
mod models;
mod schema;
mod security;

// --- Model and Type Definitions ---
type DbInsertableReminder = models::InsertableReminder;
type DbReminder = models::Reminder;
type DbTodo = models::Todo;
type DbInsertableTodo = models::InsertableTodo;
type DbInsertableUser = models::InsertableUser;
type DbUser = models::User;
type DbPool = Pool<ConnectionManager<SqliteConnection>>;
type ApiUser = api::User;
type NewUser = api::NewUser;
type ApiTodo = api::Todo;
type NewTodo = api::NewTodo;
type ApiSessionToken = api::SessionToken;
type ApiClaims = api::Claims;
type AuthenticatedUser = api::AuthenticatedUser;
type RepeatRule = api::RepeatRule;

// --- Custom Error Handling ---

// Define a custom error type for our application
#[derive(Debug)]
pub enum CustomError {
    DatabaseError(DieselError),
    NotFound,
    UsernameTaken,
    PasswordHashError(argon2::password_hash::Error),
    JwtError(jsonwebtoken::errors::Error),
    MissingConfig,
    MissingAuthToken,
}

pub struct AppConfig {
    pub jwt_secret: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = CustomError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Step 1: Get the AppConfig state for the secret key.
        // If it's not present, it's a server configuration error.
        let app_config = match req.rocket().state::<AppConfig>() {
            Some(config) => config,
            None => {
                // Log the server error for debugging
                eprintln!("Missing AppConfig in Rocket state!");
                return Outcome::Error((Status::InternalServerError, CustomError::MissingConfig));
            }
        };

        // Step 2: Extract and validate the token from the header.
        // We use `if let` to handle multiple failure points concisely.
        if let Some(auth_header) = req.headers().get_one("Authorization") {
            if let Some(token_str) = auth_header.strip_prefix("Bearer ") {
                // Step 3: Decode the token.
                let key = DecodingKey::from_secret(app_config.jwt_secret.as_ref());
                let validation = Validation::default();

                match decode::<ApiClaims>(token_str, &key, &validation) {
                    Ok(token_data) => {
                        // Success! Provide the AuthenticatedUser.
                        return Outcome::Success(AuthenticatedUser {
                            user_id: token_data.claims.sub,
                        });
                    }
                    Err(e) => {
                        // Token is present but invalid (expired, bad signature, etc.)
                        return Outcome::Error((Status::Unauthorized, CustomError::from(e)));
                    }
                }
            }
        }

        // If we reach here, it means the header was missing or not "Bearer".
        Outcome::Error((Status::Unauthorized, CustomError::MissingAuthToken))
    }
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
            CustomError::UsernameTaken => (
                Status::BadRequest,
                json!({"error": "The provided username is already taken."}),
            ),
            CustomError::PasswordHashError(e) => {
                eprintln!("An error occured handling password hashing: {:?}", e);
                (
                    Status::InternalServerError,
                    json!({"error": "An unexpected error occured handling authentication."}),
                )
            }
            CustomError::JwtError(e) => {
                eprintln!("An error occured handling JWT: {:?}", e);
                (
                    Status::InternalServerError,
                    json!({"error": "An unexpected error occured handling authentication."}),
                )
            }
            CustomError::MissingConfig => (
                Status::InternalServerError,
                json!({"error": "Server configuration error."}),
            ),
            CustomError::MissingAuthToken => (
                Status::Unauthorized,
                json!({"error": "Missing or invalid authorization token."}),
            ),
        };

        // Build the response
        Response::build()
            .status(status)
            .header(ContentType::JSON)
            .sized_body(
                error_json.to_string().len(),
                Cursor::new(error_json.to_string()),
            )
            .ok()
    }
}

impl From<DieselError> for CustomError {
    fn from(error: DieselError) -> Self {
        CustomError::DatabaseError(error)
    }
}

impl From<argon2::password_hash::Error> for CustomError {
    fn from(error: argon2::password_hash::Error) -> Self {
        CustomError::PasswordHashError(error)
    }
}

impl From<jsonwebtoken::errors::Error> for CustomError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        CustomError::JwtError(error)
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

    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let app_config = AppConfig { jwt_secret };

    rocket::custom(figment)
        .manage(pool)
        .manage(app_config)
        .mount(
            "/api",
            routes![
                list_all_todos,
                add_todo,
                update_todo,
                delete_todo,
                register_user,
                login,
            ],
        )
        .attach(cors)
}

#[get("/todos")]
fn list_all_todos(
    pool: &State<DbPool>,
    auth_user: AuthenticatedUser,
) -> Result<Json<Vec<ApiTodo>>, CustomError> {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    let db_todos = schema::todos::table
        .filter(schema::todos::user_id.eq(auth_user.user_id))
        .load::<DbTodo>(&mut conn)?;

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
    auth_user: AuthenticatedUser,
    new_todo_json: Json<NewTodo>,
) -> Result<Json<ApiTodo>, CustomError> {
    let new_todo = new_todo_json.into_inner();
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    conn.transaction(|conn| {
        let db_todo = DbInsertableTodo {
            user_id: auth_user.user_id,
            title: new_todo.title.clone(),
            description: new_todo.description.clone(),
            due: new_todo.due.map(|dt| dt.naive_utc()),
            repeat: new_todo.repeat.to_string(),
            completed: false,
        };

        let inserted_todo: DbTodo = diesel::insert_into(schema::todos::table)
            .values(&db_todo)
            .get_result(conn)?;

        let db_reminders: Vec<DbInsertableReminder> = new_todo
            .reminder
            .iter()
            .map(|rem| DbInsertableReminder {
                todo_id: inserted_todo.id,
                reminder: rem.naive_utc(),
            })
            .collect();
        if !db_reminders.is_empty() {
            diesel::insert_into(schema::reminders::table)
                .values(&db_reminders)
                .execute(conn)?;
        }

        Ok(Json(ApiTodo {
            id: inserted_todo.id,
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
    auth_user: AuthenticatedUser,
    id: i32,
    updated_todo_json: Json<ApiTodo>,
) -> Result<Json<ApiTodo>, CustomError> {
    let updated_todo = updated_todo_json.into_inner();
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    conn.transaction(|conn| {
        let target = schema::todos::table
            .filter(schema::todos::user_id.eq(auth_user.user_id))
            .filter(schema::todos::id.eq(id));
        diesel::update(target)
            .set((
                schema::todos::title.eq(&updated_todo.title),
                schema::todos::description.eq(&updated_todo.description),
                schema::todos::due.eq(updated_todo.due.map(|dt| dt.naive_utc())),
                schema::todos::repeat.eq(updated_todo.repeat.to_string()),
                schema::todos::completed.eq(updated_todo.completed),
            ))
            .execute(conn)?;

        diesel::delete(schema::reminders::table.filter(schema::reminders::todo_id.eq(id)))
            .execute(conn)?;

        let db_reminders: Vec<DbInsertableReminder> = updated_todo
            .reminder
            .iter()
            .map(|rem| DbInsertableReminder {
                todo_id: id,
                reminder: rem.naive_utc(),
            })
            .collect();
        if !db_reminders.is_empty() {
            diesel::insert_into(schema::reminders::table)
                .values(&db_reminders)
                .execute(conn)?;
        }

        Ok(Json(updated_todo))
    })
}

#[delete("/todos/<id>")]
fn delete_todo(
    pool: &State<DbPool>,
    auth_user: AuthenticatedUser,
    id: i32,
) -> Result<Status, CustomError> {
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    conn.transaction(|conn| {
        let num_deleted = diesel::delete(
            schema::todos::table
                .filter(schema::todos::user_id.eq(auth_user.user_id))
                .filter(schema::todos::id.eq(id)),
        )
        .execute(conn)?;

        if num_deleted == 0 {
            Err(CustomError::NotFound)
        } else {
            Ok(Status::NoContent)
        }
    })
}

#[post("/users/register", data = "<user_json>")]
fn register_user(
    pool: &State<DbPool>,
    user_json: Json<NewUser>,
) -> Result<Json<ApiUser>, CustomError> {
    let user = user_json.into_inner();
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    conn.transaction(|conn| {
        let username_taken = schema::users::table
            .filter(schema::users::username.eq(&user.username))
            .select(schema::users::id)
            .first::<i32>(conn)
            .optional()?
            .is_some();

        if username_taken {
            return Err(CustomError::UsernameTaken);
        }
        let db_user = DbInsertableUser {
            username: user.username,
            password: security::hash_password(&user.password)?,
        };
        let new_db_user = diesel::insert_into(schema::users::table)
            .values(&db_user)
            .get_result::<DbUser>(conn)?;

        Ok(Json(ApiUser {
            id: new_db_user.id,
            username: new_db_user.username,
        }))
    })
}

#[post("/users/login", data = "<user_json>")]
fn login(
    pool: &State<DbPool>,
    app_config: &State<AppConfig>,
    user_json: Json<NewUser>,
) -> Result<Json<ApiSessionToken>, CustomError> {
    let user = user_json.into_inner();
    let mut conn = pool.get().expect("Failed to get DB connection from pool");

    conn.transaction(|conn| {
        let db_user = schema::users::table
            .filter(schema::users::username.eq(&user.username))
            .first::<DbUser>(conn)?;

        if security::verify_password(&user.password, &db_user.password)? {
            Ok(Json(ApiSessionToken {
                token: jsonwebtoken::encode(
                    &Header::default(),
                    &ApiClaims {
                        sub: db_user.id,
                        exp: (Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
                    },
                    &jsonwebtoken::EncodingKey::from_secret(app_config.jwt_secret.as_ref()),
                )?,
            }))
        } else {
            Err(CustomError::NotFound)
        }
    })
}
