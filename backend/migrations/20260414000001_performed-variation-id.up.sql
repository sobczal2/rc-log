ALTER TABLE session.performed_variation
ADD COLUMN id UUID;

UPDATE session.performed_variation
SET id = md5(session_id::text || ':' || variation_id::text)::uuid
WHERE id IS NULL;

ALTER TABLE session.performed_variation
ALTER COLUMN id SET NOT NULL;

ALTER TABLE session.performed_variation
DROP CONSTRAINT performed_variation_pkey;

ALTER TABLE session.performed_variation
ADD CONSTRAINT performed_variation_pkey PRIMARY KEY (id);

CREATE INDEX idx_performed_variation_session_id ON session.performed_variation(session_id);
CREATE INDEX idx_performed_variation_session_id_id ON session.performed_variation(session_id, id);