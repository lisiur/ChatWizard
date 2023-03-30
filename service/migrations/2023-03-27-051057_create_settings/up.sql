-- Your SQL goes here
CREATE TABLE settings (
  id BINARY PRIMARY KEY NOT NULL,
  user_id BINARY NOT NULL,
  language TEXT NOT NULL,
  theme TEXT NOT NULL,
  api_key TEXT,
  proxy TEXT,
  forward_url TEXT,
  forward_api_key BOOLEAN NOT NULL
);