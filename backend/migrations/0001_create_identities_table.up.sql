CREATE TYPE identity_status AS ENUM (
    'PENDING',
    'VERIFIED',
    'EXPIRED',
    'LOCKED'
    );

CREATE TABLE identities
(
    id            UUID PRIMARY KEY,
    email         VARCHAR(254)             NOT NULL UNIQUE,
    password_hash VARCHAR(255)             NOT NULL,
    status        identity_status          NOT NULL,
    created_at    TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at    TIMESTAMP WITH TIME ZONE NOT NULL
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

CREATE TYPE otp_purpose AS ENUM (
    'EMAIL_VERIFICATION',
    'PHONE_VERIFICATION',
    'LOGIN',
    'PASSWORD_RESET',
    'EMAIL_CHANGE',
    'PASSWORD_CHANGE',
    'DELETE_ACCOUNT'
    );

CREATE TYPE otp_status AS ENUM (
    'ACTIVE',
    'EXPIRED',
    'CONSUMED',
    'REVOKED'
    );

CREATE TABLE otps
(
    id          UUID PRIMARY KEY,
    identity_id UUID                     NOT NULL REFERENCES identities (id),
    purpose     otp_purpose              NOT NULL,
    code_hash   VARCHAR(255)             NOT NULL,
    status      otp_status               NOT NULL,
    attempts    SMALLINT                 NOT NULL,
    consumed_at TIMESTAMP WITH TIME ZONE,
    expires_at  TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at  TIMESTAMP WITH TIME ZONE NOT NULL,

    CHECK (expires_at > created_at),
    CHECK (
        (status = 'CONSUMED' AND consumed_at IS NOT NULL)
            OR
        (status <> 'CONSUMED' AND consumed_at IS NULL)
        )
);

CREATE UNIQUE INDEX uniq_active_otp
    ON otps (identity_id, purpose)
    WHERE status = 'ACTIVE';