-- Create maneuver schema
CREATE SCHEMA IF NOT EXISTS maneuver;

-- Create tag table
CREATE TABLE maneuver.tag (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE
);

-- Create maneuver table
CREATE TABLE maneuver.maneuver (
    id UUID PRIMARY KEY,
    vehicle_type VARCHAR(50) NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    difficulty INTEGER NOT NULL CHECK (difficulty >= 1 AND difficulty <= 7),
    video_path TEXT
);

-- Create junction table for many-to-many relationship
CREATE TABLE maneuver.maneuver_tag (
    maneuver_id UUID NOT NULL REFERENCES maneuver.maneuver(id) ON DELETE CASCADE,
    tag_id UUID NOT NULL REFERENCES maneuver.tag(id) ON DELETE CASCADE,
    PRIMARY KEY (maneuver_id, tag_id)
);

-- Create indexes
CREATE INDEX idx_maneuver_vehicle_type ON maneuver.maneuver(vehicle_type);
CREATE INDEX idx_maneuver_difficulty ON maneuver.maneuver(difficulty);
