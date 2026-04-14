ALTER TABLE maneuver.variation
    ADD COLUMN video_asset_id UUID;

UPDATE maneuver.variation v
SET video_asset_id = av.id
FROM asset.video av
WHERE av.name = v.video_asset_name;

ALTER TABLE maneuver.variation
    ALTER COLUMN video_asset_id SET NOT NULL;

ALTER TABLE maneuver.variation
    ADD CONSTRAINT fk_variation_video_asset_id
    FOREIGN KEY (video_asset_id)
    REFERENCES asset.video(id);

ALTER TABLE model.model
    ADD COLUMN photo_asset_id UUID;

UPDATE model.model m
SET photo_asset_id = ap.id
FROM asset.photo ap
WHERE ap.name = m.photo_asset_name;

ALTER TABLE model.model
    ADD CONSTRAINT fk_model_photo_asset_id
    FOREIGN KEY (photo_asset_id)
    REFERENCES asset.photo(id);

ALTER TABLE "user"."user"
    ADD COLUMN photo_asset_id UUID;

UPDATE "user"."user" u
SET photo_asset_id = ap.id
FROM asset.photo ap
WHERE ap.name = u.photo_asset_name;

ALTER TABLE "user"."user"
    ADD CONSTRAINT fk_user_photo_asset_id
    FOREIGN KEY (photo_asset_id)
    REFERENCES asset.photo(id);

ALTER TABLE maneuver.variation
    DROP COLUMN video_asset_name;

ALTER TABLE model.model
    DROP COLUMN photo_asset_name;

ALTER TABLE "user"."user"
    DROP COLUMN photo_asset_name;

DROP INDEX asset.idx_asset_video_name;
DROP INDEX asset.idx_asset_photo_name;

ALTER TABLE asset.video
    DROP COLUMN name;

ALTER TABLE asset.photo
    DROP COLUMN name;
