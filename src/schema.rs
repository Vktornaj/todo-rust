// @generated automatically by Diesel CLI.

diesel::table! {
    _status (id) {
        id -> Uuid,
        status_value -> Varchar,
    }
}

diesel::table! {
    _tag (id) {
        id -> Uuid,
        tag_value -> Varchar,
    }
}

diesel::table! {
    _todo (id) {
        id -> Uuid,
        user_id -> Uuid,
        title -> Varchar,
        description -> Nullable<Varchar>,
        status -> Int4,
        create_date -> Timestamp,
        done_date -> Nullable<Timestamp>,
        deadline -> Nullable<Timestamp>,
    }
}

diesel::table! {
    _todo_tag (todo_id, tag_id) {
        todo_id -> Uuid,
        tag_id -> Uuid,
    }
}

diesel::table! {
    _user (id) {
        id -> Uuid,
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
