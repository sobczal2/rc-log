-- Revert: move difficulty back from variation to maneuver

-- Restore difficulty column on maneuver table (fill with minimum variation difficulty)
ALTER TABLE maneuver.maneuver
    ADD COLUMN difficulty INTEGER NOT NULL DEFAULT 1 CHECK (difficulty >= 1 AND difficulty <= 7);

UPDATE maneuver.maneuver m
SET difficulty = (
    SELECT MIN(v.difficulty)
    FROM maneuver.variation v
    WHERE v.maneuver_id = m.id
);

ALTER TABLE maneuver.maneuver ALTER COLUMN difficulty DROP DEFAULT;

-- Restore the old index
CREATE INDEX idx_maneuver_difficulty ON maneuver.maneuver (difficulty);

-- Drop variation-level difficulty index
DROP INDEX IF EXISTS maneuver.idx_variation_difficulty;

-- Drop difficulty column from variation table
ALTER TABLE maneuver.variation DROP COLUMN difficulty;
