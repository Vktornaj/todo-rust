// @generated automatically by Diesel CLI.

diesel::table! {
    _status (id) {
        id -> Int4,
        status_value -> Varchar,
    }
}

diesel::table! {
    _tag (id) {
        id -> Int4,
        tag_value -> Varchar,
    }
}

diesel::table! {
    _todo (id) {
        id -> Int4,
        user_id -> Int4,
        title -> Varchar,
        description -> Nullable<Varchar>,
        status -> Int4,
        create_date -> Timestamptz,
        done_date -> Nullable<Timestamptz>,
        deadline -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    _todo_tag (todo_id, tag_id) {
        todo_id -> Int4,
        tag_id -> Int4,
    }
}

diesel::table! {
    _user (id) {
        id -> Int4,
        username -> Varchar,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        password -> Varchar,
    }
}

diesel::joinable!(_todo -> _user (user_id));
diesel::joinable!(_todo_tag -> _tag (tag_id));
diesel::joinable!(_todo_tag -> _todo (todo_id));

diesel::allow_tables_to_appear_in_same_query!(
    _status,
    _tag,
    _todo,
    _todo_tag,
    _user,
);