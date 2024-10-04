-- Add up migration script here
CREATE OR REPLACE FUNCTION set_updated_at() RETURNS trigger AS '
    BEGIN 
        new.updated_at := ''now'';
        return new;
    END;
' LANGUAGE 'plpgsql';

CREATE TABLE IF NOT EXISTS books (
    book_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    isbn TEXT NOT NULL,
    description TEXT NOT NULL,
    created_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3),   
    updated_at TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
);

CREATE TRIGGER books_updated_at_trigger
    BEFORE UPDATE ON books FOR EACH ROW
    EXECUTE PROCEDURE set_updated_at();
