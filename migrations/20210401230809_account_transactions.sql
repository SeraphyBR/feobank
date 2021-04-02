-- Add migration script here
CREATE TABLE account_transaction (
  account_id TEXT NOT NULL,
  transaction_id TEXT NOT NULL,
  FOREIGN KEY(account_id) REFERENCES account(id),
  FOREIGN KEY(transaction_id) REFERENCES transactions(id)
);