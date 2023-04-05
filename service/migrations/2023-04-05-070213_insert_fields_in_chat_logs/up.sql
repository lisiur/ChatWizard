-- Your SQL goes here
ALTER TABLE chat_logs ADD COLUMN finished BOOLEAN NOT NULL DEFAULT true;

UPDATE chat_logs SET finished = true;