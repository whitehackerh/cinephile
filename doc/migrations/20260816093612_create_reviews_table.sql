CREATE TABLE reviews (
  id UUID PRIMARY KEY,
  user_id UUID NOT NULL,
  rating SMALLINT NOT NULL,
  content TEXT,
  work_type VARCHAR NOT NULL,
  target_path VARCHAR NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  deleted_at TIMESTAMP NULL
);

CREATE UNIQUE INDEX idx_reviews_user_target_active 
ON reviews (user_id, target_path) 
WHERE deleted_at IS NULL;
