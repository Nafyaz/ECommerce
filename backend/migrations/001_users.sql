CREATE TABLE users
(
    id                UUID PRIMARY KEY                  DEFAULT gen_random_uuid(),
    name              VARCHAR(128)             NOT NULL,
    email             VARCHAR(254),
    email_verified_at TIMESTAMP WITH TIME ZONE,
    phone             VARCHAR(32),
    phone_verified_at TIMESTAMP WITH TIME ZONE,
    password_hash     VARCHAR(255),

    created_at        TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at        TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_user_email ON users (email);
CREATE INDEX idx_user_phone ON users (phone);
-- CREATE INDEX idx_user_active ON identity (deleted_at) WHERE deleted_at IS NULL;

CREATE TABLE user_roles
(
    id          SMALLINT PRIMARY KEY,
    name        VARCHAR(16)              NOT NULL UNIQUE,
    description VARCHAR(64)              NOT NULL UNIQUE,

    created_at  TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at  TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE TABLE users__user_roles
(
    user_id      UUID     NOT NULL REFERENCES users (id),
    user_role_id SMALLINT NOT NULL REFERENCES user_roles (id),
    PRIMARY KEY (user_id, user_role_id)
);

-- INSERT INTO user_roles (id, name, description)
-- VALUES (1, 'Super Admin', 'Administrator with full system access');