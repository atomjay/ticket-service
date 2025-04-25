INSERT INTO users (email, password_hash, is_admin)
VALUES (
  'admin@example.com',
  '$argon2id$v=19$m=4096,t=3,p=1$...$...',
  true
);
