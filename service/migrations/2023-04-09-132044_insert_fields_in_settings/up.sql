-- Your SQL goes here
ALTER TABLE settings ADD COLUMN enable_web_server BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE settings ADD COLUMN hide_main_window BOOLEAN NOT NULL DEFAULT false;
ALTER TABLE settings ADD COLUMN hide_taskbar BOOLEAN NOT NULL DEFAULT false;

UPDATE settings SET enable_web_server = false;
UPDATE settings SET hide_main_window = false;
UPDATE settings SET hide_taskbar = false;
