-- Drop indexes
DROP INDEX IF EXISTS asset.idx_asset_video_name;
DROP INDEX IF EXISTS asset.idx_asset_photo_name;

-- Drop tables
DROP TABLE IF EXISTS asset.video;
DROP TABLE IF EXISTS asset.photo;

-- Drop schema
DROP SCHEMA IF EXISTS asset;
