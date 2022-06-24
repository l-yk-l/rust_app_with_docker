CREATE TABLE IF NOT EXISTS log
(
  log_id SERIAL PRIMARY KEY,
  log_text TEXT,
  created_at TIMESTAMP DEFAULT now()
);