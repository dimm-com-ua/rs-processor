ALTER TABLE pc_process_flow_element_argument ADD CONSTRAINT allowed_directions
    CHECK (direction = ANY ('{in, out, undefined}'::text[]));
