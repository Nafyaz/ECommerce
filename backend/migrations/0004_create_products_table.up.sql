-- CREATE TABLE product_tags
-- (
--     id          SMALLINT PRIMARY KEY,
--     name        VARCHAR(16)              NOT NULL UNIQUE,
--     description VARCHAR(128)             NOT NULL,
--
--     created_at  TIMESTAMP WITH TIME ZONE NOT NULL,
--     updated_at  TIMESTAMP WITH TIME ZONE NOT NULL
-- );

CREATE TYPE currency AS ENUM (
    'USD',
    'EUR',
    'GBP',
    'JPY',
    'BDT',
    'INR',
    'AUD',
    'CAD',
    'CHF',
    'CNY',
    'HKD',
    'SGD',
    'SEK',
    'NOK',
    'DKK',
    'NZD',
    'KRW',
    'TRY',
    'RUB',
    'BRL',
    'MXN',
    'AED',
    'SAR',
    'KWD'
    );

CREATE TABLE products
(
    id                 UUID PRIMARY KEY,
    name               VARCHAR(128)             NOT NULL,
    description        VARCHAR(2048),
    supplier_id        UUID                     NOT NULL REFERENCES vendors (id),

    price_amount_minor BIGINT                   NOT NULL,
    price_currency     currency                 NOT NULL,

--     tag_id         SMALLINT                 NOT NULL REFERENCES product_tags (id),

    created_at         TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at         TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE TYPE product_image_status AS ENUM (
    'PendingUpload',
    'Uploaded',
    'Processing',
    'Ready',
    'Failed',
    'Deleted'
    );

CREATE TABLE product_images
(
    id            UUID PRIMARY KEY,
    product_id    UUID                     NOT NULL REFERENCES products (id),
    object_key    VARCHAR(2048)            NOT NULL UNIQUE,
    content_type  VARCHAR(16)              NOT NULL,
    status        product_image_status     NOT NULL,
    file_size     BIGINT                   NOT NULL,
    display_order INTEGER                  NOT NULL,

    created_at    TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at    TIMESTAMP WITH TIME ZONE NOT NULL,

    CONSTRAINT chk_product_images_file_size_positive CHECK (file_size > 0),
    CONSTRAINT chk_product_images_position_nonnegative CHECK (display_order >= 0)
);

-- CREATE INDEX idx_product_name ON product (name);
-- CREATE INDEX idx_product_created_at ON product (created_at DESC);
-- CREATE INDEX idx_product_active ON product (deleted_at) WHERE deleted_at IS NULL;