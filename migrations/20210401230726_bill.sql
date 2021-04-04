-- Add migration script here
CREATE TABLE bill (
  id TEXT PRIMARY KEY NOT NULL,
  account_id TEXT NOT NULL,
  favored_name TEXT NOT NULL,
  value REAL NOT NULL,
  created_date DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(account_id) REFERENCES account(id)
);