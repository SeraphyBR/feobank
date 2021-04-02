-- Add migration script here
CREATE TABLE transactions (
  id TEXT PRIMARY KEY NOT NULL,
  account_src TEXT NOT NULL,
  account_dist TEXT NOT NULL,
  value REAL NOT NULL,
  data TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(account_src) REFERENCES account(id),
  FOREIGN KEY(account_dist) REFERENCES account(id)
);