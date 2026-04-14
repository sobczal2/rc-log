ALTER TABLE asset.video
    ADD COLUMN name VARCHAR(255);

UPDATE asset.video
SET name = id::text;

ALTER TABLE asset.video
    ALTER COLUMN name SET NOT NULL;

ALTER TABLE asset.video
    ADD CONSTRAINT asset_video_name_key UNIQUE (name);

CREATE INDEX idx_asset_video_name ON asset.video(name);

ALTER TABLE asset.photo
    ADD COLUMN name VARCHAR(255);

UPDATE asset.photo
SET name = id::text;

ALTER TABLE asset.photo
    ALTER COLUMN name SET NOT NULL;

ALTER TABLE asset.photo
    ADD CONSTRAINT asset_photo_name_key UNIQUE (name);

CREATE INDEX idx_asset_photo_name ON asset.photo(name);

ALTER TABLE maneuver.variation
    ADD COLUMN video_asset_name VARCHAR(255);

UPDATE maneuver.variation v
SET video_asset_name = av.name
FROM asset.video av
WHERE av.id = v.video_asset_id;

ALTER TABLE maneuver.variation
    ALTER COLUMN video_asset_name SET NOT NULL;

ALTER TABLE model.model
    ADD COLUMN photo_asset_name VARCHAR(255);

UPDATE model.model m
SET photo_asset_name = ap.name
FROM asset.photo ap
WHERE ap.id = m.photo_asset_id;

ALTER TABLE "user"."user"
    ADD COLUMN photo_asset_name VARCHAR(255);

UPDATE "user"."user" u
SET photo_asset_name = ap.name
FROM asset.photo ap
WHERE ap.id = u.photo_asset_id;

ALTER TABLE "user"."user"
    DROP CONSTRAINT fk_user_photo_asset_id;

ALTER TABLE model.model
    DROP CONSTRAINT fk_model_photo_asset_id;

ALTER TABLE maneuver.variation
    DROP CONSTRAINT fk_variation_video_asset_id;

ALTER TABLE "user"."user"
    DROP COLUMN photo_asset_id;

ALTER TABLE model.model
    DROP COLUMN photo_asset_id;

ALTER TABLE maneuver.variation
    DROP COLUMN video_asset_id;
