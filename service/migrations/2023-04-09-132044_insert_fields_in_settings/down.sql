-- This file should undo anything in `up.sql`
ALTER TABLE settings DROP COLUMN enable_web_server;
ALTER TABLE settings DROP COLUMN hide_main_window;
ALTER TABLE settings DROP COLUMN hide_taskbar;