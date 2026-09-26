CREATE TABLE collection_works (
    id UUID PRIMARY KEY,
    collection_id UUID NOT NULL REFERENCES collections(id) ON DELETE CASCADE,
    work_type VARCHAR NOT NULL,
    target_path VARCHAR NOT NULL,
    added_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT unique_collection_target UNIQUE (collection_id, target_path)
);

CREATE INDEX idx_collection_works_collection_id ON collection_works(collection_id);
