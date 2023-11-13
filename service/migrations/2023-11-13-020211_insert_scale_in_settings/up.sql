-- Your SQL goes here
ALTER TABLE settings ADD COLUMN scale INT NOT NULL DEFAULT 14;

UPDATE settings SET scale = 14;
