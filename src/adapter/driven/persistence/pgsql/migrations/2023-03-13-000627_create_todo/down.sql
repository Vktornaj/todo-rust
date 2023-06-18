-- This file should undo anything in `up.sql`
DROP FUNCTION find_tags_sql(INT);
DROP FUNCTION find_user_by_username(text);
DROP FUNCTION find_todo_sql(varchar, varchar, varchar, INT, varchar[]);
DROP FUNCTION create_tag(character varying, character varying);
DROP TABLE _status;
DROP TABLE _todo_tag;
DROP TABLE _tag;
DROP TABLE _todo;
DROP TABLE _user;