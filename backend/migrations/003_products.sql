CREATE TABLE product_tags
(
    id          SMALLINT PRIMARY KEY,
    name        VARCHAR(16)              NOT NULL UNIQUE,
    description VARCHAR(128)             NOT NULL,

    created_at  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by  UUID                     NOT NULL REFERENCES users (id),
    updated_at  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_by  UUID                     NOT NULL REFERENCES users (id),
    deleted_at  TIMESTAMP WITH TIME ZONE,
    deleted_by  UUID REFERENCES users (id)
);

CREATE TABLE products
(
    id             UUID PRIMARY KEY                  DEFAULT gen_random_uuid(),
    name           VARCHAR(128)             NOT NULL,
    slug           VARCHAR(128)             NOT NULL,
    description    TEXT,
    vendor_id      UUID                     NOT NULL REFERENCES vendors (id),

    tag_id         SMALLINT                 NOT NULL REFERENCES product_tags (id),

    price_amount   INT                      NOT NULL,
    price_currency VARCHAR(16)              NOT NULL,

    created_at     TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by     UUID                     NOT NULL REFERENCES users (id),
    updated_at     TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_by     UUID                     NOT NULL REFERENCES users (id),
    deleted_at     TIMESTAMP WITH TIME ZONE,
    deleted_by     UUID REFERENCES users (id),

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