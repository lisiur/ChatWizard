-- Your SQL goes here
ALTER TABLE chats ADD COLUMN sort INTEGER NOT NULL DEFAULT 0;
ALTER TABLE chats ADD COLUMN stick BOOLEAN NOT NULL DEFAULT false;

UPDATE chats SET sort = query.row_num
  FROM (
    SELECT ROW_NUMBER() OVER (ORDER BY created_at DESC) AS row_num, id 
    FROM chats
  ) AS query 
  WHERE query.id = chats.id;
UPDATE chats SET stick = false;