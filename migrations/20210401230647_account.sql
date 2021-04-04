-- Add migration script here
CREATE TABLE account (
  id TEXT PRIMARY KEY NOT NULL,
  agency INTEGER NOT NULL,
  balance REAL NOT NULL DEFAULT 0,
  created_date DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
