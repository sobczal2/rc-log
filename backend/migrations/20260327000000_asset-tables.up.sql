-- Create asset schema
CREATE SCHEMA IF NOT EXISTS asset;

-- Create video table
CREATE TABLE asset.video (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    small_path TEXT NOT NULL,
    medium_path TEXT,
    large_path TEXT
);

-- Create photo table
CREATE TABLE asset.photo (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    small_path TEXT NOT NULL,
    medium_path TEXT,
    large_path TEXT
);

-- Create indexes
CREATE INDEX idx_asset_video_name ON asset.video(name);
CREATE INDEX idx_asset_photo_name ON asset.photo(name);
