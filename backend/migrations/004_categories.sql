CREATE TABLE categories
(
    id          UUID PRIMARY KEY                  DEFAULT gen_random_uuid(),
    name        VARCHAR(128)             NOT NULL,
    slug        VARCHAR(128)             NOT NULL UNIQUE,
    description TEXT,
    image_url   VARCHAR(2048),
    parent_id   UUID REFERENCES categories (id),

    created_at  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by  UUID REFERENCES users (id),
    updated_at  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_by  UUID REFERENCES users (id),
    deleted_at  TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_category_name ON categories (name);
CREATE INDEX idx_category_created_at ON categories (created_at DESC);
CREATE INDEX idx_category_active ON categories (deleted_at) WHERE deleted_at IS NULL;
