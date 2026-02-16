CREATE TABLE roles
(
    id   SMALLINT PRIMARY KEY,
    name VARCHAR(64) NOT NULL UNIQUE
);

CREATE TABLE users
(
    id                       UUID PRIMARY KEY                  DEFAULT gen_random_uuid(),
    name                     VARCHAR(128)             NOT NULL,
    slug                     VARCHAR(128)             NOT NULL UNIQUE,
    email                    VARCHAR(254),
    email_verified_at        TIMESTAMP WITH TIME ZONE,
    phone_number             VARCHAR(32),
    phone_number_verified_at TIMESTAMP WITH TIME ZONE,
    password_hash            VARCHAR(255),
    last_status_changed_at   TIMESTAMP WITH TIME ZONE,

    created_at               TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_by               UUID                     NOT NULL REFERENCES users (id),
    updated_at               TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_by               UUID REFERENCES users (id),
    deleted_at               TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_user_name ON users (email);
CREATE INDEX idx_user_created_at ON users (created_at DESC);
CREATE INDEX idx_user_active ON users (deleted_at) WHERE deleted_at IS NULL;

CREATE TABLE users_roles
(
    user_id UUID     NOT NULL REFERENCES users (id),
    role_id SMALLINT NOT NULL REFERENCES roles (id),
    PRIMARY KEY (user_id, role_id)
);

