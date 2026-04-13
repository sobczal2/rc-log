-- Move difficulty from maneuver to variation

-- Add difficulty column to variation table
ALTER TABLE maneuver.variation
    ADD COLUMN difficulty INTEGER NOT NULL DEFAULT 1 CHECK (difficulty >= 1 AND difficulty <= 7);

-- Remove default so new rows must explicitly provide a value
ALTER TABLE maneuver.variation ALTER COLUMN difficulty DROP DEFAULT;

-- Drop difficulty from maneuver table (now derived from variations)
ALTER TABLE maneuver.maneuver DROP COLUMN difficulty;

-- Drop the old maneuver-level difficulty index
DROP INDEX IF EXISTS maneuver.idx_maneuver_difficulty;

-- Add index on variation difficulty for filtering/sorting
CREATE INDEX idx_variation_difficulty ON maneuver.variation (difficulty);
