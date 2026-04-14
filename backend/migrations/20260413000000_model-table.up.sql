CREATE SCHEMA model;

CREATE TABLE model.model (
    id UUID PRIMARY KEY,
    owner_id UUID NOT NULL REFERENCES "user"."user"(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    type VARCHAR(50) NOT NULL,
    photo_asset_name VARCHAR(255)
);

CREATE INDEX idx_model_owner_id ON model.model(owner_id);
