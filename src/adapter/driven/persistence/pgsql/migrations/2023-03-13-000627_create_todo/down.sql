-- This file should undo anything in `up.sql`
DROP FUNCTION find_user_by_username(text);
DROP FUNCTION create_tag_if_not_exists_in_user(character varying,character varying);
DROP TABLE _status;
DROP TABLE _todo_tag;
DROP TABLE _tag;
DROP TABLE _todo;
DROP TABLE _user;