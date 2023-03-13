-- Your SQL goes here
CREATE TABLE _user (
    id uuid PRIMARY KEY,
    username VARCHAR NOT NULL,
    first_name VARCHAR,
    last_name VARCHAR,
    password VARCHAR NOT NULL
);
CREATE TABLE _status (
    id uuid PRIMARY KEY,
    status_value VARCHAR NOT NULL
);
CREATE TABLE _tag (
    id uuid PRIMARY KEY,
    tag_value VARCHAR NOT NULL
);
CREATE TABLE _todo (
    id uuid PRIMARY KEY,
    user_id uuid,
    title VARCHAR NOT NULL,
    description VARCHAR,
    status INT,
    create_date DATE NOT NULL,
    done_date DATE,
    deadline DATE,
    CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES _user(id)
);
CREATE TABLE _todo_tag (
    todo_id uuid NOT NULL,
    tag_id uuid NOT NULL,
    PRIMARY KEY (todo_id, tag_id),
    CONSTRAINT fk_todo FOREIGN KEY(todo_id) REFERENCES _todo(id),
    CONSTRAINT fk_tag FOREIGN KEY(tag_id) REFERENCES _tag(id)
);