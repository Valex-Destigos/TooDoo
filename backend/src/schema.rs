// @generated automatically by Diesel CLI.

diesel::table! {
    reminders (id) {
        id -> Integer,
        todo_id -> Integer,
        reminder -> Timestamp,
    }
}

diesel::table! {
    todos (id) {
        id -> Integer,
        user_id -> Integer,
        title -> Text,
        description -> Text,
        due -> Nullable<Timestamp>,
        repeat -> Text,
        completed -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
    }
}

diesel::joinable!(reminders -> todos (todo_id));
diesel::joinable!(todos -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    reminders,
    todos,
    users,
);
