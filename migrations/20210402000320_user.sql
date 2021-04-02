-- Add migration script here
CREATE TABLE user (
  id TEXT PRIMARY KEY NOT NULL,
  account_id TEXT NOT NULL,
  cpf TEXT NOT NULL,
  password TEXT NOT NULL,
  name TEXT NOT NULL,
  address TEXT NOT NULL,
  phone TEXT NOT NULL,
  birthdate TIMESTAMP NOT NULL,
  last_login TIMESTAMP,
  FOREIGN KEY(account_id) REFERENCES account(id)
);