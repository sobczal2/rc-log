-- Create variation table
CREATE TABLE maneuver.variation (
    id UUID PRIMARY KEY,
    maneuver_id UUID NOT NULL REFERENCES maneuver.maneuver(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    video_asset_name VARCHAR(255) NOT NULL,
    is_default BOOLEAN NOT NULL DEFAULT FALSE
);

-- Ensure at most one default variation per maneuver
CREATE UNIQUE INDEX idx_variation_maneuver_default ON maneuver.variation (maneuver_id) WHERE is_default = TRUE;
CREATE INDEX idx_variation_maneuver_id ON maneuver.variation (maneuver_id);

-- Remove video_path from maneuver table (moved to variations)
ALTER TABLE maneuver.maneuver DROP COLUMN IF EXISTS video_path;
