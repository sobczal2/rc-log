DROP INDEX IF EXISTS idx_performed_variation_session_id_id;
DROP INDEX IF EXISTS idx_performed_variation_session_id;

ALTER TABLE session.performed_variation
DROP CONSTRAINT performed_variation_pkey;

ALTER TABLE session.performed_variation
ADD CONSTRAINT performed_variation_pkey PRIMARY KEY (session_id, variation_id);

ALTER TABLE session.performed_variation
DROP COLUMN IF EXISTS id;