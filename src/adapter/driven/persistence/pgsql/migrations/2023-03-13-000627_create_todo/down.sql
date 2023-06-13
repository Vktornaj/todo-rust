-- This file should undo anything in `up.sql`
DROP TABLE _status;
DROP TABLE _todo_tag;
DROP TABLE _tag;
DROP TABLE _todo;
DROP TABLE _user;
DROP FUNCTION find_user_by_username(text);