-- Your SQL goes here
CREATE TABLE chat_models (
  id BINARY PRIMARY KEY NOT NULL,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  price FLOAT NOT NULL,
  unit TEXT NOT NULL,
  vendor TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER auto_update_chat_models_updated_at
  AFTER UPDATE ON chat_models
  FOR EACH ROW
  BEGIN
    UPDATE chat_models SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
  END;