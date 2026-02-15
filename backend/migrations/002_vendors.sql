CREATE TABLE vendor_statuses
(
    id   SMALLINT PRIMARY KEY,
    name VARCHAR(64) NOT NULL UNIQUE
);

INSERT INTO vendor_statuses (id, name)
VALUES (1, 'Pending Approval'),
       (2, 'Active'),
       (3, 'Deactivated'),
       (4, 'Suspended');

CREATE TABLE vendors
(
    id                       UUID PRIMARY KEY                                                  DEFAULT gen_random_uuid(),
    name                     VARCHAR(128)             NOT NULL,
    slug                     VARCHAR(128)             NOT NULL UNIQUE,
    description              TEXT,
    email                    VARCHAR(254),
    email_verified_at        TIMESTAMP WITH TIME ZONE,
    phone_number             VARCHAR(32),
    phone_number_verified_at TIMESTAMP WITH TIME ZONE,
    status_id                SMALLINT                 NOT NULL REFERENCES vendor_statuses (id) DEFAULT 1,
    last_status_changed_at   TIMESTAMP WITH TIME ZONE,
    logo_url                 VARCHAR(2048),
    banner_url               VARCHAR(2048),

    created_at               TIMESTAMP WITH TIME ZONE NOT NULL                                 DEFAULT NOW(),
    created_by               UUID                     NOT NULL REFERENCES users (id),
    updated_at               TIMESTAMP WITH TIME ZONE NOT NULL                                 DEFAULT NOW(),
    updated_by               UUID REFERENCES users (id),
    deleted_at               TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_vendor_name ON vendors (name);
CREATE INDEX idx_vendor_status_id ON vendors (status_id);
CREATE INDEX idx_vendor_created_at ON vendors (created_at DESC);
CREATE INDEX idx_vendor_active ON vendors (status_id, deleted_at) WHERE deleted_at IS NULL;

-- CREATE TRIGGER update_vendors_updated_at
--     BEFORE UPDATE ON vendors
--     FOR EACH ROW
--     EXECUTE FUNCTION update_updated_at_column();