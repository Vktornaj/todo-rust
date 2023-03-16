-- Your SQL goes here
CREATE TABLE _user (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    first_name VARCHAR,
    last_name VARCHAR,
    password VARCHAR NOT NULL
);

CREATE TABLE _status (
    id SERIAL PRIMARY KEY,
    status_value VARCHAR NOT NULL
);

CREATE TABLE _tag (
    id SERIAL PRIMARY KEY,
    tag_value VARCHAR NOT NULL
);

CREATE TABLE _todo (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    title VARCHAR NOT NULL,
    description VARCHAR,
    status INT NOT NULL,
    create_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    done_date TIMESTAMP WITH TIME ZONE,
    deadline TIMESTAMP WITH TIME ZONE,
    CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES _user(id)
);

CREATE TABLE _todo_tag (
    todo_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    PRIMARY KEY (todo_id, tag_id),
    CONSTRAINT fk_todo FOREIGN KEY(todo_id) REFERENCES _todo(id),
    CONSTRAINT fk_tag FOREIGN KEY(tag_id) REFERENCES _tag(id)
);
