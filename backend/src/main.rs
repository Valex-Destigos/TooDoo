#[macro_use]
extern crate rocket;

use rocket::{State, http::Method, serde::json::Json};
use rocket_cors::{AllowedOrigins, CorsOptions};
use std::sync::Mutex;

mod todo;

type Todo = todo::Todo;
type NewTodo = todo::NewTodo;
type TodoStore = Mutex<Vec<Todo>>;

#[launch]
fn rocket() -> _ {
    let allowed_origins = AllowedOrigins::all();

    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        // allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        // allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error creating CORS fairing");

    let db: TodoStore = Mutex::new(vec![]);

    let figment = rocket::Config::figment().merge(("address", "0.0.0.0"));

    rocket::custom(figment)
        .manage(db)
        .mount("/api", routes![add_todo, list_all_todos])
        .attach(cors)
}

#[get("/todos")]
fn list_all_todos(db: &State<TodoStore>) -> Json<Vec<Todo>> {
    Json(db.lock().unwrap().clone())
}

#[post("/todos", data = "<new_todo_json>")]
fn add_todo(db: &State<TodoStore>, new_todo_json: Json<NewTodo>) -> Json<Todo> {
    let new_todo = new_todo_json.into_inner();

    let mut todos = db.lock().unwrap();
    let new_id = todos.last().map(|todo| todo.id + 1).unwrap_or(1);

    let todo_to_add = Todo {
        id: new_id,
        title: new_todo.title,
        description: new_todo.description,
        due: new_todo.due,
        reminder: new_todo.reminder,
        repeat: new_todo.repeat,
        completed: false,
    };

    todos.push(todo_to_add.clone());

    Json(todo_to_add)
}
