-- Restore video_path column on maneuver table
ALTER TABLE maneuver.maneuver ADD COLUMN IF NOT EXISTS video_path TEXT;

-- Drop variation indexes
DROP INDEX IF EXISTS maneuver.idx_variation_maneuver_default;
DROP INDEX IF EXISTS maneuver.idx_variation_maneuver_id;

-- Drop variation table
DROP TABLE IF EXISTS maneuver.variation;
