-- This file should undo anything in `up.sql`
ALTER TABLE chats DROP COLUMN archive;
ALTER TABLE chats DROP COLUMN archived_at;