-- Your SQL goes here
ALTER TABLE chats ADD COLUMN archive BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE chats ADD COLUMN archived_at TIMESTAMP;

UPDATE chats SET archive = false;