CREATE TABLE identities
(
    id                UUID PRIMARY KEY,
    email             VARCHAR(254)             NOT NULL UNIQUE,
    email_verified_at TIMESTAMP WITH TIME ZONE,
    password_hash     VARCHAR(255)             NOT NULL,

    created_at        TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at        TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE TABLE roles
(
    id          SMALLINT PRIMARY KEY,
    name        VARCHAR(16)              NOT NULL UNIQUE,
    description VARCHAR(64)              NOT NULL UNIQUE,

    created_at  TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at  TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE TABLE permissions
(
    id         SMALLINT PRIMARY KEY,
    name       VARCHAR(16)              NOT NULL UNIQUE,

    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE TABLE roles__permissions
(
    role_id       SMALLINT NOT NULL REFERENCES roles (id),
    permission_id SMALLINT NOT NULL REFERENCES permissions (id),
    PRIMARY KEY (role_id, permission_id)
);

CREATE TABLE identities__roles
(
    identity_id UUID     NOT NULL REFERENCES identities (id),
    role_id     SMALLINT NOT NULL REFERENCES roles (id),
    PRIMARY KEY (identity_id, role_id)
);

CREATE TABLE revoked_tokens
(
    id         UUID PRIMARY KEY,
    token      VARCHAR(255)             NOT NULL,
    revoked_at TIMESTAMP WITH TIME ZONE NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL
);

-- Long living refresh tokens??
-- CREATE TABLE refresh_tokens
-- (
--     id          UUID PRIMARY KEY,
--     identity_id UUID                     NOT NULL REFERENCES identities (id),
--     token       VARCHAR(255)             NOT NULL,
--     expires_at  TIMESTAMP WITH TIME ZONE NOT NULL,
--     created_at  TIMESTAMP WITH TIME ZONE NOT NULL
-- );