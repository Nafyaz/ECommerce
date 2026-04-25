CREATE TABLE vendor_statuses
(
    id          SMALLINT PRIMARY KEY,
    name        VARCHAR(16)              NOT NULL UNIQUE,
    description VARCHAR(64)              NOT NULL UNIQUE,

    created_at  TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at  TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE TABLE vendors
(
    id         UUID PRIMARY KEY,
    name       VARCHAR(128)             NOT NULL,
    owner_id   UUID                     NOT NULL REFERENCES users (id),
--     status_id  SMALLINT                 NOT NULL REFERENCES vendor_statuses (id),

    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE INDEX idx_vendor_owner ON vendors (owner_id);
-- CREATE INDEX idx_vendor_status_id ON vendors (status_id);
-- CREATE INDEX idx_vendor_created_at ON vendors (created_at DESC);
-- CREATE INDEX idx_vendor_active ON vendors (status_id, deleted_at) WHERE deleted_at IS NULL;

-- CREATE TRIGGER update_vendors_updated_at
--     BEFORE UPDATE ON vendors
--     FOR EACH ROW
--     EXECUTE FUNCTION update_updated_at_column();

-- CREATE TABLE vendors_users_roles
-- (
--     id          SMALLINT PRIMARY KEY,
--     name        VARCHAR(16)              NOT NULL UNIQUE,
--     description VARCHAR(64)              NOT NULL UNIQUE,
--
--     created_at  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
--     updated_at  TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
-- );

-- CREATE TABLE vendors__users
-- (
--     vendor_id UUID     NOT NULL REFERENCES vendors (id),
--     user_id   UUID     NOT NULL REFERENCES identity (id),
--     role_id   SMALLINT NOT NULL REFERENCES vendors_users_roles (id),
--     PRIMARY KEY (vendor_id, user_id)
-- )