-- Your SQL goes here
CREATE TABLE prompt_sources (
  id BINARY PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  url TEXT NOT NULL,
  type TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER auto_update_prompt_sources_updated_at
  AFTER UPDATE ON prompt_sources
  FOR EACH ROW
  BEGIN
    UPDATE prompt_sources SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
  END;