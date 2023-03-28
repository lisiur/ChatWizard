-- Your SQL goes here
CREATE TABLE chat_logs (
  id BINARY PRIMARY KEY NOT NULL,
  chat_id BINARY NOT NULL,
  role TEXT NOT NULL,
  message TEXT NOT NULL,
  model TEXT NOT NULL,
  tokens INT NOT NULL,
  cost FLOAT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER auto_update_chat_logs_updated_at
  AFTER UPDATE ON chat_logs
  FOR EACH ROW
  BEGIN
    UPDATE chat_logs SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
  END;