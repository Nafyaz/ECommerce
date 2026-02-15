CREATE TABLE products
(
    id          UUID PRIMARY KEY                  DEFAULT gen_random_uuid(),
    name        VARCHAR(128)             NOT NULL,
    slug        VARCHAR(128)             NOT NULL,
    description TEXT,
    vendor_id   UUID                     NOT NULL REFERENCES vendors (id),
    category_id UUID                     NOT NULL REFERENCES categories (id),
    price       INT                      NOT NULL,

    created_at  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by  UUID                     NOT NULL REFERENCES users (id),
    updated_at  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_by  UUID                     NOT NULL REFERENCES users (id),
    deleted_at  TIMESTAMP WITH TIME ZONE,

    UNIQUE (vendor_id, slug)
);

CREATE TABLE product_images
(
    id         UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id UUID          NOT NULL REFERENCES products (id),
    image_url  VARCHAR(2048) NOT NULL,

    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_product_name ON products (name);
CREATE INDEX idx_product_created_at ON products (created_at DESC);
-- CREATE INDEX idx_product_active ON products (deleted_at) WHERE deleted_at IS NULL;