CREATE TABLE users
(
    id                UUID PRIMARY KEY,
    identity_id       UUID                     NOT NULL UNIQUE REFERENCES identities (id),
    name              VARCHAR(128)             NOT NULL,
    phone             VARCHAR(32) UNIQUE,
    phone_verified_at TIMESTAMP WITH TIME ZONE,

    created_at        TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at        TIMESTAMP WITH TIME ZONE NOT NULL
);
