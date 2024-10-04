DROP TRIGGER IF EXISTS books_updated_at_trigger;
DROP TRIGGER IF EXECUTE users_updated_at_trigger;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS roles;
DROP TABLE IF EXISTS books;

DROP FUNCTION set_updated_at;