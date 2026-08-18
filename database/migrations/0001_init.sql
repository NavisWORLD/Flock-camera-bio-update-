BEGIN;

CREATE TABLE camera_events (
    event_id UUID PRIMARY KEY,
    camera_id TEXT NOT NULL,
    observed_at_ns NUMERIC(39, 0) NOT NULL,
    zone_id TEXT,
    event_kind TEXT NOT NULL,
    attributes JSONB NOT NULL DEFAULT '{}'::jsonb,
    face_occlusion TEXT NOT NULL DEFAULT 'unknown'
        CHECK (face_occlusion IN ('present', 'absent', 'unknown')),
    source_uri TEXT,
    received_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE sensor_observations (
    observation_id UUID PRIMARY KEY,
    sensor_id TEXT NOT NULL,
    source_kind TEXT NOT NULL,
    start_ns NUMERIC(39, 0) NOT NULL,
    end_ns NUMERIC(39, 0) NOT NULL,
    source_digest BYTEA NOT NULL CHECK (octet_length(source_digest) = 32),
    quality REAL NOT NULL CHECK (quality >= 0 AND quality <= 1),
    uncertainty REAL NOT NULL CHECK (uncertainty >= 0 AND uncertainty <= 1),
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE signal_templates (
    template_id UUID PRIMARY KEY,
    observation_id UUID NOT NULL REFERENCES sensor_observations(observation_id) ON DELETE CASCADE,
    feature_schema TEXT NOT NULL,
    feature_version TEXT NOT NULL,
    features JSONB NOT NULL,
    quality REAL NOT NULL CHECK (quality >= 0 AND quality <= 1),
    uncertainty REAL NOT NULL CHECK (uncertainty >= 0 AND uncertainty <= 1),
    source_digest BYTEA NOT NULL CHECK (octet_length(source_digest) = 32),
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE safety_events (
    safety_event_id UUID PRIMARY KEY,
    camera_event_id UUID REFERENCES camera_events(event_id) ON DELETE SET NULL,
    observation_id UUID REFERENCES sensor_observations(observation_id) ON DELETE SET NULL,
    rule_id TEXT NOT NULL,
    severity TEXT NOT NULL CHECK (severity IN ('informational', 'elevated')),
    rationale JSONB NOT NULL DEFAULT '{}'::jsonb,
    human_review_required BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE ledger_records (
    record_id UUID PRIMARY KEY,
    event_id UUID NOT NULL,
    observed_at_ns NUMERIC(39, 0) NOT NULL,
    sensor_ids JSONB NOT NULL,
    template_ids JSONB NOT NULL,
    payload_digest BYTEA NOT NULL CHECK (octet_length(payload_digest) = 32),
    previous_record_digest BYTEA NOT NULL CHECK (octet_length(previous_record_digest) = 32),
    record_digest BYTEA NOT NULL UNIQUE CHECK (octet_length(record_digest) = 32),
    signer_key_id TEXT NOT NULL,
    signature BYTEA NOT NULL,
    policy_context TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE audit_events (
    audit_id BIGSERIAL PRIMARY KEY,
    request_id UUID,
    actor_id TEXT NOT NULL,
    action TEXT NOT NULL,
    target_type TEXT NOT NULL,
    target_id TEXT,
    decision TEXT NOT NULL CHECK (decision IN ('allowed', 'denied', 'observed')),
    reason TEXT NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE policy_decisions (
    decision_id UUID PRIMARY KEY,
    action TEXT NOT NULL,
    allowed BOOLEAN NOT NULL,
    reason TEXT NOT NULL,
    context JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE retention_jobs (
    job_id UUID PRIMARY KEY,
    target_type TEXT NOT NULL,
    cutoff_at TIMESTAMPTZ NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('pending', 'running', 'complete', 'failed')),
    affected_rows BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    completed_at TIMESTAMPTZ
);

CREATE INDEX camera_events_time_idx ON camera_events(observed_at_ns);
CREATE INDEX camera_events_zone_idx ON camera_events(zone_id, observed_at_ns);
CREATE INDEX sensor_observations_time_idx ON sensor_observations(start_ns, end_ns);
CREATE INDEX ledger_event_idx ON ledger_records(event_id, observed_at_ns);
CREATE INDEX audit_time_idx ON audit_events(occurred_at);

COMMENT ON TABLE signal_templates IS
'Anonymous derived signal features. This production schema intentionally contains no person identity foreign key.';

COMMIT;
