CREATE SCHEMA IF NOT EXISTS training_program;

CREATE TABLE training_program.training_program (
    id UUID PRIMARY KEY,
    author_id UUID NOT NULL REFERENCES "user"."user"(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL
);

CREATE INDEX idx_training_program_author_id ON training_program.training_program(author_id);
CREATE INDEX idx_training_program_name ON training_program.training_program(name);

CREATE TABLE training_program.part (
    id UUID PRIMARY KEY,
    training_program_id UUID NOT NULL REFERENCES training_program.training_program(id) ON DELETE CASCADE,
    position INTEGER NOT NULL CHECK (position >= 0),
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    CONSTRAINT uq_training_program_part_position UNIQUE (training_program_id, position)
);

CREATE INDEX idx_training_program_part_program_id ON training_program.part(training_program_id);

CREATE TABLE training_program.part_variation (
    part_id UUID NOT NULL REFERENCES training_program.part(id) ON DELETE CASCADE,
    position INTEGER NOT NULL CHECK (position >= 0),
    variation_id UUID NOT NULL REFERENCES maneuver.variation(id) ON DELETE CASCADE,
    PRIMARY KEY (part_id, position)
);

CREATE INDEX idx_training_program_part_variation_part_id ON training_program.part_variation(part_id);
CREATE INDEX idx_training_program_part_variation_variation_id ON training_program.part_variation(variation_id);
