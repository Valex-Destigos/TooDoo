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
        title -> Text,
        description -> Text,
        due -> Nullable<Timestamp>,
        repeat -> Text,
        completed -> Bool,
    }
}

diesel::joinable!(reminders -> todos (todo_id));

diesel::allow_tables_to_appear_in_same_query!(
    reminders,
    todos,
);
