CREATE SCHEMA IF NOT EXISTS session;

CREATE TABLE session.session (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES "user"."user"(id) ON DELETE CASCADE,
    date DATE NOT NULL,
    model_id UUID REFERENCES model.model(id) ON DELETE SET NULL,
    note TEXT
);

CREATE INDEX idx_session_user_id ON session.session(user_id);
CREATE INDEX idx_session_user_id_date ON session.session(user_id, date DESC);

CREATE TABLE session.performed_variation (
    session_id UUID NOT NULL REFERENCES session.session(id) ON DELETE CASCADE,
    variation_id UUID NOT NULL REFERENCES maneuver.variation(id) ON DELETE CASCADE,
    quality SMALLINT NOT NULL,
    comfort SMALLINT NOT NULL,
    repeatability SMALLINT NOT NULL,
    note TEXT,
    PRIMARY KEY (session_id, variation_id)
);

CREATE INDEX idx_performed_variation_variation_id ON session.performed_variation(variation_id);
