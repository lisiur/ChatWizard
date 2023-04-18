-- Your SQL goes here
CREATE TABLE plugins (
  id BINARY PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  version TEXT NOT NULL,
  author TEXT NOT NULL,
  code BINARY NOT NULL,
  config TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER auto_update_plugins_updated_at
  AFTER UPDATE ON plugins
  FOR EACH ROW
  BEGIN
    UPDATE plugins SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
  END;