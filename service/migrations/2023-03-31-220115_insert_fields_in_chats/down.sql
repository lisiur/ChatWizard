-- This file should undo anything in `up.sql`
ALTER TABLE chats DROP COLUMN sort;
ALTER TABLE chats DROP COLUMN stick;